// O(n*m) solutions seem to be possible, but bruteforce approach is fast enough
use std::io::BufRead;

#[derive(Debug)]
struct Grid {
    matrix: Vec<Vec<u8>>,
    width: usize,
    height: usize
}

#[derive(Clone, Copy)]
enum Direction {
    TOP,
    RIGHT,
    BOTTOM,
    LEFT
}

impl Grid {

    fn new(width: usize, height: usize) -> Grid {
        Grid {
            matrix: vec![vec![0u8; width]; height],
            width: width,
            height: height
        }
    }

    fn set(&mut self, y: usize, x: usize, value: u8) {
        assert_eq!(true, x < self.width);
        assert_eq!(true, y < self.height);
        self.matrix[y][x] = value;
    }

    fn is_visible(&mut self, mut y: isize, mut x: isize, direction: Direction) -> bool {
        let (dx, dy): (isize, isize) = match direction {
            Direction::TOP => (0, -1),
            Direction::RIGHT => (1, 0),
            Direction::BOTTOM => (0, 1),
            Direction::LEFT => (-1, 0)
        };
        let mut in_bounds = self.in_bounds(y, x);
        let tree_height = self.matrix[y as usize][x as usize];
        x += dx;
        y += dy;
        while in_bounds {
            if self.matrix[y as usize][x as usize] >= tree_height {
                return false;
            }
            x += dx;
            y += dy;
            in_bounds = self.in_bounds(y, x);
        }
        true
    }

    fn scan(&mut self) -> u16 {
        static DIRS: [Direction; 4] = [Direction::TOP, Direction::RIGHT, Direction::BOTTOM, Direction::LEFT];
        let mut cnt: u16 = (self.width as u16) * 2 + ((self.height as u16) - 2) * 2;
        for y in 1isize..(self.height as isize)-1 {
            for x in 1isize..(self.width as isize)-1 {
                let mut is_visible = false;
                for dir in DIRS.iter() {
                    if self.is_visible(y, x, *dir) {
                        is_visible = true;
                        break;
                    }
                }
                if is_visible {
                    cnt += 1;
                }
            }
        }
        cnt
    }

    fn count_visible(&mut self, mut y: isize, mut x: isize, direction: Direction) -> u16 {
        let (dx, dy): (isize, isize) = match direction {
            Direction::TOP => (0, -1),
            Direction::RIGHT => (1, 0),
            Direction::BOTTOM => (0, 1),
            Direction::LEFT => (-1, 0)
        };
        let mut visible_count = 0u16;
        let mut in_bounds = self.in_bounds(y, x);
        let tree_height = self.matrix[y as usize][x as usize];
        x += dx;
        y += dy;
        while in_bounds {
            visible_count += 1;
            if self.matrix[y as usize][x as usize] >= tree_height {
                break;
            }
            x += dx;
            y += dy;
            in_bounds = self.in_bounds(y, x);
        }
        visible_count
    }

    fn scan2(&mut self) -> u32 {
        static DIRS: [Direction; 4] = [Direction::TOP, Direction::RIGHT, Direction::BOTTOM, Direction::LEFT];
        let mut max_score = 0u32;
        for y in 1isize..(self.height as isize)-1 {
            for x in 1isize..(self.width as isize)-1 {
                let mut score = 1u32;
                for dir in DIRS.iter() {
                    let cnt = self.count_visible(y, x, *dir);
                    score = score * cnt as u32;
                }
                if score > max_score {
                    max_score = score;
                }
            }
        }
        max_score
    }

    fn in_bounds(&self, y: isize, x: isize) -> bool {
        x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height
    }

}

fn load_grid() -> Grid {
    let stdin = std::io::stdin();

    let mut matrix: Vec<Vec<u8>> = Vec::new();

    for line in stdin.lock().lines() {
        if let Ok(s) = line {
            let mut row: Vec<u8> = Vec::new();
            for val in s.chars() {
                row.push(val.to_string().parse().unwrap());
            }
            matrix.push(row);
        }
    }

    let mut grid = Grid::new(matrix[0].len(), matrix.len());
    for (y, row) in matrix.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            grid.set(y, x, *val);
        }
    }

    grid
}

fn main() {
    let mut grid = load_grid();
    let result = grid.scan();
    println!("{}", result);
    let result2 = grid.scan2();
    println!("{}", result2);
}
