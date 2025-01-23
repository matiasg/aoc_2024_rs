use std::collections::{HashMap, HashSet};

fn parse(lines: &[&str]) -> (HashMap<char, Vec<(isize, isize)>>, isize, isize) {
    let height = lines.len() as isize;
    let width = lines[0].len() as isize;
    let mut ret: HashMap<char, Vec<(isize, isize)>> = HashMap::new();
    for (i, line) in lines.iter().enumerate() {
        for (j, chr) in line.chars().enumerate() {
            if chr != '.' {
                ret.entry(chr).or_default().push((i as isize, j as isize));
            }
        }
    }
    (ret, height, width)
}

fn count_symmetries(
    antennas: HashMap<char, Vec<(isize, isize)>>,
    height: isize,
    width: isize,
) -> usize {
    let mut added: HashSet<(isize, isize)> = HashSet::new();
    for (_, positions) in antennas {
        for (i, j) in positions.clone() {
            for (k, l) in positions.clone() {
                if (k, l) == (i, j) {
                    continue;
                }
                let (ni, nj) = (2 * k - i, 2 * l - j);
                if 0 <= ni && ni < height && 0 <= nj && nj < width {
                    added.insert((ni, nj));
                }
            }
        }
    }
    added.len()
}

fn count_whole_lines(
    antennas: HashMap<char, Vec<(isize, isize)>>,
    height: isize,
    width: isize,
) -> usize {
    let mut added: HashSet<(isize, isize)> = HashSet::new();
    for (_, positions) in antennas {
        for (i, j) in positions.clone() {
            for (k, l) in positions.clone() {
                if (k, l) != (i, j) {
                    for t in 1.. {
                        let (ni, nj) = (i + t * (k - i), j + t * (l - j));
                        if 0 <= ni && ni < height && 0 <= nj && nj < width {
                            added.insert((ni, nj));
                        } else {
                            break;
                        }
                    }
                }
            }
        }
    }
    added.len()
}

fn prob1(lines: &[&str]) -> usize {
    let (g, h, w) = parse(lines);
    count_symmetries(g, h, w)
}

fn prob2(lines: &[&str]) -> usize {
    let (g, h, w) = parse(lines);
    count_whole_lines(g, h, w)
}

pub(crate) fn main() {
    let lines: Vec<&str> = include_str!("../input/day_8").trim().split("\n").collect();
    println!("prob1: {}", prob1(&lines));
    println!("prob2: {}", prob2(&lines));
}

#[cfg(test)]
pub mod tests {
    use super::{parse, prob1, prob2};

    fn input() -> Vec<&'static str> {
        vec![
            "............",
            "........0...",
            ".....0......",
            ".......0....",
            "....0.......",
            "......A.....",
            "............",
            "............",
            "........A...",
            ".........A..",
            "............",
            "............",
        ]
    }

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(&input()), 14);
    }

    #[test]
    fn test_prob2() {
        assert_eq!(prob2(&input()), 34);
    }

    #[test]
    fn test_parse() {
        let (g, h, w) = parse(&input());
        assert_eq!(g.len(), 2);
        assert_eq!(g[&'0'], vec![(1, 8), (2, 5), (3, 7), (4, 4)]);
        assert_eq!(h, 12);
        assert_eq!(w, 12);
    }
}
