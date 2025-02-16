use std::collections::{HashMap, HashSet};

struct Pad {
    symbols: HashMap<char, (isize, isize)>,
    forbidden: HashSet<(isize, isize)>,
}
impl Pad {
    fn from(lines: &[&str]) -> Self {
        let mut symbols: HashMap<char, (isize, isize)> = HashMap::new();
        let mut forbidden: HashSet<(isize, isize)> = HashSet::new();
        for (i, &line) in lines.iter().enumerate() {
            for (j, chr) in line.chars().enumerate() {
                if chr != ' ' {
                    symbols.insert(chr, (i as isize, j as isize));
                } else {
                    forbidden.insert((i as isize, j as isize));
                }
            }
        }
        Self { symbols, forbidden }
    }

    fn to(&self, from: char, to: char) -> Vec<String> {
        let &current = self.symbols.get(&from).unwrap();
        let &target = self.symbols.get(&to).unwrap();
        let horizontal = vec![
            if target.1 > current.1 { '>' } else { '<' };
            (target.1 - current.1).unsigned_abs()
        ];
        let vertical = vec![
            if target.0 > current.0 { 'v' } else { '^' };
            (target.0 - current.0).unsigned_abs()
        ];
        if horizontal.is_empty() && vertical.is_empty() {
            return vec!["".to_string()];
        }
        if horizontal.is_empty() {
            return vec![vertical.iter().collect()];
        }
        if vertical.is_empty() {
            return vec![horizontal.iter().collect()];
        }
        vec![
            horizontal.iter().chain(vertical.iter()).collect(),
            vertical.iter().chain(horizontal.iter()).collect(),
        ]
    }

    fn write(&self, message: String) -> Vec<String> {
        let mut ret: Vec<String> = vec![String::from("")];
        let message = format!("A{message}");
        for (f, t) in message.chars().zip(message.chars().skip(1)) {
            ret = ret
                .iter()
                .flat_map(|a| {
                    self.to(f, t)
                        .iter()
                        .map(|b| {
                            a.chars()
                                .chain(b.chars().chain(std::iter::once('A')))
                                .collect()
                        })
                        .collect::<Vec<String>>()
                })
                .filter(|w| self.good_word(w.clone()))
                .collect();
        }
        ret
    }

    fn good_word(&self, word: String) -> bool {
        let mut pos = self.symbols[&'A'];
        for c in word.chars() {
            match c {
                '>' => pos = (pos.0, pos.1 + 1),
                '<' => pos = (pos.0, pos.1 - 1),
                'v' => pos = (pos.0 + 1, pos.1),
                '^' => pos = (pos.0 - 1, pos.1),
                _ => (),
            }
            if self.forbidden.contains(&pos) {
                return false;
            }
        }
        true
    }
}

fn pads() -> (Pad, Pad) {
    (
        Pad::from(&["789", "456", "123", " 0A"]),
        Pad::from(&[" ^A", "<v>"]),
    )
}

fn shortest_way(pad: &Pad, word: String) -> String {
    pad.write(word)
        .iter()
        .filter(|&w| pad.good_word(w.clone()))
        .min_by_key(|w| w.len())
        .unwrap()
        .to_string()
}
fn shortest_instruction(word: String) -> String {
    let (pad0, pad1) = pads();
    let words1 = pad0.write(word);
    let words2: Vec<String> = words1.iter().flat_map(|w| pad1.write(w.clone())).collect();
    let words3: Vec<String> = words2.iter().flat_map(|w| pad1.write(w.clone())).collect();
    words3.iter().min_by_key(|&w| w.len()).unwrap().clone()
}

fn complexity(word: String) -> usize {
    word.chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse()
        .unwrap()
}

fn prob1(input: &[&str]) -> usize {
    input
        .iter()
        .map(|&w| {
            let sil = shortest_instruction(w.to_string());
            let comp = complexity(w.to_string());
            println!("{w} -> {sil} * {comp}");
            sil.len() * comp
        })
        .sum()
}

fn prob2(input: &[&str]) -> usize {
    0
}

pub(crate) fn main() {
    let input: Vec<&str> = include_str!("../input/day_21").trim().split("\n").collect();
    println!("prob1: {}", prob1(&input.clone()));
    println!("prob2: {}", prob2(&input.clone()));
}

#[cfg(test)]
pub mod tests {
    use super::{prob1, prob2, shortest_instruction, Pad};

    fn input() -> Vec<&'static str> {
        vec!["029A", "980A", "179A", "456A", "379A"]
    }

    #[test]
    fn test_write() {
        let pad = Pad::from(&["789", "456", "123", " 0A"]);
        assert_eq!(pad.to('A', '2'), vec!["<^", "^<"]);
        assert_eq!(pad.to('A', '0'), vec!["<"]);
        assert_eq!(pad.write("02".to_string()), vec!["<A^A"]);
    }

    #[test]
    fn test_shortest() {
        assert_eq!(
            shortest_instruction("029A".to_string()).len(),
            "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A".len()
        );
        assert_eq!(
            shortest_instruction("179A".to_string()).len(),
            "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A".len()
        );
    }

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(&input()), 126384);
    }

    #[test]
    fn test_prob2() {
        assert_eq!(prob2(&input()), 1);
    }
}
