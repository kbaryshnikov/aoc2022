use std::io::BufRead;
use std::collections::HashMap;

fn calc_score(a: u8, winner_score: u8) -> u8 {
    let b: u8;

    if winner_score == 3 {
        b = a
    } else if winner_score == 0 {
        b = if a == 1 {
            3
        } else {
            a - 1
        }
    } else if winner_score == 6 {
        b = if a == 3 {
            1
        } else {
            a + 1
        }
    } else {
        panic!("Invalid input: winner_score = {}", winner_score);
    }

    winner_score + b
}

fn main() {
    let stdin = std::io::stdin();

    let mut shapes = HashMap::new();
    shapes.insert("A", 1u8);
    shapes.insert("B", 2u8);
    shapes.insert("C", 3u8);

    let mut outcome = HashMap::new();
    outcome.insert("X", 0u8);
    outcome.insert("Y", 3u8);
    outcome.insert("Z", 6u8);

    let mut sum_score = 0u32;

    for line in stdin.lock().lines() {
        if let Ok(s) = line {
            if let Some((a, b)) = s.split_once(' ') {
                let shape = shapes[a];
                let outcome = outcome[b];
                let score = calc_score(shape, outcome);
                sum_score += score as u32;
            }
        } else {
            println!("{}", "Failed to read line");
        }
    }
    
    println!("{}", sum_score);

}
