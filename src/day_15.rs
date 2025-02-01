use std::collections::HashSet;

struct Maze {
    height: isize,
    width: isize,
    boxes: HashSet<(isize, isize)>,
    rocks: HashSet<(isize, isize)>,
    instructions: Vec<char>,
    position: (isize, isize),
}

impl Maze {
    fn from(lines: &[&str]) -> Self {
        let width = lines[0].len() as isize;
        let (height, _) = lines
            .iter()
            .enumerate()
            .find(|(_, &l)| l.is_empty())
            .unwrap();
        let mut boxes: HashSet<(isize, isize)> = HashSet::new();
        let mut rocks: HashSet<(isize, isize)> = HashSet::new();
        let mut position: (isize, isize) = (0, 0);
        for (i, line) in lines.get(0..height).unwrap().iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                let p = (i as isize, j as isize);
                match c {
                    '#' => _ = rocks.insert(p),
                    'O' => _ = boxes.insert(p),
                    '@' => position = p,
                    _ => (),
                }
            }
        }
        let mut instructions: Vec<char> = vec![];
        for line in lines.get(height + 1..).unwrap() {
            instructions.extend(line.chars());
        }
        Self {
            height: height as isize,
            width,
            boxes,
            rocks,
            instructions,
            position,
        }
    }

    fn wide_from(lines: &[&str]) -> Self {
        let thin = Self::from(lines);
        let boxes: HashSet<(isize, isize)> = thin.boxes.iter().map(|&(i, j)| (i, 2 * j)).collect();
        let rocks: HashSet<(isize, isize)> = thin
            .rocks
            .iter()
            .flat_map(|&(i, j)| vec![(i, 2 * j), (i, 2 * j + 1)])
            .collect();
        let position = (thin.position.0, 2 * thin.position.1);
        Self {
            height: thin.height,
            width: 2 * thin.width,
            boxes,
            rocks,
            instructions: thin.instructions,
            position,
        }
    }

    fn direction(inst: char) -> (isize, isize) {
        match inst {
            '>' => (0, 1),
            '<' => (0, -1),
            'v' => (1, 0),
            '^' => (-1, 0),
            _ => panic!("wrong instruction"),
        }
    }

    fn do_move(&mut self, inst_idx: usize) {
        let inst = self.instructions[inst_idx];
        let dir = Self::direction(inst);
        let next_nobox_dist = (1..self.height.max(self.width))
            .find(|&t| {
                !self
                    .boxes
                    .contains(&(self.position.0 + t * dir.0, self.position.1 + t * dir.1))
            })
            .unwrap();
        let nobox_pos = (
            self.position.0 + next_nobox_dist * dir.0,
            self.position.1 + next_nobox_dist * dir.1,
        );
        if !self.rocks.contains(&nobox_pos) {
            let next_position = (self.position.0 + dir.0, self.position.1 + dir.1);
            if next_nobox_dist > 1 {
                self.boxes.remove(&next_position);
                self.boxes.insert(nobox_pos);
            }
            self.position = next_position;
        }
    }

    fn wide_occupies(&self, pos: (isize, isize)) -> (u8, (isize, isize)) {
        // 0: unoccupied, pos
        // 1: box, pos of box (might be to the left of pos)
        // 2: rock, pos of rock
        if self.rocks.contains(&pos) {
            return (2, pos);
        }
        if self.boxes.contains(&pos) {
            return (1, pos);
        }
        if self.boxes.contains(&(pos.0, pos.1 - 1)) {
            return (1, (pos.0, pos.1 - 1));
        }
        (0, pos)
    }

    fn wide_do_move(&mut self, inst: char) {
        let dir = Self::direction(inst);
        let next_position = (self.position.0 + dir.0, self.position.1 + dir.1);
        let (o, p) = self.wide_occupies(next_position);
        if o == 0 {
            self.position = next_position;
        } else if o == 1 {
            if let Some(tomove) = self.moving_boxes(p, dir) {
                self.boxes = self.boxes.difference(&tomove).copied().collect();
                for p in tomove {
                    self.boxes.insert((p.0 + dir.0, p.1 + dir.1));
                }
                self.position = next_position;
            }
        }
    }

    fn coordinate_sum(&self) -> isize {
        self.boxes.iter().map(|&(i, j)| 100 * i + j).sum()
    }

    fn moving_boxes(
        &self,
        p: (isize, isize),
        dir: (isize, isize),
    ) -> Option<HashSet<(isize, isize)>> {
        let mut ret: HashSet<(isize, isize)> = HashSet::new();
        let mut nextline: HashSet<(isize, isize)> = HashSet::from([p]);
        while !nextline.is_empty() {
            let nextoccupied: HashSet<(isize, isize)> = if dir.0 == 0 {
                nextline
                    .iter()
                    .map(|&(i, j)| (i, j + dir.1 + (dir.1 + 1) / 2))
                    .collect()
            } else {
                nextline
                    .iter()
                    .flat_map(|&(i, j)| vec![(i + dir.0, j), (i + dir.0, j + 1)])
                    .collect()
            };
            if !self
                .rocks
                .intersection(&nextoccupied)
                .copied()
                .collect::<Vec<(isize, isize)>>()
                .is_empty()
            {
                return None;
            }
            let nextboxes: HashSet<(isize, isize)> = if dir.0 == 0 {
                nextline.iter().map(|&(i, j)| (i, j + 2 * dir.1)).collect()
            } else {
                nextline
                    .iter()
                    .flat_map(|&(i, j)| {
                        vec![(i + dir.0, j - 1), (i + dir.0, j), (i + dir.0, j + 1)]
                    })
                    .collect()
            };
            ret.extend(nextline.into_iter());
            nextline = nextboxes.intersection(&self.boxes).copied().collect();
        }
        Some(ret)
    }

    fn display(&self) {
        let mut d: Vec<String> =
            vec![String::from_iter(vec![' '; self.width as usize]); self.height as usize];
        for p in self.rocks.iter() {
            d[p.0 as usize].replace_range((p.1 as usize)..(p.1 as usize + 1), "#");
        }
        for p in self.boxes.iter() {
            d[p.0 as usize].replace_range((p.1 as usize)..(p.1 as usize + 2), "[]");
        }
        d[self.position.0 as usize].replace_range(
            (self.position.1 as usize)..(self.position.1 as usize + 1),
            "@",
        );
        println!("{}", String::from_iter(vec!['='; self.width as usize]));
        d.iter().for_each(|l| println!("{l}"));
        println!("{}", String::from_iter(vec!['='; self.width as usize]));
    }
}

fn prob1(input: &[&str]) -> isize {
    let mut m = Maze::from(input);
    for i in 0..m.instructions.len() {
        m.do_move(i);
    }
    m.coordinate_sum()
}

fn prob2(input: &[&str]) -> isize {
    let mut m = Maze::wide_from(input);
    let insts = m.instructions.clone();
    for c in insts {
        m.wide_do_move(c);
    }
    m.coordinate_sum()
}

pub(crate) fn main() {
    let input: Vec<&str> = include_str!("../input/day_15").trim().split("\n").collect();
    println!("prob1: {}", prob1(&input.clone()));
    println!("prob2: {}", prob2(&input.clone()));
}

#[cfg(test)]
pub mod tests {
    use std::collections::HashSet;

    use super::{prob1, prob2, Maze};

    fn input() -> Vec<&'static str> {
        vec![
            "##########",
            "#..O..O.O#",
            "#......O.#",
            "#.OO..O.O#",
            "#..O@..O.#",
            "#O#..O...#",
            "#O..O..O.#",
            "#.OO.O.OO#",
            "#....O...#",
            "##########",
            "",
            "<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^",
            "vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v",
            "><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<",
            "<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^",
            "^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><",
            "^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^",
            ">^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^",
            "<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>",
            "^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>",
            "v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
        ]
    }

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(&input()), 10092);
    }

    #[test]
    fn test_maze() {
        let m = Maze::from(&input());
        assert_eq!(m.height, 10);
        assert_eq!(m.width, 10);
        assert_eq!(m.rocks.len(), 10 * 4 - 4 + 1);
        assert_eq!(m.boxes.len(), 21);
        assert_eq!(m.position, (4, 4));
        assert_eq!(m.instructions.len(), 70 * 10);
        assert_eq!(
            m.instructions.get(..4).unwrap(),
            "<vv>".chars().collect::<Vec<char>>()
        )
    }

    #[test]
    fn test_move() {
        let mut m = Maze::from(&input());
        m.do_move(0);
        assert!(m.boxes.contains(&(4, 2)));
        assert_eq!(m.position, (4, 3));
        assert_eq!(m.boxes.len(), 21);
        for i in 1..m.instructions.len() {
            m.do_move(i);
        }
        assert_eq!(m.boxes.len(), 21);
        assert_eq!(m.position, (4, 3));
    }

    #[test]
    fn test_wide_maze() {
        let mut m = Maze::wide_from(&input());
        assert_eq!(m.width, 20);
        assert_eq!(m.height, 10);
        assert_eq!(m.boxes.len(), 21);
        assert_eq!(m.rocks.len(), 74);
        assert!(m.boxes.contains(&(4, 6)));
        assert_eq!(m.wide_occupies((4, 7)), (1, (4, 6)));
        m.wide_do_move('<');
        assert!(m.boxes.contains(&(4, 5)));
        assert_eq!(m.position, (4, 7));
        m.wide_do_move('^');
        assert_eq!(m.boxes.len(), 21);
        assert_eq!(m.position, (3, 7));
        assert!(m.boxes.contains(&(2, 6)));
        m.wide_do_move('^');
        assert_eq!(m.position, (3, 7));

        let mut m = Maze::wide_from(&input());
        m.wide_do_move('<');
        m.wide_do_move('v');
        m.wide_do_move('<');
        m.wide_do_move('^');
        let some: HashSet<(isize, isize)> = HashSet::from([(2, 4), (2, 6), (3, 5)]);
        assert!(m.boxes.is_superset(&some));
        let prev_boxes = m.boxes.clone();
        m.wide_do_move('^');
        assert_eq!(m.boxes, prev_boxes);
    }

    fn input2() -> Vec<&'static str> {
        vec!["#####", "#.@O#", "#####", "", ">"]
    }

    #[test]
    fn test_wide_move() {
        let mut m = Maze::wide_from(&input());
        m.wide_do_move('^');
        assert!(m.boxes.is_superset(&HashSet::from([(3, 4), (3, 6)])));
        m.wide_do_move('<');
        assert!(m.boxes.is_superset(&HashSet::from([(3, 3), (3, 5)])));
        m.wide_do_move('<');
        assert!(m.boxes.is_superset(&HashSet::from([(3, 2), (3, 4)])));
        let mut m = Maze::wide_from(&input2());
        assert_eq!(m.boxes, HashSet::from([(1, 6)]));
        m.wide_do_move('>');
        assert_eq!(m.boxes, HashSet::from([(1, 6)]));
        m.wide_do_move('>');
        assert_eq!(m.boxes, HashSet::from([(1, 6)]));
    }

    #[test]
    fn test_prob2() {
        assert_eq!(prob2(&input()), 9021);
    }
}
