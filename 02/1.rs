use std::io::BufRead;
use std::collections::HashMap;

fn calc_score(a: u8, b: u8) -> u8 {
    let mut winner_score = 0u8;
    if a == b {
        winner_score = 3;
    } else if a == 1 && b == 3 {
        winner_score = 0;
    } else if a == 3 && b == 1 {
        winner_score = 6;
    } else if a < b {
        winner_score = 6;
    }
    winner_score + b
}

fn main() {
    let stdin = std::io::stdin();

    let mut shapes1 = HashMap::new();
    shapes1.insert("A", 1u8);
    shapes1.insert("B", 2u8);
    shapes1.insert("C", 3u8);

    let mut shapes2 = HashMap::new();
    shapes2.insert("X", 1u8);
    shapes2.insert("Y", 2u8);
    shapes2.insert("Z", 3u8);

    let mut sum_score = 0u32;

    for line in stdin.lock().lines() {
        if let Ok(s) = line {
            if let Some((a, b)) = s.split_once(' ') {
                let shape1 = shapes1[a];
                let shape2 = shapes2[b];
                let score = calc_score(shape1, shape2);
                sum_score += score as u32;
            }
        } else {
            println!("{}", "Failed to read line");
        }
    }
    println!("{}", sum_score);

}
