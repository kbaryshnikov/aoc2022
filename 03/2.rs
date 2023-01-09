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

fn first_intersection_in_sorted(a: &Vec<u8>, b: &Vec<u8>, c: &Vec<u8>) -> u8 {
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < a.len() && j < b.len() && k < c.len() {
        if a[i] == b[j] && a[i] == c[k] {
            return a[i]
        }
        if a[i] == b[j] {
            if a[i] < c[k] {
                i += 1
            } else {
                k += 1
            }
        } else if a[i] == c[k] {
            if a[i] < b[j] {
                i += 1
            } else {
                j += 1
            }
        } else if a[i] < b[j] && a[i] < c[k] && i < a.len() {
            i += 1
        } else if b[j] < a[i] && b[j] < c[k] && j < b.len() {
            j += 1
        } else {
            k += 1
        }
    }
    0
}

fn main() {
    let stdin = std::io::stdin();
    let mut sum = 0u32;

    let mut lines = stdin.lock().lines();
    while let (Some(l1), Some(l2), Some(l3)) = (lines.next(), lines.next(), lines.next()) {
        if let (Ok(a), Ok(b), Ok(c)) = (l1, l2, l3) {
            let codes_a = &to_char_codes_sorted(&a);
            let codes_b = &to_char_codes_sorted(&b);
            let codes_c = &to_char_codes_sorted(&c);
            let code = first_intersection_in_sorted(codes_a, codes_b, codes_c);
            // println!("{}", code);
            sum += code as u32;
        }
    }
    
    println!("{}", sum);
}
