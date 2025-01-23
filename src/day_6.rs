#[derive(Clone)]
struct Maze {
    position: (isize, isize),
    direction: u8,
    grid: Vec<Vec<u8>>,
}

impl Maze {
    fn new(grid_str: &[&str]) -> Self {
        let mut grid: Vec<Vec<u8>> = vec![];
        let mut position: (isize, isize) = (0, 0);
        for (i, line_str) in grid_str.iter().enumerate() {
            let mut line: Vec<u8> = vec![];
            for (j, c) in line_str.chars().enumerate() {
                line.push(match c {
                    '#' => 0x10,
                    '^' => 0x1,
                    _ => 0x0,
                });
                if c == '^' {
                    position = (i as isize, j as isize);
                }
            }
            grid.push(line);
        }
        Self {
            position,
            direction: 0,
            grid,
        }
    }

    fn height(&self) -> isize {
        self.grid.len() as isize
    }

    fn width(&self) -> isize {
        self.grid[0].len() as isize
    }

    fn inside(&self, ij: (isize, isize)) -> bool {
        0 <= ij.0 && ij.0 < self.height() && 0 <= ij.1 && ij.1 < self.width()
    }

    fn at(&self, ij: (isize, isize)) -> u8 {
        self.grid[ij.0 as usize][ij.1 as usize]
    }

    fn set(&mut self, ij: (isize, isize), new_val: u8) {
        self.grid[ij.0 as usize][ij.1 as usize] = new_val;
    }

    fn dir_vector(&self) -> (isize, isize) {
        match self.direction {
            0 => (-1, 0),
            1 => (0, 1),
            2 => (1, 0),
            3 => (0, -1),
            _ => panic!("wrong direction"),
        }
    }
    /// return codes:
    ///     0 if already visited in diferent directions or rotating
    ///     1 if not visited before
    ///     2 if already visited in same direction => loop
    ///     3 out of the grid
    fn step(&mut self) -> usize {
        let dir = self.dir_vector();
        let new_pos = (self.position.0 + dir.0, self.position.1 + dir.1);
        if self.inside(new_pos) {
            if self.at(new_pos) == 0x10 {
                self.direction = (self.direction + 1) % 4;
                return 0;
            }
            self.position = new_pos;
            let grid_val = self.at(self.position);
            if grid_val & (1 << self.direction) == 0 {
                self.set(self.position, grid_val + (1 << self.direction));
                if grid_val & 0xf > 0 {
                    return 0;
                } else {
                    return 1;
                }
            } else {
                return 2;
            }
        }
        3
    }

    fn is_looped(&mut self) -> bool {
        loop {
            let s = self.step();
            if s == 2 {
                return true;
            }
            if s == 3 {
                return false;
            }
        }
    }
}

fn prob1(lines: &[&str]) -> usize {
    let mut ret: usize = 1; // starting position count
    let mut maze = Maze::new(lines);
    loop {
        let s = maze.step();
        if s == 3 {
            break;
        }
        ret += s;
    }
    ret
}

fn prob2(lines: &[&str]) -> usize {
    let mut ret: usize = 0;
    let orig_maze = Maze::new(lines);
    for i in 0..orig_maze.height() {
        for j in 0..orig_maze.width() {
            let mut maze = orig_maze.clone();
            maze.set((i, j), 0x10);
            if maze.is_looped() {
                ret += 1;
            }
        }
    }
    ret
}

pub(crate) fn main() {
    let lines: Vec<&str> = include_str!("../input/day_6").trim().split("\n").collect();
    println!("prob1: {}", prob1(&lines));
    println!("prob2: {}", prob2(&lines));
}

#[cfg(test)]
pub mod tests {
    use crate::day_6::{prob1, prob2, Maze};

    fn input() -> Vec<&'static str> {
        vec![
            "....#.....",
            ".........#",
            "..........",
            "..#.......",
            ".......#..",
            "..........",
            ".#..^.....",
            "........#.",
            "#.........",
            "......#...",
        ]
    }

    #[test]
    fn test_new() {
        let maze = Maze::new(&input());
        assert_eq!(
            maze.grid
                .into_iter()
                .flatten()
                .map(|u| u as isize)
                .sum::<isize>(),
            8 * 0x10 + 1
        );
        assert_eq!(maze.position, (6, 4));
        assert_eq!(maze.direction, 0);
    }

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(&input()), 41);
    }

    #[test]
    fn test_prob2() {
        assert_eq!(prob2(&input()), 6);
    }
}
