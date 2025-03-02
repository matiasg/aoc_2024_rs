use itertools::iproduct;

struct KeyOrLock {
    is_key: bool,
    combination: [u8; 5],
}
impl KeyOrLock {
    fn parse(input: &[&str]) -> Self {
        let is_lock = input[0].starts_with('#');
        let mut counter: [u8; 5] = [0; 5];
        for line in input.iter() {
            for (i, c) in line.chars().enumerate() {
                if c == '#' {
                    counter[i] += 1;
                }
            }
        }
        counter.iter_mut().for_each(|v| *v -= 1);
        Self {
            is_key: !is_lock,
            combination: counter,
        }
    }

    fn matches(&self, other: &KeyOrLock) -> bool {
        self.combination
            .iter()
            .zip(other.combination.iter())
            .all(|(l, k)| l + k <= 5)
    }
}

fn parse(input: &[&str]) -> (Vec<KeyOrLock>, Vec<KeyOrLock>) {
    let mut keys: Vec<KeyOrLock> = vec![];
    let mut locks: Vec<KeyOrLock> = vec![];
    for batch in input.chunk_by(|&a, &b| a.len() == b.len()) {
        if batch.len() < 2 {
            continue;
        }
        let kl = KeyOrLock::parse(batch);
        if kl.is_key {
            keys.push(kl)
        } else {
            locks.push(kl);
        }
    }
    (keys, locks)
}

fn prob1(input: &[&str]) -> usize {
    let (keys, locks) = parse(input);
    iproduct![keys.iter(), locks.iter()]
        .filter(|(k, l)| k.matches(l))
        .count()
}

pub(crate) fn main() {
    let input: Vec<&str> = include_str!("../input/day_25").trim().split("\n").collect();
    println!("prob1: {}", prob1(&input.clone()));
    println!("prob2: no prob2!");
}

#[cfg(test)]
pub mod tests {
    use super::prob1;

    fn input() -> Vec<&'static str> {
        vec![
            "#####", ".####", ".####", ".####", ".#.#.", ".#...", ".....", "", "#####", "##.##",
            ".#.##", "...##", "...#.", "...#.", ".....", "", ".....", "#....", "#....", "#...#",
            "#.#.#", "#.###", "#####", "", ".....", ".....", "#.#..", "###..", "###.#", "###.#",
            "#####", "", ".....", ".....", ".....", "#....", "#.#..", "#.#.#", "#####",
        ]
    }

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(&input()), 3);
    }
}
