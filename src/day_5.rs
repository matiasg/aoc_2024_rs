use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

use regex::Regex;

struct Rules {
    order: HashMap<usize, HashSet<usize>>,
}

impl Rules {
    fn new() -> Self {
        Self {
            order: HashMap::new(),
        }
    }
    fn add(&mut self, from: usize, to: usize) {
        self.order.entry(from).or_default().insert(to);
    }
    fn is_before(&self, first: usize, second: usize) -> bool {
        self.order.contains_key(&first) && self.order.get(&first).unwrap().contains(&second)
    }
    fn correct_order(&self, update: &[usize]) -> bool {
        if update.is_empty() {
            return true;
        };
        update
            .iter()
            .zip(update.iter().skip(1))
            .all(|(&f, &s)| self.is_before(f, s))
    }
    fn compare(&self, first: usize, second: usize) -> Ordering {
        if self.is_before(first, second) {
            return Ordering::Less;
        }
        if self.is_before(second, first) {
            return Ordering::Greater;
        }
        Ordering::Equal
    }
    fn order(&self, update: &mut [usize]) -> Vec<usize> {
        update.sort_by(|&f, &s| self.compare(f, s));
        update.to_vec()
    }
}

fn make_rules_and_updates(lines: &[&str]) -> (Rules, Vec<Vec<usize>>) {
    let rulesre = Regex::new("^(\\d+)\\|(\\d+)$").unwrap();
    let mut rules = Rules::new();
    let mut updates: Vec<Vec<usize>> = vec![];

    for line in lines {
        if rulesre.is_match(line) {
            let c = rulesre.captures(line).unwrap();
            let from: usize = c.get(1).unwrap().as_str().parse().unwrap();
            let to: usize = c.get(2).unwrap().as_str().parse().unwrap();
            rules.add(from, to);
        } else if !line.is_empty() {
            updates.push(line.split(",").map(|n| n.parse().unwrap()).collect())
        }
    }
    (rules, updates)
}

fn prob1(lines: &[&str]) -> usize {
    let (rules, updates) = make_rules_and_updates(lines);
    updates
        .iter()
        .filter(|&u| rules.correct_order(u))
        .map(|u| u.get(u.len() / 2).unwrap())
        .sum()
}

fn prob2(lines: &[&str]) -> usize {
    let (rules, updates) = make_rules_and_updates(lines);
    updates
        .iter()
        .filter(|&u| !rules.correct_order(u))
        .map(|u| rules.order(&mut u.clone()))
        .map(|u| u[u.len() / 2])
        .sum()
}

pub(crate) fn main() {
    let lines: Vec<&str> = include_str!("../input/day_5").trim().split("\n").collect();
    println!("prob1: {}", prob1(&lines));
    println!("prob2: {}", prob2(&lines));
}

#[cfg(test)]
pub mod tests {
    use super::{make_rules_and_updates, prob1, prob2};

    fn input() -> Vec<&'static str> {
        vec![
            "47|53",
            "97|13",
            "97|61",
            "97|47",
            "75|29",
            "61|13",
            "75|53",
            "29|13",
            "97|29",
            "53|29",
            "61|53",
            "97|53",
            "61|29",
            "47|13",
            "75|47",
            "97|75",
            "47|61",
            "75|61",
            "47|29",
            "75|13",
            "53|13",
            "",
            "75,47,61,53,29",
            "97,61,53,29,13",
            "75,29,13",
            "75,97,47,61,53",
            "61,13,29",
            "97,13,75,29,47",
        ]
    }

    #[test]
    fn test_rules_updates() {
        let (rules, updates) = make_rules_and_updates(&input());
        assert_eq!(updates.len(), 6);
        assert_eq!(updates[2], vec![75, 29, 13]);
        assert_eq!(rules.order.len(), 6);
        assert!(rules.is_before(75, 53));
        assert!(!rules.is_before(5, 53));
        assert!(!rules.is_before(53, 75));
    }

    #[test]
    fn test_correct_order() {
        let (rules, updates) = make_rules_and_updates(&input());
        assert!(rules.correct_order(&updates[0]));
        assert!(rules.correct_order(&updates[1]));
        assert!(rules.correct_order(&updates[2]));
        assert!(!rules.correct_order(&updates[3]));
        assert!(!rules.correct_order(&updates[4]));
        assert!(!rules.correct_order(&updates[5]));
    }

    #[test]
    fn test_prob_1() {
        assert_eq!(prob1(&input()), 143);
    }

    #[test]
    fn test_prob_2() {
        assert_eq!(prob2(&input()), 123);
    }
}
