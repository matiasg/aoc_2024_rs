use regex::Regex;

#[derive(Clone)]
struct Machine {
    rega: isize,
    regb: isize,
    regc: isize,
    program: Vec<(isize, isize)>,
    execptr: usize,
    output: Vec<isize>,
}

impl Machine {
    fn _getreg(line: &str) -> isize {
        let regreg = Regex::new("^Register ([ABC]): (\\d+)$").unwrap();
        regreg
            .captures(line)
            .unwrap()
            .get(2)
            .unwrap()
            .as_str()
            .parse()
            .unwrap()
    }

    fn parse(input: &[&str]) -> Self {
        let rega = Self::_getreg(input[0]);
        let regb = Self::_getreg(input[1]);
        let regc = Self::_getreg(input[2]);
        let inst_args: Vec<isize> = input[4]
            .split(":")
            .last()
            .unwrap()
            .trim()
            .split(",")
            .map(|i| i.parse().unwrap())
            .collect();
        let program: Vec<(isize, isize)> = inst_args
            .iter()
            .step_by(2)
            .copied()
            .zip(inst_args.iter().skip(1).step_by(2).copied())
            .collect();
        Self {
            rega,
            regb,
            regc,
            program,
            execptr: 0,
            output: vec![],
        }
    }

    fn stopped(&self) -> bool {
        self.execptr >= self.program.len()
    }

    // a % 8 ^ 3 ^ 5 ^ c
    // c -> a / 2**(a % 8 ^ 3)

    fn exec(&mut self) {
        let (ins, arg) = self.program[self.execptr];
        match ins {
            0 => self.rega /= 2isize.pow(self.combo(arg) as u32),
            1 => self.regb ^= arg,
            2 => self.regb = self.combo(arg) % 8,
            3 => {
                if self.rega != 0 {
                    self.execptr = arg as usize;
                } else {
                    self.execptr += 1;
                }
            }
            4 => self.regb ^= self.regc,
            5 => self.output.push(self.combo(arg) % 8),
            6 => self.regb = self.rega / 2isize.pow(self.combo(arg) as u32),
            7 => self.regc = self.rega / 2isize.pow(self.combo(arg) as u32),
            _ => (),
        }
        if ins != 3 {
            self.execptr += 1;
        }
    }

    fn combo(&self, arg: isize) -> isize {
        if arg <= 3 {
            return arg;
        }
        match arg {
            4 => self.rega,
            5 => self.regb,
            6 => self.regc,
            _ => panic!("combo arg not understood {arg}"),
        }
    }
}

fn prob1(input: &[&str]) -> String {
    let mut m = Machine::parse(input);
    while !m.stopped() {
        m.exec();
    }
    m.output
        .iter()
        .map(|n| n.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

fn b8_isize(ns: &[isize]) -> isize {
    ns.iter()
        .rev()
        .enumerate()
        .map(|(k, n)| n * 8isize.pow(k as u32))
        .sum()
}

fn check(rega: Vec<isize>, m0: &Machine, expected: &[isize]) -> bool {
    let mut m = m0.clone();
    let len = m0.program.len() * 2;
    let nrega: Vec<isize> = rega
        .iter()
        .chain(std::iter::repeat_n(&0isize, len - rega.len()))
        .copied()
        .collect();
    m.rega = b8_isize(&nrega);
    while !m.stopped() {
        m.exec();
    }
    m.output[len - rega.len()] == expected[len - rega.len()]
}

fn tree_check(rega: Vec<isize>, m0: &Machine, expected: &[isize]) -> Option<Vec<isize>> {
    if rega.len() == expected.len() {
        return Some(rega);
    }
    let mut nrega: Vec<isize> = rega.iter().chain(std::iter::once(&0)).copied().collect();
    for val in 0..8 {
        if rega.is_empty() && val == 0 {
            continue;
        }
        nrega[rega.len()] = val;
        if check(nrega.clone(), m0, expected) {
            if let Some(cont) = tree_check(nrega.clone(), m0, expected) {
                return Some(cont);
            }
        }
    }
    None
}

fn prob2(input: &[&str]) -> isize {
    let m0 = Machine::parse(input);
    let expected: Vec<isize> = m0
        .program
        .iter()
        .flat_map(|&(i, a)| [i, a].into_iter())
        .collect();
    if let Some(ret) = tree_check(vec![], &m0, &expected) {
        return b8_isize(&ret);
    }
    panic!("No value found");
}

pub(crate) fn main() {
    let input: Vec<&str> = include_str!("../input/day_17").trim().split("\n").collect();
    println!("prob1: {}", prob1(&input.clone()));
    println!("prob2: {}", prob2(&input.clone()));
}

#[cfg(test)]
pub mod tests {
    use super::{b8_isize, prob1, prob2, Machine};

    fn input() -> Vec<&'static str> {
        vec![
            "Register A: 729",
            "Register B: 0",
            "Register C: 0",
            "",
            "Program: 0,1,5,4,3,0",
        ]
    }

    fn input2() -> Vec<&'static str> {
        vec![
            "Register A: 2024",
            "Register B: 0",
            "Register C: 0",
            "",
            "Program: 0,3,5,4,3,0",
        ]
    }

    #[test]
    fn test_machine() {
        let m = Machine::parse(&input());
        assert_eq!(m.rega, 729);
        assert_eq!(m.regb, 0);
        assert_eq!(m.regc, 0);
        assert_eq!(m.program, vec![(0, 1), (5, 4), (3, 0)]);
        assert_eq!(m.execptr, 0);
        assert!(!m.stopped());
    }

    #[test]
    fn test_exec() {
        let mut m = Machine {
            rega: 9,
            regb: 7,
            regc: 0,
            program: vec![(1, 5), (0, 5)],
            execptr: 0,
            output: vec![],
        };
        m.exec();
        assert_eq!(m.regb, 2);
        assert_eq!(m.execptr, 1);
        m.exec();
        assert_eq!(m.rega, 2);
        assert_eq!(m.regb, 2);
        assert_eq!(m.execptr, 2);
        assert!(m.stopped());
    }

    #[test]
    fn test_b8() {
        assert_eq!(b8_isize(&[1, 3, 0]), 64 + 3 * 8);
    }

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(&input()), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn test_prob2() {
        assert_eq!(prob2(&input2()), 117440);
    }
}
