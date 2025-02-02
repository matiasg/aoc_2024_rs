use num::{Bounded, Zero};
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
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

pub(crate) struct DiGraph<I> {
    pub(crate) nodes: Vec<I>,
    pub(crate) edges: Vec<(I, I)>,
}

impl<I> DiGraph<I> {
    pub(crate) fn from<II, EE>(nodes: II, edges: EE) -> DiGraph<I>
    where
        II: IntoIterator<Item = I>,
        EE: IntoIterator<Item = (I, I)>,
    {
        Self {
            nodes: nodes.into_iter().collect(),
            edges: edges.into_iter().collect(),
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.nodes.len()
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
    fn distance(&self, start: I, end: I) -> Option<usize> {
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
    pub(crate) fn distance_with<II, V>(
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
}

#[cfg(test)]
pub mod tests {
    use std::collections::HashMap;

    use super::DiGraph;

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
}
