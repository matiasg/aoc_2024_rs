use std::collections::HashMap;

struct Towels<'a> {
    towels: Vec<&'a str>,
    onsets: Vec<&'a str>,
    ways: HashMap<String, usize>,
}
impl<'a> Towels<'a> {
    fn parse(lines: &[&'a str]) -> Self {
        let towels: Vec<&'a str> = lines[0].split(", ").collect();
        let onsets: Vec<&'a str> = lines[2..].to_vec();
        let ways: HashMap<String, usize> = HashMap::new();
        Self {
            towels,
            onsets,
            ways,
        }
    }

    fn leftovers(&self, word: &'a str) -> impl Iterator<Item = &str> {
        self.towels
            .iter()
            .filter(|&t| word.starts_with(t))
            .map(move |t| word.get(t.len()..).unwrap())
    }

    fn can_make(&mut self, o: String) -> bool {
        if o.is_empty() {
            return true;
        }
        if self.ways.contains_key(o.as_str()) {
            return false;
        }
        let left: Vec<String> = self.leftovers(o.as_str()).map(|l| l.to_string()).collect();
        for l in left {
            if self.can_make(l) {
                return true;
            }
        }
        self.ways.insert(o, 0);
        false
    }

    fn ways_to_make(&mut self, o: String) -> usize {
        if o.is_empty() {
            return 1;
        }
        if let Some(&ret) = self.ways.get(&o) {
            return ret;
        }
        let left: Vec<String> = self.leftovers(o.as_str()).map(|l| l.to_string()).collect();
        let mut ret = 0;
        for l in left {
            ret += self.ways_to_make(l);
        }
        self.ways.insert(o, ret);
        ret
    }
}

fn prob1(input: &[&str]) -> usize {
    let mut ts = Towels::parse(input);
    let mut ret = 0usize;
    for o in ts.onsets.clone().into_iter() {
        if ts.can_make(o.to_string()) {
            ret += 1;
        }
    }
    ret
}

fn prob2(input: &[&str]) -> usize {
    let mut ts = Towels::parse(input);
    let ts_ts: Vec<String> = ts.onsets.iter().map(|t| t.to_string()).collect();
    ts_ts.iter().map(|t| ts.ways_to_make(t.to_string())).sum()
}

pub(crate) fn main() {
    let input: Vec<&str> = include_str!("../input/day_19").trim().split("\n").collect();
    println!("prob1: {}", prob1(&input.clone()));
    println!("prob2: {}", prob2(&input.clone()));
}

#[cfg(test)]
pub mod tests {
    use super::{prob1, prob2};

    fn input() -> Vec<&'static str> {
        vec![
            "r, wr, b, g, bwu, rb, gb, br",
            "",
            "brwrr",
            "bggr",
            "gbbr",
            "rrbgbr",
            "ubwu",
            "bwurrg",
            "brgr",
            "bbrgwb",
        ]
    }

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(&input()), 6);
    }

    #[test]
    fn test_prob2() {
        assert_eq!(prob2(&input()), 16);
    }
}
