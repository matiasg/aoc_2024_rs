use std::collections::HashMap;

fn parse(input: &str) -> Vec<usize> {
    input.split(" ").map(|n| n.parse().unwrap()).collect()
}

fn blink(n: usize) -> Vec<usize> {
    if n == 0 {
        return vec![1];
    }
    let digits = (n as f64).log(10.).floor() as u32 + 1;
    if digits % 2 == 0 {
        let left = n / (10u32.pow(digits / 2) as usize);
        let right = n % (10u32.pow(digits / 2) as usize);
        return vec![left, right];
    }
    vec![2024 * n]
}

fn blink_on_vec(ns: Vec<usize>) -> Vec<usize> {
    ns.iter().flat_map(|&n| blink(n)).collect()
}

fn prob1(input: &str) -> usize {
    let ns = parse(input);
    blink_n_times(ns, 25)
}

fn blink_n_times(mut ns: Vec<usize>, times: usize) -> usize {
    for _ in 0..times {
        ns = blink_on_vec(ns);
    }
    ns.len()
}

fn counter<I>(ns: I) -> HashMap<I::Item, usize>
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

fn blink_n_times_map(ns: Vec<usize>, times: usize) -> usize {
    let mut nsc = counter(ns);
    for _ in 0..times {
        let mut nsca: HashMap<usize, usize> = HashMap::new();
        for (&n, nx) in nsc.iter() {
            for m in blink(n) {
                *nsca.entry(m).or_default() += nx;
            }
        }
        nsc = nsca;
    }
    nsc.values().sum()
}

fn prob2(input: &str) -> usize {
    let ns = parse(input);
    blink_n_times_map(ns, 75)
}

pub(crate) fn main() {
    let input = include_str!("../input/day_11").trim();
    println!("prob1: {}", prob1(input));
    println!("prob2: {}", prob2(input));
}

#[cfg(test)]
pub mod tests {
    use super::{blink, prob1, prob2};

    fn input() -> &'static str {
        "125 17"
    }

    #[test]
    fn test_blink() {
        assert_eq!(blink(0), vec![1]);
        assert_eq!(blink(1), vec![2024]);
        assert_eq!(blink(10), vec![1, 0]);
        assert_eq!(blink(99), vec![9, 9]);
    }

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(input()), 55312);
    }

    #[test]
    fn test_prob2() {
        assert_eq!(prob2(input()), 65601038650482); // actually, not in the statement as example
    }
}
