use std::io::BufRead;

fn parse_range(s: &str) -> (u16, u16) {
    let Some((a, b)) = s.split_once('-') else { panic!("Invalid format: {}", s); };
    (a.parse().unwrap(), b.parse().unwrap())
}

#[allow(for_loops_over_fallibles)]
fn main() {
    let stdin = std::io::stdin();
    let mut n = 0;
    for line in stdin.lock().lines() {
        if let Ok(s) = line {
            for (a1, a2) in s.split_once(',') {
                let (s1, e1) = parse_range(a1);
                let (s2, e2) = parse_range(a2);
                if (s1 <= s2 && e2 <= e1) || (s2 <= s1 && e1 <= e2) {
                    n += 1;
                }
            }
        } else {
            println!("{}", "Failed to read line");
        }
    }

    println!("{}", n);
}
