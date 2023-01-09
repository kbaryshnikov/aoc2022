use std::io::BufRead;
use std::env;

fn parse_crates_line(s: &str, count: usize) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    for i in 0..count {
        let offset = i * 4;
        let crt = &s[offset..offset+3].as_bytes();
        if crt[0] == 0x20 {
            result.push(0);
        } else {
            result.push(crt[1]);
        }
    }
    result
}

fn parse_crates(crates_lines: &mut Vec<String>) -> Vec<Vec<u8>> {
    let indicies = crates_lines.pop().unwrap();
    let count: usize = indicies.trim().split_whitespace().next_back().unwrap().parse().unwrap();
    let mut stacks: Vec<Vec<u8>> = Vec::new();
    for _ in 0..count {
        stacks.push(Vec::new());
    }
    for crates_line in crates_lines.into_iter().rev() {
        let line = parse_crates_line(&crates_line, count);
        for i in 0..count {
            if line[i] > 0 {
                stacks[i].push(line[i]);
            }
        }
    }
    stacks
}

fn parse_cmd_line(cmd_line: &str) -> (u8, usize, usize) {
    let mut tokens = cmd_line.split_whitespace();
    tokens.next();
    let mv: u8 = tokens.next().unwrap().parse().unwrap();
    tokens.next();
    let fm: usize = tokens.next().unwrap().parse().unwrap();
    tokens.next();
    let to: usize = tokens.next().unwrap().parse().unwrap();
    (mv, fm - 1, to - 1)
}

fn print_stacks(stacks: &Vec<Vec<u8>>) {
    for stack in stacks {
        for crt in stack {
            print!("{}", *crt as char);
        }
        println!("");
    }
    println!("-------------------");
}

fn print_top_crates(stacks: &Vec<Vec<u8>>) {
    for stack in stacks {
        print!("{}", *stack.last().unwrap() as char);
    }
    println!("");
}

fn mover_9000(stacks: &mut Vec<Vec<u8>>, mv: u8, fm: usize, to: usize) {
    for _ in 0..mv {
        let value = stacks[fm].pop().unwrap();
        stacks[to].push(value);
    }
}

fn mover_9001(stacks: &mut Vec<Vec<u8>>, mv: u8, fm: usize, to: usize) {
    let mut tmp: Vec<u8> = Vec::new();
    for _ in 0..mv {
        let value = stacks[fm].pop().unwrap();
        tmp.push(value);
    }
    for value in tmp.into_iter().rev() {
        stacks[to].push(value);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mover = match args.len() {
        2 => match args[1].as_str() {
            "9000" => mover_9000,
            "9001" => mover_9001,
            _ => {
                eprintln!("Invalid mover: {}", args[1]);
                std::process::exit(1);
            }
        },
        _ => {
            eprintln!("Usage: {0} 9000 or {0} 9001", args[0]);
            std::process::exit(1);
        },
    };
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines();
    let mut crates_lines = Vec::new();
    while let Ok(s) = lines.next().unwrap() {
        if s.len() == 0 {
            break;
        }
        crates_lines.push(s);
    }
    let mut stacks = parse_crates(&mut crates_lines);
    for cmd_line in lines {
        print_stacks(&stacks);
        let (mv, fm, to) = parse_cmd_line(&cmd_line.unwrap());
        mover(&mut stacks, mv, fm, to);
    }
    print_stacks(&stacks);
    print_top_crates(&stacks);
}
