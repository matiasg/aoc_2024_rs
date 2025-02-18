use itertools::iproduct;
use std::collections::{HashMap, HashSet};

use crate::utils::{Maze, IJ};

fn prob1(input: &[&str], min_diff: usize) -> usize {
    let input: Vec<String> = input.iter().map(|s| s.to_string()).collect();
    let m = Maze::from(&input);
    let d0 = m.distance().unwrap();

    let mut mdig = m.as_digraph();
    let wall = m.wall.clone();
    for &ij in wall.iter() {
        mdig.nodes.push(ij);
        for kl in m.neighbors(ij) {
            mdig.edges.push((kl, ij));
        }
    }

    let weights: HashMap<(IJ, IJ), usize> =
        mdig.edges.iter().map(|&(ij, kl)| ((ij, kl), 1)).collect();
    let start_dists = mdig.distance_with(m.start, wall.clone(), weights.clone());
    let end_dists = mdig.distance_with(m.end, wall.clone(), weights.clone());

    let sum_dists: HashMap<IJ, usize> = start_dists
        .keys()
        .map(|&ij| {
            (
                ij,
                start_dists.get(&ij).unwrap() + end_dists.get(&ij).unwrap(),
            )
        })
        .collect();
    sum_dists.values().filter(|&d| d + min_diff <= d0).count()
}

fn prob2(input: &[&str], max_dist: isize, min_save: usize) -> usize {
    let input: Vec<String> = input.iter().map(|s| s.to_string()).collect();
    let m = Maze::from(&input);
    let mdig = m.as_digraph();
    let d0 = mdig.distance(m.start, m.end).unwrap();
    let input_rev: Vec<String> = input
        .iter()
        .map(|s| {
            s.replace("#", "x")
                .replace(".", "#")
                .replace("S", "#")
                .replace("E", "#")
        })
        .collect();
    let mdig_reverse = Maze::from(&input_rev).as_digraph();
    let wall: HashSet<IJ> = mdig_reverse.nodes.iter().copied().collect();
    let mut empty: Vec<IJ> = mdig.nodes.to_vec();
    empty.sort();
    assert_eq!(
        wall.len() + empty.len(),
        m.height as usize * m.width as usize
    );
    let minij = IJ { i: 0, j: 0 };
    let maxij = IJ {
        i: m.height - 1,
        j: m.width - 1,
    };
    let sdists = mdig.distances_from(m.start);
    let edists = mdig.distances_from(m.end);
    let mut ret = 0usize;
    for &ij in empty.iter() {
        for kl in ij.closer_than_and_bounded(max_dist, minij, maxij) {
            if wall.contains(&kl) {
                continue;
            }
            if !iproduct![ij.neighbors().iter(), kl.neighbors().iter()]
                .any(|(ij_nbr, kl_nbr)| ij_nbr.inside(minij, maxij) && kl_nbr.inside(minij, maxij))
            {
                continue;
            }
            let drev = ij.distance(&kl) as usize;
            let dsij = sdists.get(&ij).unwrap();
            let dkle = edists.get(&kl).unwrap();
            let dtot = dsij + drev + dkle;
            if dtot + min_save <= d0 {
                ret += 1;
            }
        }
    }
    ret
}

pub(crate) fn main() {
    let input: Vec<&str> = include_str!("../input/day_20").trim().split("\n").collect();
    println!("prob1: {}", prob1(&input.clone(), 100));
    println!("prob2: {}", prob2(&input.clone(), 20, 100));
}

#[cfg(test)]
pub mod tests {
    use super::{prob1, prob2};

    fn input() -> Vec<&'static str> {
        vec![
            "###############",
            "#...#...#.....#",
            "#.#.#.#.#.###.#",
            "#S#...#.#.#...#",
            "#######.#.#.###",
            "#######.#.#...#",
            "#######.#.###.#",
            "###..E#...#...#",
            "###.#######.###",
            "#...###...#...#",
            "#.#####.#.###.#",
            "#.#...#.#.#...#",
            "#.#.#.#.#.#.###",
            "#...#...#...###",
            "###############",
        ]
    }

    fn input2() -> Vec<&'static str> {
        vec![
            "#########################################",
            "#...#.............#.....#.....#.....#...#",
            "###.#.###.#########.###.###.#####.###.#.#",
            "#...#...#.#.#.....#...#...#.#.........#.#",
            "#..##.###.#.#####.#####.#.#.#.#####.#.#.#",
            "#.......#.....#.#.....#.#...#...#...#.#.#",
            "#.###########.#.#.####.####.#.###########",
            "#.#.#...#...#.....#.................#...#",
            "#.#.#.#.#.#.###.#.#.###.#########.#####.#",
            "#.....#...#.....#...#.........#...#.#.#.#",
            "#####.#####.#####.#.#.#.#.#######.#.#.#.#",
            "#.....#.........#.#.#...#...#...#.#...#.#",
            "#.#########.#######.#####.#.##..###.###.#",
            "#...#.......#.....#.#...#.#...#.....#...#",
            "#.###.###########.#.###.#.#.###.#######.#",
            "#.#.#.............#.....#.#...#...#.....#",
            "###.#.#####.#####.#.###.#.#####.#####.###",
            "#...#.#.........#.#...#...#...#.#.....#.#",
            "###.###.#.#########.#####.###.#.#.#.#.#.#",
            "#S#.#...#.#.....#.....#.........#.#.#..E#",
            "#.#.#.#########.#.#########.#.###.#####.#",
            "#.....#.........#...#.#...#.#.....#...#.#",
            "###.#####..##.#.#####.#.###.#####.###.###",
            "#.#.#...#.#.#.#.#...#...#...#.........#.#",
            "#.#.###.###.#.#.#.#####.####.##.#.#####.#",
            "#.#.#.#.#.#...#.........#.#...#.#.#...#.#",
            "#.#.#.#.#.#####.###.#.#.#.###.#.###.###.#",
            "#...#.......#...#...#.#.#.........#.#...#",
            "#######.#####.#####.###.#.#.#####.#.###.#",
            "#.............#.....#.#.#.#.....#.......#",
            "###############.#####.#.#########.#.#.###",
            "#.....#...#.#.........#.#...#...#.#.#.#.#",
            "#.#.#.#.#.#.###.#########.###.###.#####.#",
            "#.#.#.#.#...........#.#.............#...#",
            "###.#.#.###.#######.#.#.#.###.###.#.#.###",
            "#...#...#...#.#...#.#...#...#.#.#.#.#...#",
            "###.#.#######.#.#.#.###.#####.#..##.#.###",
            "#.#.#...#.....#.#.#.......#.#.#...#.....#",
            "#.#.#####.###.#.#.#.#.#####.#####.###.#.#",
            "#.....#.....#.......#.............#...#.#",
            "#########################################",
        ]
    }

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(&input(), 1), 44);
    }

    #[test]
    fn test_prob2() {
        assert_eq!(prob2(&input(), 20, 50), 285);
        assert_eq!(prob2(&input2(), 25, 35), 259);
        assert_eq!(prob2(&input2(), 20, 30), 299);
    }
}
