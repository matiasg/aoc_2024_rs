use regex::Regex;
use std::{collections::HashMap, fs};

fn parse_line(line: &str) -> (i32, i32) {
    let re = Regex::new("^(\\d+)\\s+(\\d+)$").unwrap();
    let numbers = re.captures(line).unwrap();
    let n1: i32 = numbers.get(1).unwrap().as_str().parse().unwrap();
    let n2: i32 = numbers.get(2).unwrap().as_str().parse().unwrap();
    (n1, n2)
}

fn split(lines: Vec<&str>) -> (Vec<i32>, Vec<i32>) {
    let mut col1: Vec<i32> = vec![];
    let mut col2: Vec<i32> = vec![];
    for l in lines {
        let (n1, n2) = parse_line(l);
        col1.push(n1);
        col2.push(n2);
    }
    (col1, col2)
}

fn do_prob_1(lines: Vec<&str>) -> i32 {
    let (mut c1, mut c2) = split(lines);
    c1.sort();
    c2.sort();
    c1.iter()
        .zip(c2.iter())
        .map(|(&n1, &n2)| (n1 - n2).abs())
        .sum()
}

fn counter<I>(it: I) -> HashMap<I::Item, isize>
where
    I: IntoIterator,
    I::Item: Eq + core::hash::Hash,
{
    let mut ret: HashMap<I::Item, isize> = HashMap::new();
    for i in it {
        ret.entry(i).and_modify(|v| *v += 1).or_insert(1);
    }
    ret
}

fn do_prob_2(lines: Vec<&str>) -> isize {
    let (c1, c2) = split(lines);
    let cc1 = counter(c1);
    let cc2 = counter(c2);
    cc1.iter()
        .map(|(k, v)| *k as isize * cc2.get(k).unwrap_or(&0) * v)
        .sum()
}

pub(crate) fn main() {
    let input = fs::read_to_string("input/day_1").expect("no input file");
    let input: Vec<&str> = input.trim().split("\n").collect();
    println!("result: {}", do_prob_1(input.clone()));
    println!("result: {}", do_prob_2(input.clone()));
}

#[cfg(test)]
pub mod tests {
    use crate::day_1::{do_prob_1, do_prob_2};

    fn input() -> Vec<&'static str> {
        vec!["3   4", "4   3", "2   5", "1   3", "3   9", "3   3"]
    }

    #[test]
    fn test() {
        let r = do_prob_1(input());
        assert_eq!(r, 11);
    }

    #[test]
    fn test_p2() {
        assert_eq!(do_prob_2(input()), 31);
    }
}
