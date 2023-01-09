use std::io::BufRead;

fn main() {
    let stdin = std::io::stdin();

    let mut max: i32 = 0;
    let mut curr: i32 = 0;
    let mut update_max = |value: i32| {
        if max < value {
            max = value;
        }
    };
    for line in stdin.lock().lines() {
        if let Ok(s) = line {
            let svalue = s.trim();
            if svalue.len() > 0 {
                let value: i32 = svalue.parse().unwrap();
                curr += value;
            } else {
                update_max(curr);
                curr = 0;
            }
        }
        }
    update_max(curr);
    println!("{}", max);
}
