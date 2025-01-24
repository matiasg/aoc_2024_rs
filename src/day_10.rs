use std::collections::{HashMap, HashSet};

struct MapGrid {
    starts: Vec<usize>,
    nodes: HashMap<usize, Vec<usize>>,
}
impl MapGrid {
    fn from(lines: &[&str]) -> Self {
        let height = lines.len();
        let width = lines[0].len();
        let mut starts: Vec<usize> = vec![];
        let mut nodes: HashMap<usize, Vec<usize>> = HashMap::new();
        for (i, line) in lines.iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                let c = c as i32;
                if c == '0' as i32 {
                    starts.push(i * width + j)
                };
                for (k, linek) in lines.iter().enumerate().take(i + 2).skip(i.max(1) - 1) {
                    for (l, d) in linek.chars().enumerate().take(j + 2).skip(j.max(1) - 1) {
                        if (i as isize - k as isize).abs() + (j as isize - l as isize).abs() == 1
                            && k < height
                            && l < width
                        {
                            let d = d as i32;
                            if d == c + 1 {
                                nodes.entry(i * width + j).or_default().push(k * width + l);
                            }
                        }
                    }
                }
            }
        }
        Self { starts, nodes }
    }

    fn reachable_in(&self, start: usize, steps: usize) -> HashSet<usize> {
        let mut ret = HashSet::from([start]);
        for _ in 0..steps {
            ret = ret
                .iter()
                .flat_map(|u| self.nodes.get(u).unwrap_or(&vec![]).clone())
                .collect();
        }
        ret
    }

    fn reachable_counting_paths(&self, start: usize, steps: usize) -> Vec<usize> {
        let mut ret = Vec::from([start]);
        for _ in 0..steps {
            ret = ret
                .iter()
                .flat_map(|u| self.nodes.get(u).unwrap_or(&vec![]).clone())
                .collect();
        }
        ret
    }
}

fn prob1(input: &[&'static str]) -> usize {
    let g = MapGrid::from(input);
    g.starts.iter().map(|&s| g.reachable_in(s, 9).len()).sum()
}

fn prob2(input: &[&'static str]) -> usize {
    let g = MapGrid::from(input);
    g.starts
        .iter()
        .map(|&s| g.reachable_counting_paths(s, 9).len())
        .sum()
}

pub(crate) fn main() {
    let input: Vec<&str> = include_str!("../input/day_10").trim().split("\n").collect();
    println!("prob1: {}", prob1(&input.clone()));
    println!("prob2: {}", prob2(&input.clone()));
}

#[cfg(test)]
pub mod tests {
    use std::collections::HashSet;

    use super::{prob1, prob2, MapGrid};

    fn input() -> Vec<&'static str> {
        vec![
            "89010123", "78121874", "87430965", "96549874", "45678903", "32019012", "01329801",
            "10456732",
        ]
    }

    #[test]
    fn test_parse() {
        let g = MapGrid::from(&input());
        assert_eq!(g.starts.len(), 9);
        assert_eq!(g.nodes[&2], vec![3, 10]);
    }

    #[test]
    fn test_from_to_steps() {
        let g = MapGrid::from(&input());
        assert_eq!(g.reachable_in(2, 1), HashSet::from([3, 10]));
        assert_eq!(g.reachable_in(2, 9).len(), 5);
        assert_eq!(g.reachable_in(4, 9).len(), 6);
        assert_eq!(g.reachable_in(56, 1), HashSet::new());
    }

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(&input()), 36);
    }

    #[test]
    fn test_prob2() {
        assert_eq!(prob2(&input()), 81);
    }
}
