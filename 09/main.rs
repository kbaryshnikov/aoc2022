use std::io::BufRead;
use std::env;
use std::cmp;
use std::collections::HashSet;

#[derive(Debug)]
struct Grid {
    tails_count: usize,
    visited: HashSet<(i32, i32)>,
    positions: Vec<(i32, i32)>
}

impl Grid {

    fn new(tails_count: usize) -> Grid {
        let mut grid = Grid {
            tails_count: tails_count,
            visited: HashSet::new(),
            positions: vec![(0, 0); tails_count + 1],
        };
        grid.mark_visited();
        grid
    }

    fn mark_visited(&mut self) {
        self.visited.insert(self.positions[self.tails_count]);
    }

    fn calc_distance(a: (i32, i32), b: (i32, i32)) -> u32 {
        let (ax, ay) = a;
        let (bx, by) = b;
        let dx: u32 = (ax - bx).abs() as u32;
        let dy: u32 = (ay - by).abs() as u32;
        cmp::max(dx, dy)
    }

    fn calc_move(to: (i32, i32), from: (i32, i32)) -> (i32, i32) {
        let (to_x, to_y) = to;
        let (from_x, from_y) = from;
        let mut dx: i32 = to_x - from_x;
        let mut dy: i32 = to_y - from_y;
        if dx.abs() <= 2 && dy.abs() <= 2 {
            dx = dx.clamp(-1, 1);
            dy = dy.clamp(-1, 1);
        } else if dx.abs() == 2 && dy == 0 {
            dx = dx.clamp(-1, 1);
        } else if dx == 0 && dy.abs() == 2 {
            dy = dy.clamp(-1, 1);
        }
        (dx, dy)
    }

    fn mv(&mut self, dx: i8, dy: i8) {
        let (orig_hx, orig_hy) = self.positions[0];
        let new_hx = orig_hx + dx as i32;
        let new_hy = orig_hy + dy as i32;
        self.positions[0] = (new_hx, new_hy);
        for i in 1..=self.tails_count {
            let distance = Self::calc_distance(self.positions[i - 1], self.positions[i]);
            if distance > 1 {
                let (dx, dy) = Self::calc_move(self.positions[i - 1], self.positions[i]);
                let (x, y) = self.positions[i];
                self.positions[i] = (x + dx, y + dy);
                if i == self.tails_count {
                    self.mark_visited();
                    //self.print(20, 20);
                }
            }
        }
    }

    fn mvn(&mut self, dx: i8, dy: i8, times: i32) {
        for _ in 0..times {
            self.mv(dx, dy);
        }
    }

    fn count_visited(&self) -> usize {
        self.visited.len()
    }

    fn print(&self, width: i32, height: i32) {
        for y in 0..height {
            for x in 0..width {
                let mut found = false;
                for i in 0..=self.tails_count {
                    if self.positions[i] == (x, y) {
                        print!("{}", i);
                        found = true;
                        break;
                    }
                }
                if !found {
                    if self.visited.contains(&(x, y)) {
                        print!("{}", "#");
                    } else {
                        print!("{}", ".");
                    }
                }
            }
            println!("");
        }
        println!("{:?}", self.positions);
    }

}

fn main() {
    let stdin = std::io::stdin();
    let args: Vec<String> = env::args().collect();
    let tails_count: usize = match args.len() {
        1 => 1usize,
        _ => match args[1].parse::<usize>() {
            Ok(value) => value,
            _ => {
                eprintln!("Invalid value: {}", args[1]);
                std::process::exit(1);
            }
        }
    };
    let mut grid = Grid::new(tails_count);

    for line in stdin.lock().lines() {
        if let Ok(s) = line {
            let mut tokens = s.split_whitespace();
            let (dx, dy, sn): (i8, i8, &str) = match (tokens.next().unwrap(), tokens.next().unwrap()) {
                ("R", n) => (1, 0, n),
                ("L", n) => (-1, 0, n),
                ("U", n) => (0, 1, n),
                ("D", n) => (0, -1, n),
                (_, _) => unreachable!(),
            };
            let n: i32 = sn.parse().unwrap();
            grid.mvn(dx, dy, n);
        }
    }

    grid.print(20, 20);
    println!("{} {}", grid.tails_count, grid.count_visited());

}
