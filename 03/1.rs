use std::io::BufRead;

fn to_char_codes_sorted(s: &str) -> Vec<u8> {
    let mut vec: Vec<u8> = s.bytes().map(|c| match c as u8 {
        97..=122 => c - 96,
        65..=90 => c - 38,
        _ => panic!("Invalid char code: {}", c)
    }).collect();
    vec.sort_unstable();
    vec
}

fn first_intersection_in_sorted(a: &Vec<u8>, b: &Vec<u8>) -> u8 {
    let mut i = 0;
    let mut j = 0;
    while i < a.len() && j < b.len() {
        if a[i] == b[j] {
            return a[i]
        }
        if a[i] > b[j] {
            j += 1
        } else {
            i += 1
        }
    }
    0
}

fn main() {
    let stdin = std::io::stdin();
    let mut sum = 0u32;

    for line in stdin.lock().lines() {
        if let Ok(s) = line {
            let count = s.len();
            if count % 2 != 0 {
                panic!("Count {} is odd", count);
            }
            let size = count / 2;
            let a = &to_char_codes_sorted(&s[0..size]);
            let b = &to_char_codes_sorted(&s[size..count]);
            let common = first_intersection_in_sorted(a, b);
            sum += common as u32;
        } else {
            println!("{}", "Failed to read line");
        }
    }
    
    println!("{}", sum);
}
