use num::{Bounded, Zero};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    fmt,
    ops::Add,
};

pub fn counter<I>(ns: I) -> HashMap<I::Item, usize>
where
    I: IntoIterator,
    I::Item: Eq + core::hash::Hash,
{
    let mut ret: HashMap<I::Item, usize> = HashMap::new();
    for n in ns {
        *ret.entry(n).or_default() += 1;
    }
    ret
}

pub struct DiGraph<I> {
    pub nodes: Vec<I>,
    pub edges: Vec<(I, I)>,
}

impl<I> DiGraph<I> {
    pub fn from<II, EE>(nodes: II, edges: EE) -> DiGraph<I>
    where
        II: IntoIterator<Item = I>,
        EE: IntoIterator<Item = (I, I)>,
    {
        Self {
            nodes: nodes.into_iter().collect(),
            edges: edges.into_iter().collect(),
        }
    }

    pub fn len(&self) -> usize {
        self.nodes.len()
    }

    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    fn children(&self, from: I) -> HashSet<I>
    where
        I: Eq + core::hash::Hash + Copy,
    {
        self.edges
            .iter()
            .filter(|&(f, _)| f == &from)
            .map(|&(_, t)| t)
            .collect()
    }
}

impl<I> DiGraph<I>
where
    I: Eq + core::hash::Hash + Copy,
{
    pub fn distance(&self, start: I, end: I) -> Option<usize> {
        if start == end {
            return Some(0);
        }
        let mut visited: HashMap<I, usize> = HashMap::new();
        let mut last_visited: HashSet<I> = HashSet::from([start]);
        let mut distance = 0usize;
        while !last_visited.is_empty() {
            let mut next_visited: HashSet<I> = last_visited
                .iter()
                .flat_map(|&n| self.children(n))
                .collect();
            if next_visited.contains(&end) {
                return Some(distance + 1);
            }
            next_visited = next_visited
                .difference(&visited.keys().copied().collect())
                .copied()
                .collect();
            last_visited.iter().for_each(|&n| {
                visited.entry(n).or_insert(distance);
            });
            last_visited = next_visited;
            distance += 1;
        }
        None
    }

    /// disregards the nodes
    pub fn distance_with<II, V>(
        &self,
        start: I,
        ends: II,
        weights: HashMap<(I, I), V>,
    ) -> HashMap<I, V>
    where
        II: IntoIterator<Item = I>,
        I: std::fmt::Debug + Ord,
        V: Add + Bounded + Zero + Ord + Copy + std::fmt::Debug,
    {
        // optimization: get children in weights fast
        let mut wchildren: HashMap<I, HashSet<I>> = HashMap::new();
        for (f, t) in weights.keys().copied() {
            wchildren.entry(f).or_default().insert(t);
        }
        let ends: HashSet<I> = ends.into_iter().collect();
        let mut ret: HashMap<I, V> = HashMap::new();
        let infinity = Reverse(V::max_value());
        let mut unvisited: BinaryHeap<(Reverse<V>, I)> =
            self.nodes.iter().map(|&n| (infinity, n)).collect();
        let mut visited: HashSet<I> = HashSet::new();
        unvisited.push((Reverse(V::zero()), start));
        while !unvisited.is_empty() && unvisited.peek().unwrap().0 > infinity {
            let (rdist, node) = unvisited.pop().unwrap();
            if visited.contains(&node) {
                continue;
            }
            let Reverse(dist) = rdist;
            if ends.contains(&node) {
                ret.insert(node, dist);
                if ret.len() == ends.len() {
                    break;
                }
            }
            for &child in wchildren.entry(node).or_default().iter() {
                let &ndist = weights.get(&(node, child)).unwrap();
                unvisited.push((Reverse(dist + ndist), child));
            }
            visited.insert(node);
        }
        ret
    }

    pub fn all_distances(&self) -> Distances<I> {
        let num: HashMap<I, usize> = self
            .nodes
            .iter()
            .enumerate()
            .map(|(i, &ij)| (ij, i))
            .collect();
        let mut ret: Vec<Vec<usize>> = vec![vec![usize::MAX; self.len()]; self.len()];
        for a in 0..self.len() {
            ret[a][a] = 0;
        }
        for &(a, b) in self.edges.iter() {
            ret[num[&a]][num[&b]] = 1;
        }
        for b in 0..ret.len() {
            for a in 0..ret.len() {
                let dab = ret[a][b];
                if dab == usize::MAX {
                    continue;
                }
                for c in 0..ret.len() {
                    let dbc = ret[b][c];
                    if dbc == usize::MAX {
                        continue;
                    }
                    ret[a][c] = (ret[a][c]).min(dab + dbc);
                }
            }
        }
        Distances { num, dist: ret }
        // let mut rett: HashMap<(I, I), usize> = HashMap::new();
        // for a in self.nodes.iter() {
        //     for b in self.nodes.iter() {
        //         let d = ret[num[a]][num[b]];
        //         rett.insert((*a, *b), d);
        //     }
        // }
        // rett
    }
}

pub struct Distances<I> {
    num: HashMap<I, usize>,
    dist: Vec<Vec<usize>>,
}
impl<I> Distances<I>
where
    I: Eq + std::hash::Hash,
{
    pub fn get(&self, a: &I, b: &I) -> Option<usize> {
        let r = self.dist[self.num[a]][self.num[b]];
        if r < usize::MAX {
            Some(r)
        } else {
            None
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord, std::hash::Hash)]
pub struct IJ {
    pub i: isize,
    pub j: isize,
}
impl IJ {
    fn from_usize(i: usize, j: usize) -> Self {
        Self {
            i: i as isize,
            j: j as isize,
        }
    }
    fn left(&self) -> Self {
        Self {
            i: self.i,
            j: self.j - 1,
        }
    }
    fn up(&self) -> Self {
        Self {
            i: self.i - 1,
            j: self.j,
        }
    }
    fn right(&self) -> Self {
        Self {
            i: self.i,
            j: self.j + 1,
        }
    }
    fn down(&self) -> Self {
        Self {
            i: self.i + 1,
            j: self.j,
        }
    }
    pub fn distance(&self, other: &IJ) -> isize {
        (self.i - other.i).abs() + (self.j - other.j).abs()
    }
    pub fn closer_than(&self, max_dist: isize) -> impl Iterator<Item = IJ> + use<'_> {
        (-max_dist..=max_dist).flat_map(move |i| {
            let max_dist_j = max_dist - i.abs();
            (-max_dist_j..=max_dist_j).map(move |j| IJ {
                i: i + self.i,
                j: j + self.j,
            })
        })
    }
    pub fn inside(&self, min_ij: IJ, max_ij: IJ) -> bool {
        min_ij.i <= self.i && self.i < max_ij.i && min_ij.j <= self.j && self.j < max_ij.j
    }
    pub fn closer_than_and_bounded(
        &self,
        max_dist: isize,
        min_ij: IJ,
        max_ij: IJ,
    ) -> impl Iterator<Item = IJ> + use<'_> {
        self.closer_than(max_dist)
            .filter(move |ij| ij.inside(min_ij, max_ij))
    }

    pub fn neighbors(&self) -> [IJ; 4] {
        [self.left(), self.up(), self.right(), self.down()]
    }
}
impl fmt::Display for IJ {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.i, self.j)
    }
}

pub struct Maze {
    pub width: isize,
    pub height: isize,
    pub wall: Vec<IJ>,
    pub start: IJ,
    pub end: IJ,
}

impl Maze {
    pub fn from(input: &[String]) -> Self {
        let width = input[0].len() as isize;
        let height = input.len() as isize;
        let mut wall: Vec<IJ> = vec![];
        let mut start = IJ { i: 0, j: 0 };
        let mut end = IJ { i: 0, j: 0 };
        for (i, line) in input.iter().enumerate() {
            for (j, chr) in line.chars().enumerate() {
                match chr {
                    '#' => wall.push(IJ::from_usize(i, j)),
                    'S' => start = IJ::from_usize(i, j),
                    'E' => end = IJ::from_usize(i, j),
                    _ => (),
                }
            }
        }
        Self {
            width,
            height,
            wall,
            start,
            end,
        }
    }

    fn available(&self, ij: IJ) -> bool {
        0 <= ij.i
            && ij.i < self.height
            && 0 <= ij.j
            && ij.j < self.width
            && !self.wall.contains(&ij)
    }

    pub fn neighbors(&self, ij: IJ) -> Vec<IJ> {
        ij.neighbors()
            .iter()
            .filter(|&&ij| self.available(ij))
            .copied()
            .collect()
    }

    /// distance from start to end
    /// This is Dijkstra again, but in some applications it is better to just
    /// have the maze from this info than building the (Di)Graph
    pub fn distance(&self) -> Option<usize> {
        let mut visited: HashSet<IJ> = HashSet::new();
        let mut next: BinaryHeap<(Reverse<usize>, IJ)> =
            BinaryHeap::from([(Reverse(0), self.start)]);
        while let Some((Reverse(d), ij)) = next.pop() {
            if ij == self.end {
                return Some(d);
            }
            if visited.contains(&ij) {
                continue;
            }
            self.neighbors(ij)
                .iter()
                .filter(|&n| !visited.contains(n))
                .for_each(|&n| next.push((Reverse(d + 1), n)));
            visited.insert(ij);
        }
        None
    }

    pub fn as_digraph(&self) -> DiGraph<IJ> {
        let nodes: HashSet<IJ> = (0..self.height)
            .flat_map(|i| (0..self.width).map(move |j| IJ { i, j }))
            .collect();
        let nodes: Vec<IJ> = nodes
            .difference(&self.wall.iter().copied().collect())
            .copied()
            .collect();
        let edges: Vec<(IJ, IJ)> = nodes
            .iter()
            .flat_map(|&ij| {
                self.neighbors(ij)
                    .iter()
                    .map(move |&kl| (ij, kl))
                    .collect::<Vec<_>>()
            })
            .collect();
        DiGraph { nodes, edges }
    }
}

#[cfg(test)]
pub mod tests {
    use std::collections::{HashMap, HashSet};

    use crate::utils::IJ;

    use super::{DiGraph, Maze};

    #[test]
    fn test_ij() {
        let ij = IJ { i: 2, j: 3 };
        assert_eq!(ij.closer_than(1).collect::<Vec<IJ>>().len(), 5);
        assert_eq!(ij.closer_than(2).collect::<Vec<IJ>>().len(), 13);
        assert_eq!(ij.closer_than(3).collect::<Vec<IJ>>().len(), 25);
        assert_eq!(
            ij.closer_than_and_bounded(3, IJ { i: 0, j: 0 }, IJ { i: 3, j: 4 })
                .collect::<HashSet<IJ>>(),
            HashSet::from([
                IJ::from_usize(0, 2),
                IJ::from_usize(0, 3),
                IJ::from_usize(1, 1),
                IJ::from_usize(1, 2),
                IJ::from_usize(1, 3),
                IJ::from_usize(2, 0),
                IJ::from_usize(2, 1),
                IJ::from_usize(2, 2),
                IJ::from_usize(2, 3),
            ])
        );
    }

    #[test]
    fn test_graph() {
        let g: DiGraph<u8> = DiGraph::from([1, 2, 3], [(1, 2), (2, 3)]);
        assert_eq!(g.len(), 3);
        assert_eq!(g.edges.len(), 2);
        assert_eq!(g.distance(1, 1), Some(0));
        assert_eq!(g.distance(1, 2), Some(1));
        assert_eq!(g.distance(1, 3), Some(2));
        assert_eq!(g.distance(3, 1), None);
        assert_eq!(g.distance(2, 1), None);

        let g: DiGraph<u8> = DiGraph::from([1, 2, 3], [(1, 2), (2, 3), (1, 3)]);
        assert_eq!(g.distance(1, 3), Some(1));
    }

    #[test]
    fn test_all_distances() {
        let g: DiGraph<u8> = DiGraph::from([1, 2, 3], [(1, 2), (2, 3)]);
        let ds = g.all_distances();
        assert_eq!(ds.get(&1, &1), Some(0));
        assert_eq!(ds.get(&1, &2), Some(1));
        assert_eq!(ds.get(&1, &3), Some(2));
        assert_eq!(ds.get(&3, &1), None);
    }

    #[test]
    fn test_distances() {
        let g: DiGraph<u8> = DiGraph::from([1, 2, 3], []);
        let weights: HashMap<(u8, u8), usize> = HashMap::from([((1, 2), 1), ((2, 3), 4)]);
        let ds = g.distance_with(1, [2, 3], weights);
        assert_eq!(ds, HashMap::from([(2, 1), (3, 5)]));

        let weights: HashMap<(u8, u8), usize> =
            HashMap::from([((1, 3), 8), ((1, 2), 1), ((2, 3), 1)]);
        let ds = g.distance_with(1, [3], weights);
        assert_eq!(ds, HashMap::from([(3, 2)]));

        let weights: HashMap<(u8, u8), usize> = HashMap::from([((1, 2), 8)]);
        let ds = g.distance_with(1, [3], weights);
        assert_eq!(ds, HashMap::from([]));

        let weights: HashMap<(u8, u8), usize> = HashMap::from([((1, 2), 8)]);
        let ds = g.distance_with(1, [2, 3], weights);
        assert_eq!(ds, HashMap::from([(2, 8)]));
    }

    #[test]
    fn test_maze() {
        let m = Maze::from(&[
            "######".to_string(),
            "#.S..#".to_string(),
            "#.####".to_string(),
            "#.. E#".to_string(),
            "######".to_string(),
        ]);
        assert_eq!(m.height, 5);
        assert_eq!(m.width, 6);
        assert_eq!(m.start, IJ { i: 1, j: 2 });
    }

    #[test]
    fn test_maze_distance() {
        let mut mz: Vec<String> = vec![
            "##########".to_string(),
            "# S      #".to_string(),
            "######## #".to_string(),
            "#   E    #".to_string(),
            "##########".to_string(),
        ];
        let m = Maze::from(&mz);
        assert_eq!(m.distance(), Some(12));

        mz[2].replace_range(8..9, "#");
        let m = Maze::from(&mz);
        assert_eq!(m.distance(), None);

        mz[2].replace_range(3..4, " ");
        let m = Maze::from(&mz);
        assert_eq!(m.distance(), Some(4));
    }
}
