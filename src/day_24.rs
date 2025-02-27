use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;
use regex::Regex;

#[derive(std::hash::Hash, PartialEq, Eq, Clone, Debug)]
enum BoolOperator {
    And,
    Or,
    Xor,
}
impl BoolOperator {
    fn result(&self, first: bool, second: bool) -> bool {
        match self {
            Self::And => first && second,
            Self::Or => first || second,
            Self::Xor => first ^ second,
        }
    }
}

#[derive(std::hash::Hash, PartialEq, Eq, Clone, Debug)]
struct BoolOperation {
    first: String,
    operator: BoolOperator,
    second: String,
}

impl BoolOperation {
    fn parse(line: &str) -> Option<(Self, String)> {
        let opsreg = Regex::new("^([a-z0-9]+) (AND|OR|XOR) ([a-z0-9]+) -> ([a-z0-9]+)$").unwrap();
        if let Some(opcap) = opsreg.captures(line) {
            let first = opcap.get(1).unwrap().as_str().to_string();
            let second = opcap.get(3).unwrap().as_str().to_string();
            let operator: BoolOperator = match opcap.get(2).unwrap().as_str() {
                "AND" => BoolOperator::And,
                "OR" => BoolOperator::Or,
                "XOR" => BoolOperator::Xor,
                _ => panic!("wrong operator"),
            };
            Some((
                Self {
                    first,
                    operator,
                    second,
                },
                opcap.get(4).unwrap().as_str().to_string(),
            ))
        } else {
            None
        }
    }
    fn result(&self, values: &HashMap<String, bool>) -> bool {
        let first = values[&self.first];
        let second = values[&self.second];
        self.operator.result(first, second)
    }
}

struct BoolGraph {
    nodes: HashSet<String>,
    values: HashMap<String, bool>,
    operations: Vec<(BoolOperation, String)>,
}
impl BoolGraph {
    fn parse(input: &[&str]) -> Self {
        let valreg = Regex::new("^([a-z0-9]+): ([01])$").unwrap();
        let mut nodes: HashSet<String> = HashSet::new();
        let mut values: HashMap<String, bool> = HashMap::new();
        let mut operations: Vec<(BoolOperation, String)> = vec![];
        for line in input {
            if let Some(valgroups) = valreg.captures(line) {
                let node = valgroups.get(1).unwrap().as_str().to_string();
                nodes.insert(node.clone());
                let valu8: u8 = valgroups.get(2).unwrap().as_str().parse().unwrap();
                values.insert(node.clone(), valu8 == 1);
            } else if let Some((operation, target)) = BoolOperation::parse(line) {
                operations.push((operation.clone(), target.clone()));
                nodes.insert(target);
                nodes.insert(operation.first);
                nodes.insert(operation.second);
            }
        }

        Self {
            nodes,
            values,
            operations,
        }
    }

    fn end_value(&self) -> usize {
        value_of(&self.nodes_starting_with('z'), &self.values)
    }

    fn reset(&mut self) {
        self.values = HashMap::new();
        self.nodes
            .iter()
            .filter(|n| n.starts_with("x") || n.starts_with("y") || n.starts_with("z"))
            .for_each(|n| {
                self.values.insert(n.clone(), false);
            });
    }

    fn nodes_starting_with(&self, start: char) -> Vec<String> {
        let mut ends: Vec<String> = self
            .nodes
            .iter()
            .filter(|k| k.starts_with(start))
            .cloned()
            .collect();
        ends.sort();
        ends
    }

    fn all_children(&self, nodes: &[String]) -> HashSet<String> {
        let mut ret: HashSet<String> = HashSet::new();
        let mut next: VecDeque<String> = nodes.iter().cloned().collect();
        while let Some(n) = next.pop_front() {
            ret.insert(n.clone());
            for (op, target) in self.operations.iter() {
                // if (op.first == n && nodes.contains(&op.second))
                //     || (op.second == n && nodes.contains(&op.first))
                if (op.first == n || op.second == n) && !ret.contains(target) {
                    next.push_back(target.clone())
                }
            }
        }
        ret
    }
    fn between(&self, start: &[String], end: &[String]) -> HashSet<String> {
        self.all_children(start)
            .intersection(&self.all_parents(end))
            .cloned()
            .collect()
    }

    fn all_parents(&self, nodes: &[String]) -> HashSet<String> {
        let mut ret: HashSet<String> = HashSet::new();
        let mut next: VecDeque<String> = nodes.iter().cloned().collect();
        while let Some(n) = next.pop_front() {
            ret.insert(n.clone());
            for (op, target) in self.operations.iter() {
                if *target == n {
                    if !ret.contains(&op.first) {
                        next.push_back(op.first.clone());
                    }
                    if !ret.contains(&op.second) {
                        next.push_back(op.second.clone());
                    }
                }
            }
        }
        ret
    }
}

fn value_of(nodes_to_add: &[String], values: &HashMap<String, bool>) -> usize {
    let mut ret: usize = 0;
    let mut power: usize = 1;
    for e in nodes_to_add {
        if let Some(&v) = values.get(e) {
            ret += (if v { 1 } else { 0 }) * power;
        }
        // ret += (if values[e] { 1 } else { 0 }) * power;
        power *= 2;
    }
    ret
}

fn produce_output(bg: &mut BoolGraph) {
    let mut remaining_ops: VecDeque<(BoolOperation, String)> = bg
        .operations
        .iter()
        .map(|(o, n)| (o.clone(), n.clone()))
        .collect();
    let mut ineffective_ops = 0;
    while ineffective_ops < remaining_ops.len() {
        let (op, tgt) = remaining_ops.pop_front().unwrap();
        if bg.values.contains_key(&op.first) && bg.values.contains_key(&op.second) {
            let tgt_val = op.result(&bg.values);
            bg.values.insert(tgt.clone(), tgt_val);
            ineffective_ops = 0;
        } else {
            remaining_ops.push_back((op, tgt));
            ineffective_ops += 1;
        }
    }
}

fn prob1(input: &[&str]) -> usize {
    let mut bg = BoolGraph::parse(input);
    produce_output(&mut bg);
    bg.end_value()
}

fn level_has_problem(
    bg: &mut BoolGraph,
    xnodes: &[String],
    ynodes: &[String],
    znodes: &[String],
    level: usize,
    check_three_levels: bool,
) -> Option<usize> {
    let max_val = if check_three_levels { 31 } else { 7 };
    for v in 1..=max_val {
        bg.reset();
        if v % 2 == 1 {
            bg.values.insert(xnodes[level].clone(), true);
        }
        if v >> 1 & 1 == 1 {
            bg.values.insert(ynodes[level].clone(), true);
        }
        if level < xnodes.len() - 1 {
            if v >> 2 & 1 == 1 {
                bg.values.insert(xnodes[level + 1].clone(), true);
            }
            if v >> 3 & 1 == 1 {
                bg.values.insert(ynodes[level + 1].clone(), true);
            }
        }
        if level < xnodes.len() - 2 && v >> 4 & 1 == 1 {
            bg.values.insert(xnodes[level + 2].clone(), true);
        }
        produce_output(bg);
        let (xval, yval, zval) = (
            value_of(xnodes, &bg.values),
            value_of(ynodes, &bg.values),
            value_of(znodes, &bg.values),
        );
        if xval + yval != zval {
            return Some(v);
        }
    }
    None
}
fn first_level_not_ok(
    bg: &mut BoolGraph,
    xnodes: &[String],
    ynodes: &[String],
    znodes: &[String],
    level_check_start: usize,
    level_check_end: usize,
    max_value: usize,
) -> Option<usize> {
    for level in level_check_start..=level_check_end {
        let problems_at_level = level_has_problem(bg, xnodes, ynodes, znodes, level, false);
        if let Some(value) = problems_at_level {
            if value <= max_value {
                return Some(level);
            }
        }
    }
    None
}

fn switch(bg: &mut BoolGraph, na: String, nb: String) {
    for op_tgt in bg.operations.iter_mut() {
        if op_tgt.1 == na {
            op_tgt.1 = nb.clone();
        } else if op_tgt.1 == nb {
            op_tgt.1 = na.clone();
        }
    }
}

fn prob2(input: &[&str]) -> String {
    let mut bg = BoolGraph::parse(input);

    let mut ret: Vec<String> = vec![];
    let xnodes = bg.nodes_starting_with('x');
    let ynodes = bg.nodes_starting_with('y');
    let znodes = bg.nodes_starting_with('z');

    // heuristic highly suited for this case
    for level in 0..xnodes.len() {
        if level_has_problem(&mut bg, &xnodes, &ynodes, &znodes, level, false).is_some() {
            let level_to_fix =
                first_level_not_ok(&mut bg, &xnodes, &ynodes, &znodes, level - 1, level, 2);
            if level_to_fix.is_some() {
                let level_to_fix = level_to_fix.unwrap();
                let start = [xnodes[level_to_fix].clone(), ynodes[level_to_fix].clone()];
                let end = [
                    znodes[level_to_fix].clone(),
                    znodes[level_to_fix + 1].clone(),
                ];
                let all_bad = bg.between(&start, &end);

                for ab in all_bad.iter().combinations(2) {
                    switch(&mut bg, ab[0].clone(), ab[1].clone());
                    // check starting one level before, as carry bits might bear the problem
                    let still_problem = level_has_problem(
                        &mut bg,
                        &xnodes,
                        &ynodes,
                        &znodes,
                        level_to_fix - 1,
                        true,
                    );
                    if still_problem.is_none() {
                        ret.extend(ab.iter().copied().cloned());
                        break;
                    }
                    switch(&mut bg, ab[0].clone(), ab[1].clone());
                }
            }
        }
    }
    ret.sort();
    ret.join(",")
}

pub(crate) fn main() {
    let input: Vec<&str> = include_str!("../input/day_24").trim().split("\n").collect();
    println!("prob1: {}", prob1(&input.clone()));
    println!("prob2: {}", prob2(&input.clone()));
}

#[cfg(test)]
pub mod tests {
    use super::prob1;

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(&input()), 2024);
    }

    fn input() -> Vec<&'static str> {
        vec![
            "x00: 1",
            "x01: 0",
            "x02: 1",
            "x03: 1",
            "x04: 0",
            "y00: 1",
            "y01: 1",
            "y02: 1",
            "y03: 1",
            "y04: 1",
            "",
            "ntg XOR fgs -> mjb",
            "y02 OR x01 -> tnw",
            "kwq OR kpj -> z05",
            "x00 OR x03 -> fst",
            "tgd XOR rvg -> z01",
            "vdt OR tnw -> bfw",
            "bfw AND frj -> z10",
            "ffh OR nrd -> bqk",
            "y00 AND y03 -> djm",
            "y03 OR y00 -> psh",
            "bqk OR frj -> z08",
            "tnw OR fst -> frj",
            "gnj AND tgd -> z11",
            "bfw XOR mjb -> z00",
            "x03 OR x00 -> vdt",
            "gnj AND wpb -> z02",
            "x04 AND y00 -> kjc",
            "djm OR pbm -> qhw",
            "nrd AND vdt -> hwm",
            "kjc AND fst -> rvg",
            "y04 OR y02 -> fgs",
            "y01 AND x02 -> pbm",
            "ntg OR kjc -> kwq",
            "psh XOR fgs -> tgd",
            "qhw XOR tgd -> z09",
            "pbm OR djm -> kpj",
            "x03 XOR y03 -> ffh",
            "x00 XOR y04 -> ntg",
            "bfw OR bqk -> z06",
            "nrd XOR fgs -> wpb",
            "frj XOR qhw -> z04",
            "bqk OR frj -> z07",
            "y03 OR x01 -> nrd",
            "hwm AND bqk -> z03",
            "tgd XOR rvg -> z12",
            "tnw OR pbm -> gnj",
        ]
    }
}
