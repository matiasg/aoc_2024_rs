use std::collections::HashMap;

fn parse(input: &[&str]) -> Vec<isize> {
    input.iter().map(|l| l.parse().unwrap()).collect()
}

fn next(secret: isize) -> isize {
    let s1 = (secret ^ (secret * 0x40)) % 0x1000000;
    let s2 = (s1 ^ (s1 / 0x20)) % 0x1000000;

    (s2 ^ (s2 * 0x800)) % 0x1000000
}

fn next_nth(secret: isize, times: usize) -> isize {
    let mut secret = secret;
    for _ in 0..times {
        secret = next(secret);
    }
    secret
}

fn next_ns(secret: isize, length: usize) -> Vec<isize> {
    let mut ret: Vec<isize> = vec![secret];
    let mut s = secret;
    for _ in 0..length {
        s = next(s);
        ret.push(s);
    }
    ret
}

fn windows(secret_chain: Vec<isize>) -> impl Iterator<Item = (isize, isize, isize, isize)> {
    (3..secret_chain.len()).map(move |n| {
        (
            secret_chain[n - 3],
            secret_chain[n - 2],
            secret_chain[n - 1],
            secret_chain[n],
        )
    })
}

fn prob1(input: &[&str]) -> isize {
    let secrets = parse(input);
    secrets.iter().map(|&s| next_nth(s, 2000)).sum()
}

fn collectable(secret: isize) -> HashMap<(isize, isize, isize, isize), isize> {
    let mut m_collected: HashMap<(isize, isize, isize, isize), isize> = HashMap::new();
    let m_bananas: Vec<isize> = next_ns(secret, 2000).iter().map(|d| d % 10).collect();
    let b_diffs: Vec<isize> = m_bananas
        .iter()
        .zip(m_bananas.iter().skip(1))
        .map(|(s0, s1)| s1 - s0)
        .collect();
    for (&b, window) in m_bananas.iter().skip(4).zip(windows(b_diffs.clone())) {
        m_collected.entry(window).or_insert(b);
    }
    m_collected
}

fn prob2(input: &[&str]) -> isize {
    let secrets = parse(input);
    let mut collected: HashMap<(isize, isize, isize, isize), isize> = HashMap::new();
    for secret in secrets {
        let m_collected = collectable(secret);
        m_collected.iter().for_each(|(&w, b)| {
            *collected.entry(w).or_default() += b;
        });
    }
    *collected.values().max().unwrap()
}

pub(crate) fn main() {
    let input: Vec<&str> = include_str!("../input/day_22").trim().split("\n").collect();
    println!("prob1: {}", prob1(&input.clone()));
    println!("prob2: {}", prob2(&input.clone()));
}

#[cfg(test)]
pub mod tests {

    use super::{next, parse, prob1, prob2};

    fn input() -> Vec<&'static str> {
        vec!["1", "10", "100", "2024"]
    }

    #[test]
    fn test_next() {
        let secrets = vec![
            "123", "15887950", "16495136", "527345", "704524", "1553684", "12683156", "11100544",
            "12249484", "7753432", "5908254",
        ];
        let secrets = parse(&secrets);
        for (&s0, &s1) in secrets.iter().zip(secrets.iter().skip(1)) {
            assert_eq!(next(s0), s1);
        }
    }

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(&input()), 37327623);
    }

    #[test]
    fn test_prob2() {
        let input = vec!["1", "2", "3", "2024"];
        assert_eq!(prob2(&input), 23);
    }
}
