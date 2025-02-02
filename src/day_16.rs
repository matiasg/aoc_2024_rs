use std::collections::HashMap;

use crate::utils::DiGraph;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Orientation {
    East,
    South,
    West,
    North,
}
impl Orientation {
    fn iter() -> impl Iterator<Item = Orientation> {
        [
            Orientation::East,
            Orientation::South,
            Orientation::West,
            Orientation::North,
        ]
        .iter()
        .copied()
    }
    fn clock() -> Vec<(Orientation, Orientation)> {
        let oris: Vec<Orientation> = Self::iter().collect();
        (0..oris.len())
            .map(|i| (oris[i], oris[(i + 1) % 4]))
            .collect()
    }
    fn direction() -> Vec<(Orientation, (isize, isize))> {
        vec![
            (Orientation::East, (0, 1)),
            (Orientation::South, (1, 0)),
            (Orientation::West, (0, -1)),
            (Orientation::North, (-1, 0)),
        ]
    }
}

type XYO = (isize, isize, Orientation);

struct Maze {
    graph: DiGraph<XYO>,
    weights: HashMap<(XYO, XYO), isize>,
    start: (isize, isize, Orientation),
    end: (isize, isize),
}

fn contiguous(i: isize, j: isize, maxi: isize, maxj: isize) -> Vec<(isize, isize)> {
    let mut ret = vec![];
    if i > 0 {
        ret.push((i - 1, j));
    }
    if j > 0 {
        ret.push((i, j - 1));
    }
    if i < maxi - 1 {
        ret.push((i + 1, j));
    }
    if j < maxj - 1 {
        ret.push((i, j + 1));
    }

    ret
}

fn parse(input: &[&str]) -> Maze {
    let mut start: XYO = (0, 0, Orientation::East);
    let mut end: (isize, isize) = (0, 0);
    let mut nodes: Vec<XYO> = vec![];
    let edges: Vec<(XYO, XYO)> = vec![];
    let maxi = input.len() as isize;
    let maxj = input[0].len() as isize;
    let mut weights: HashMap<(XYO, XYO), isize> = HashMap::new();
    for (i, line) in input.iter().enumerate() {
        let ii = i as isize;
        for (j, c) in line.chars().enumerate() {
            let ij = j as isize;
            if c == 'S' {
                start = (i as isize, j as isize, Orientation::East)
            } else if c == 'E' {
                end = (i as isize, j as isize);
            }
            if c != '#' {
                nodes.extend(Orientation::iter().map(|io| (ii, ij, io)));
                for (oa, ob) in Orientation::clock() {
                    weights.insert(((ii, ij, oa), (ii, ij, ob)), 1000);
                    weights.insert(((ii, ij, ob), (ii, ij, oa)), 1000);
                }
                for (oa, dir) in Orientation::direction() {
                    let (ii1, ij1) = (ii + dir.0, ij + dir.1);
                    if (0 <= ii1 && ii1 < maxi && 0 <= ij1 && ij1 < maxj)
                        && &input[ii1 as usize][ij1 as usize..ij1 as usize + 1] != "#"
                    {
                        weights.insert(((ii, ij, oa), (ii1, ij1, oa)), 1);
                    }
                }
            }
        }
    }
    let graph: DiGraph<XYO> = DiGraph::from(nodes, edges);
    Maze {
        graph,
        weights,
        start,
        end,
    }
}

fn prob1(input: &[&str]) -> isize {
    let m = parse(input);
    *m.graph
        .distance_with(
            m.start,
            Orientation::iter()
                .map(|o| (m.end.0, m.end.1, o))
                .collect::<Vec<XYO>>(),
            m.weights,
        )
        .values()
        .min()
        .unwrap()
}

fn prob2(input: &[&str]) -> usize {
    0
}

pub(crate) fn main() {
    let input: Vec<&str> = include_str!("../input/day_16").trim().split("\n").collect();
    println!("prob1: {}", prob1(&input.clone()));
    println!("prob2: {}", prob2(&input.clone()));
}

#[cfg(test)]
pub mod tests {
    use super::{parse, prob1, prob2, Orientation};

    fn input() -> Vec<&'static str> {
        vec![
            "###############",
            "#.......#....E#",
            "#.#.###.#.###.#",
            "#.....#.#...#.#",
            "#.###.#####.#.#",
            "#.#.#.......#.#",
            "#.#.#####.###.#",
            "#...........#.#",
            "###.#.#####.#.#",
            "#...#.....#.#.#",
            "#.#.#.###.#.#.#",
            "#.....#...#.#.#",
            "#.###.#.#.#.#.#",
            "#S..#.....#...#",
            "###############",
        ]
    }

    #[test]
    fn test_parse() {
        let maze = parse(&input());
        assert_eq!(maze.graph.len(), 416);
        assert_eq!(maze.start, (13, 1, Orientation::East));
        assert_eq!(maze.end, (1, 13));
    }

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(&input()), 7036);
    }

    #[test]
    fn test_prob2() {
        prob2(&input());
    }
}
