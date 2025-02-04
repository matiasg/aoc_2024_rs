use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap, HashSet},
    iter,
};

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

type Ijo = (isize, isize, Orientation);

struct Maze {
    graph: DiGraph<Ijo>,
    weights: HashMap<(Ijo, Ijo), isize>,
    start: (isize, isize, Orientation),
    end: (isize, isize),
}

fn parse(input: &[&str]) -> Maze {
    let mut start: Ijo = (0, 0, Orientation::East);
    let mut end: (isize, isize) = (0, 0);
    let mut nodes: Vec<Ijo> = vec![];
    let edges: Vec<(Ijo, Ijo)> = vec![];
    let maxi = input.len() as isize;
    let maxj = input[0].len() as isize;
    let mut weights: HashMap<(Ijo, Ijo), isize> = HashMap::new();
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
    let graph: DiGraph<Ijo> = DiGraph::from(nodes, edges);
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
                .collect::<Vec<Ijo>>(),
            m.weights,
        )
        .values()
        .min()
        .unwrap()
}

fn prob2(input: &[&str]) -> usize {
    let mindist = Reverse(prob1(input));
    let m = parse(input);
    let ends: Vec<Ijo> = Orientation::iter().map(|o| (m.end.0, m.end.1, o)).collect();
    // somewhat copy distance_with but keeping track of paths
    let mut wchildren: HashMap<Ijo, HashSet<Ijo>> = HashMap::new();
    for (f, t) in m.weights.keys().copied() {
        wchildren.entry(f).or_default().insert(t);
    }
    let ends: HashSet<Ijo> = ends.into_iter().collect();
    let mut ret: HashSet<Ijo> = HashSet::new();

    let mut shortest_used: HashMap<Ijo, HashSet<Ijo>> = HashMap::new();
    shortest_used.insert(m.start, HashSet::from([m.start]));
    let mut shortest_length: HashMap<Ijo, isize> = HashMap::from([(m.start, 0)]);

    let mut unvisited: BinaryHeap<(Reverse<isize>, Ijo)> = BinaryHeap::new();
    unvisited.push((Reverse(0), m.start));
    while !unvisited.is_empty() && unvisited.peek().unwrap().0 >= mindist {
        let (rdist, node) = unvisited.pop().unwrap();
        if Reverse(*shortest_length.get(&node).unwrap_or(&isize::MAX)) > rdist {
            continue;
        }
        let Reverse(start_node_length) = rdist;
        if ends.contains(&node) {
            ret.extend(shortest_used.get(&node).unwrap().iter().copied());
        }
        for &child in wchildren.entry(node).or_default().iter() {
            let &node_child_length = m.weights.get(&(node, child)).unwrap();

            let new_length = start_node_length + node_child_length;
            let new_used: HashSet<Ijo> = shortest_used
                .get(&node)
                .unwrap()
                .iter()
                .chain(iter::once(&child))
                .copied()
                .collect();
            if let Some(&prev_length) = shortest_length.get(&child) {
                match new_length.cmp(&prev_length) {
                    Ordering::Less => {
                        shortest_length.insert(child, new_length);
                        shortest_used.insert(child, new_used);
                    }
                    Ordering::Equal => {
                        shortest_used
                            .entry(child)
                            .and_modify(|su| su.extend(new_used));
                    }
                    Ordering::Greater => {}
                }
            } else {
                shortest_length.insert(child, new_length);
                shortest_used.insert(child, new_used);
            }

            let new_rev_dist = Reverse(new_length);
            if new_rev_dist >= mindist {
                unvisited.push((new_rev_dist, child));
            }
        }
    }
    let ret: HashSet<(isize, isize)> = ret.iter().map(|&(i, j, _)| (i, j)).collect();
    ret.len()
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
        assert_eq!(prob2(&input()), 45);
    }
}
