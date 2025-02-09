use std::collections::HashSet;

use crate::utils::DiGraph;

fn shortest_path(xys: &[XY], cut_at: usize, width: isize, height: isize) -> Option<usize> {
    let graph = make_graph(&xys[0..cut_at], width, height);
    let start = XY { x: 0, y: 0 };
    let end = XY {
        x: width - 1,
        y: height - 1,
    };
    graph.distance(start, end)
}

fn prob1(input: &[&str], cut_at: usize, width: isize, height: isize) -> usize {
    let xys: Vec<XY> = parse(input);
    shortest_path(&xys, cut_at, width, height).unwrap()
}

fn prob2(input: &[&str], width: isize, height: isize) -> XY {
    let xys: Vec<XY> = parse(input);
    let mut min = 0;
    let mut max = xys.len();
    while max > min + 1 {
        let mid = (max + min) / 2;
        if shortest_path(&xys, mid, width, height).is_some() {
            min = mid;
        } else {
            max = mid;
        }
    }
    xys[min]
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
struct XY {
    x: isize,
    y: isize,
}
impl XY {
    fn from(line: &str) -> Self {
        let xy: Vec<&str> = line.split(",").collect();
        Self {
            x: xy[0].parse().unwrap(),
            y: xy[1].parse().unwrap(),
        }
    }

    fn adjacent(&self, width: isize, height: isize) -> Vec<XY> {
        let mut ret: Vec<XY> = vec![];
        if self.x > 0 {
            ret.push(XY {
                x: self.x - 1,
                y: self.y,
            })
        }
        if self.x < width - 1 {
            ret.push(XY {
                x: self.x + 1,
                y: self.y,
            })
        }
        if self.y > 0 {
            ret.push(XY {
                x: self.x,
                y: self.y - 1,
            })
        }
        if self.y < height - 1 {
            ret.push(XY {
                x: self.x,
                y: self.y + 1,
            })
        }
        ret
    }
}

fn make_graph(broken: &[XY], width: isize, height: isize) -> DiGraph<XY> {
    let broken: HashSet<XY> = broken.iter().copied().collect();
    let nodes: Vec<XY> = (0..width)
        .flat_map(|x| (0..height).map(move |y| XY { x, y }))
        .filter(|xy| !broken.contains(xy))
        .collect();
    let mut edges: Vec<(XY, XY)> = vec![];
    for xy in nodes.iter().copied() {
        for xy2 in xy.adjacent(width, height) {
            if !broken.contains(&xy2) {
                edges.push((xy, xy2));
            }
        }
    }
    DiGraph { nodes, edges }
}

fn parse(input: &[&str]) -> Vec<XY> {
    input.iter().map(|l| XY::from(l)).collect()
}

pub(crate) fn main() {
    let input: Vec<&str> = include_str!("../input/day_18").trim().split("\n").collect();
    println!("prob1: {}", prob1(&input.clone(), 1024, 71, 71));
    let p2 = prob2(&input.clone(), 72, 72);
    println!("prob2: {},{}", p2.x, p2.y);
}

#[cfg(test)]
pub mod tests {
    use super::{prob1, prob2, XY};

    fn input() -> Vec<&'static str> {
        vec![
            "5,4", "4,2", "4,5", "3,0", "2,1", "6,3", "2,4", "1,5", "0,6", "3,3", "2,6", "5,1",
            "1,2", "5,5", "2,5", "6,5", "1,4", "0,4", "6,4", "1,1", "6,1", "1,0", "0,5", "1,6",
            "2,0",
        ]
    }

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(&input(), 12, 7, 7), 22);
    }

    #[test]
    fn test_prob2() {
        assert_eq!(prob2(&input(), 7, 7), XY { x: 6, y: 1 });
    }
}
