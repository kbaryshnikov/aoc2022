use std::io::BufRead;

fn parse_cmd(s: &str) -> (&str, Vec<&str>) {
    let mut tokens = s.split_whitespace();
    assert_eq!("$", tokens.next().unwrap());
    let cmd = tokens.next().unwrap();
    (cmd, tokens.collect())
}

fn parse_ls(s: &str) -> (u32, String) {
    let mut tokens = s.split_whitespace();
    let size_str = tokens.next().unwrap();
    let size: u32 = match size_str {
        "dir" => 0,
        _ => size_str.parse().unwrap()
    };
    (size, tokens.next().unwrap().to_string())
}

fn main() {
    const LIMIT: u32 = 100000;
    const FS_SIZE: u32 = 70000000;
    const REQUIRED_SIZE: u32 = 30000000;

    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();

    let mut path: Vec<u32> = Vec::new();
    let mut size: u32 = 0;
    let mut result: u32 = 0;
    let mut used_space: u32 = 0;
    let mut sizes: Vec<u32> = Vec::new();
    let mut reading_ls = false;
    let mut should_exit = false;

    while !should_exit {
        let rline = lines.next();
        let line = match rline {
            Some(Result::Ok(s)) => s,
            _ => "".to_string()
        };
        if reading_ls {
            if line.eq("") || line[0..1].eq("$") {
                reading_ls = false;
            } else {
                let (file_size, _) = parse_ls(&line);
                size += file_size;
                used_space += file_size;
            }
        }
        if !reading_ls {
            let (cmd, args) = if line.eq("") {
                should_exit = true;
                parse_cmd("$ cd ..")
            } else {
                parse_cmd(&line)
            };
            match cmd {
                "cd" => {
                    let dst = args[0];
                    match dst {
                        "/" => {
                            if path.len() > 0 {
                                panic!("Cannot cd / unless it's the first command");
                            }
                        },
                        ".." => {
                            if size <= LIMIT {
                                result += size;
                            }
                            sizes.push(size);
                            size = size + path.pop().unwrap();
                        },
                        _ => {
                            path.push(size);
                        }
                    }
                },
                "ls" => {
                    reading_ls = true;
                    size = 0;
                },
                _ => {
                    panic!("Unknown command: {}", cmd);
                }
            }
        }
    }
    println!("{}", result);

    let should_free = REQUIRED_SIZE - (FS_SIZE - used_space);
    result = 0;
    for size in sizes {
        if size >= should_free && (result == 0 || size < result) {
            result = size;
        }
    }
    println!("{}", result);
}
