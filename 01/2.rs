use std::io::BufRead;

fn main() {
    const SIZE: usize = 3;
    let stdin = std::io::stdin();

    let mut max: [i32; SIZE] = [0; SIZE];
    let mut curr: i32 = 0;
    let mut update_max = |value: i32| {
        let mut idx = SIZE - 1;
        loop {
            if max[idx] > value {
                break;
            }
            if idx < SIZE - 1 {
                max[idx + 1] = max[idx];
            }
            max[idx] = value;
            if idx == 0 {
                break;
            }
            idx = idx - 1;
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
    let mut sum: i32 = 0;
    for i in 0..SIZE {
        sum += max[i];
        println!("{}", max[i]);
    }
    println!("");
    println!("{}", sum);
}
