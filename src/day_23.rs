use std::collections::{HashMap, HashSet};

use crate::utils::DiGraph;

fn parse(input: &[&str]) -> DiGraph<String> {
    let mut nodes: HashSet<String> = HashSet::new();
    let mut edges: Vec<(String, String)> = vec![];

    for line in input {
        let lcomps: Vec<String> = line.split("-").map(|c| c.to_string()).collect();
        nodes.extend(lcomps.iter().cloned());
        edges.push((lcomps[0].clone(), lcomps[1].clone()));
        edges.push((lcomps[1].clone(), lcomps[0].clone()));
    }

    let dg: DiGraph<String> = DiGraph::from(nodes, edges);
    dg
}

impl<I> DiGraph<I> {
    fn neighbors(&self) -> HashMap<I, HashSet<I>>
    where
        I: Eq + std::hash::Hash + Clone,
    {
        let mut ret: HashMap<I, HashSet<I>> = HashMap::new();
        for (e1, e2) in self.edges.clone() {
            ret.entry(e1).or_default().insert(e2);
        }

        ret
    }
    fn triangles(&self) -> HashSet<(I, I, I)>
    where
        I: Eq + std::hash::Hash + Clone + Ord,
    {
        let mut ret: HashSet<(I, I, I)> = HashSet::new();
        let neighs = self.neighbors();
        for n in self.nodes.iter() {
            let mut nneighs: Vec<I> = neighs.get(n).unwrap().iter().cloned().collect();
            nneighs.sort();
            for i in 0..nneighs.len() {
                let ni = nneighs[i].clone();
                if ni <= *n {
                    continue;
                }
                for nj in nneighs.iter().skip(i + 1) {
                    if nj <= n {
                        continue;
                    }
                    if neighs.get(&ni).unwrap().contains(nj) {
                        ret.insert((n.clone(), ni.clone(), nj.clone()));
                        ret.insert((n.clone(), ni.clone(), nj.clone()));
                    }
                }
            }
        }
        ret
    }
}

fn prob1(input: &[&str]) -> usize {
    let dg = parse(input);
    let triangles = dg.triangles();
    triangles
        .iter()
        .filter(|(a, b, c)| a.starts_with("t") || b.starts_with("t") || c.starts_with("t"))
        .count()
}

fn prob2(input: &[&str]) -> String {
    let dg = parse(input);
    let ns = dg.neighbors();

    let mut nodes = dg.nodes.clone();
    nodes.sort();
    let nodes_id: HashMap<String, usize> = nodes
        .iter()
        .enumerate()
        .map(|(i, n)| (n.clone(), i))
        .collect();
    let neighbors: HashMap<usize, HashSet<usize>> = (0..nodes.len())
        .map(|i| (i, ns[&nodes[i]].iter().map(|n| nodes_id[n]).collect()))
        .collect();
    let mc = cliques_from(&[], nodes.len(), &neighbors, &mut vec![]);
    let mc: Vec<String> = mc.iter().map(|&n| nodes[n].clone()).collect();
    mc.join(",")
}

fn cliques_from(
    from: &[usize],
    len: usize,
    neighbors: &HashMap<usize, HashSet<usize>>,
    max_clique: &mut Vec<usize>,
) -> Vec<usize> {
    let all_nodes: HashSet<usize> = (0..len).collect();
    let max: isize = if from.is_empty() {
        -1
    } else {
        *from.last().unwrap() as isize
    };
    let mut from_neighs: Vec<usize> = from
        .iter()
        .fold(all_nodes, |inter, n| {
            inter.intersection(&neighbors[n]).copied().collect()
        })
        .iter()
        .filter(|&&t| t as isize > max)
        .copied()
        .collect();
    from_neighs.sort();
    for n in from_neighs {
        let from_plus_n: Vec<usize> = from.iter().chain(std::iter::once(&n)).copied().collect();
        if from_plus_n.len() > max_clique.len() {
            *max_clique = from_plus_n.clone();
        }
        let mcsub = cliques_from(&from_plus_n, len, neighbors, max_clique);
        if mcsub.len() > max_clique.len() {
            *max_clique = mcsub;
        }
    }
    max_clique.clone()
}

pub(crate) fn main() {
    let input: Vec<&str> = include_str!("../input/day_23").trim().split("\n").collect();
    println!("prob1: {}", prob1(&input.clone()));
    println!("prob2: {}", prob2(&input.clone()));
}

#[cfg(test)]
pub mod tests {
    use super::{parse, prob1, prob2};

    fn input() -> Vec<&'static str> {
        vec![
            "kh-tc", "qp-kh", "de-cg", "ka-co", "yn-aq", "qp-ub", "cg-tb", "vc-aq", "tb-ka",
            "wh-tc", "yn-cg", "kh-ub", "ta-co", "de-co", "tc-td", "tb-wq", "wh-td", "ta-ka",
            "td-qp", "aq-cg", "wq-ub", "ub-vc", "de-ta", "wq-aq", "wq-vc", "wh-yn", "ka-de",
            "kh-ta", "co-tc", "wh-qp", "tb-vc", "td-yn",
        ]
    }

    #[test]
    fn test_triangles() {
        let dg = parse(&input());
        let triangles = dg.triangles();
        assert_eq!(triangles.len(), 12);
    }

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(&input()), 7);
    }

    #[test]
    fn test_prob2() {
        assert_eq!(prob2(&input()), "co,de,ka,ta".to_string());
    }
}
