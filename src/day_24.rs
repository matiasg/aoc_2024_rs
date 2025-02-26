use std::collections::{HashMap, HashSet, VecDeque};

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
    end_nodes: HashSet<String>,
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

        let end_nodes: HashSet<String> = nodes
            .iter()
            .filter(|&n| n.starts_with("z"))
            .cloned()
            .collect();

        Self {
            nodes,
            end_nodes,
            values,
            operations,
        }
    }

    fn end_value(&self) -> usize {
        value_of(&self.nodes_starting_with('z'), &self.values)
    }

    fn reset(&mut self) {
        self.values = HashMap::new();
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
                if op.first == n || op.second == n {
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
                    next.push_back(op.first.clone());
                    next.push_back(op.second.clone());
                }
            }
        }
        ret
    }
}

fn value_of(nodes_to_add: &Vec<String>, values: &HashMap<String, bool>) -> usize {
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

fn prob2(input: &[&str]) -> usize {
    let mut bg = BoolGraph::parse(input);
    let xnodes = bg.nodes_starting_with('x');
    let ynodes = bg.nodes_starting_with('y');
    let znodes = bg.nodes_starting_with('z');
    let mut all_bad: HashSet<String> = HashSet::new();
    for xp in 0..8 {
        bg.reset();
        xnodes.iter().chain(ynodes.iter()).for_each(|n| {
            bg.values.insert(n.clone(), false);
        });
        bg.values.insert(xnodes[xp].clone(), true);
        produce_output(&mut bg);
        let (xval, yval, zval) = (
            value_of(&xnodes, &bg.values),
            value_of(&ynodes, &bg.values),
            value_of(&znodes, &bg.values),
        );
        if xval + yval != zval {
            // let children = bg.all_children(vec![xnodes[xp].clone()]);
            let zwrong: HashSet<String> = bg
                .nodes
                .iter()
                .filter(|n| n.starts_with('z') && bg.values[n.clone()])
                .chain(std::iter::once(&znodes[xp]))
                .cloned()
                .collect();
            let zwrong: Vec<String> = zwrong.iter().cloned().collect();
            let children = bg.between(&[xnodes[xp].clone()], &zwrong);
            println!(
                "{xval} + {yval} = {zval} != {} => {:?}",
                xval + yval,
                children
            );
            all_bad.extend(children.iter().cloned());
        }
        if all_bad.len() > 10 {
            break;
        }
    }
    let mut all_bad: Vec<String> = all_bad.iter().cloned().collect();
    all_bad.sort();
    println!("all bad: {:?}", all_bad);
    0
}

pub(crate) fn main() {
    let input: Vec<&str> = include_str!("../input/day_24").trim().split("\n").collect();
    println!("prob1: {}", prob1(&input.clone()));
    println!("prob2: {}", prob2(&input.clone()));
}

#[cfg(test)]
pub mod tests {
    use super::{prob1, prob2};

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(&input()), 2024);
    }

    #[test]
    fn test_prob2() {
        prob2(&input());
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
