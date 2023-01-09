use std::io::BufRead;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let packet_size = match args.len() {
        2 => match args[1].parse::<usize>() {
            Ok(value) => value,
            _ => {
                eprintln!("Invalid value: {}", args[1]);
                std::process::exit(1);
            }
        },
        _ => {
            eprintln!("Usage: {0} packet_size", args[0]);
            std::process::exit(1);
        },
    };
    let stdin = std::io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let bytes = input.as_bytes();
    let len = bytes.len();
    'outer: for i in 0..=len-packet_size {
        let mut chunk: Vec<char> = input[i..i+packet_size].chars().collect::<Vec<char>>().to_vec();
        chunk.sort_unstable();
        for j in 0..packet_size-1 {
            if chunk[j] == chunk[j + 1] {
                continue 'outer;
            }
        }
        println!("{}", i + packet_size);
        break;
    }
}
