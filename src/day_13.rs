use regex::Regex;

#[derive(Debug)]
struct Machine {
    a: (isize, isize),
    b: (isize, isize),
    prize: (isize, isize),
}
impl Machine {
    fn from(lines: &[&str]) -> Self {
        let butreg = Regex::new("^Button (A|B): X\\+(\\d+), Y\\+(\\d+)").unwrap();
        let prizereg = Regex::new("^Prize: X=(\\d+), Y=(\\d+)").unwrap();
        let buttona = butreg.captures(lines[0]).unwrap();
        let a: (isize, isize) = (
            buttona.get(2).unwrap().as_str().parse().unwrap(),
            buttona.get(3).unwrap().as_str().parse().unwrap(),
        );
        let buttonb = butreg.captures(lines[1]).unwrap();
        let b: (isize, isize) = (
            buttonb.get(2).unwrap().as_str().parse().unwrap(),
            buttonb.get(3).unwrap().as_str().parse().unwrap(),
        );
        let prizec = prizereg.captures(lines[2]).unwrap();
        let prize: (isize, isize) = (
            prizec.get(1).unwrap().as_str().parse().unwrap(),
            prizec.get(2).unwrap().as_str().parse().unwrap(),
        );
        Self { a, b, prize }
    }

    fn det(&self) -> isize {
        self.a.0 * self.b.1 - self.a.1 * self.b.0
    }

    fn solve(&self) -> Option<(isize, isize)> {
        assert!(self.det() != 0);
        let d = self.det() as f64;
        let inv: Vec<Vec<f64>> = vec![
            vec![self.b.1 as f64, -self.a.1 as f64],
            vec![-self.b.0 as f64, self.a.0 as f64],
        ];
        let solf = (
            inv[0][0] / d * self.prize.0 as f64 + inv[1][0] / d * self.prize.1 as f64,
            inv[0][1] / d * self.prize.0 as f64 + inv[1][1] / d * self.prize.1 as f64,
        );
        let soli = (solf.0.round() as isize, solf.1.round() as isize);

        let check = (
            self.a.0 * soli.0 + self.b.0 * soli.1,
            self.a.1 * soli.0 + self.b.1 * soli.1,
        );
        if self.prize == check {
            Some(soli)
        } else {
            None
        }
    }

    fn tokens(&self) -> Option<isize> {
        self.solve().map(|(s, t)| s * 3 + t)
    }

    fn machine_prob_2(&self) -> Self {
        Self {
            a: self.a,
            b: self.b,
            prize: (self.prize.0 + 10000000000000, self.prize.1 + 10000000000000),
        }
    }
}

fn parse(input: &[&str]) -> Vec<Machine> {
    (0..input.len())
        .step_by(4)
        .map(|i| Machine::from(&input[i..(i + 3)]))
        .collect()
}

fn prob1(input: &[&str]) -> isize {
    let machines = parse(input);
    machines.iter().filter_map(|m| m.tokens()).sum()
}

fn prob2(input: &[&str]) -> isize {
    let machines = parse(input);
    machines
        .iter()
        .filter_map(|m| m.machine_prob_2().tokens())
        .sum()
}

pub(crate) fn main() {
    let input: Vec<&str> = include_str!("../input/day_13").trim().split("\n").collect();
    println!("prob1: {}", prob1(&input.clone()));
    println!("prob2: {}", prob2(&input.clone()));
}

#[cfg(test)]
pub mod tests {
    use super::{parse, prob1, Machine};

    fn input() -> Vec<&'static str> {
        vec![
            "Button A: X+94, Y+34",
            "Button B: X+22, Y+67",
            "Prize: X=8400, Y=5400",
            "",
            "Button A: X+26, Y+66",
            "Button B: X+67, Y+21",
            "Prize: X=12748, Y=12176",
            "",
            "Button A: X+17, Y+86",
            "Button B: X+84, Y+37",
            "Prize: X=7870, Y=6450",
            "",
            "Button A: X+69, Y+23",
            "Button B: X+27, Y+71",
            "Prize: X=18641, Y=10279",
        ]
    }

    #[test]
    fn test_parse() {
        let p = parse(&input());
        assert_eq!(p.len(), 4);
        assert_eq!(p[0].a, (94, 34));
        assert_eq!(p[0].b, (22, 67));
        assert_eq!(p[0].prize, (8400, 5400));
    }

    #[test]
    fn test_solve() {
        let p = parse(&input());
        assert_eq!(p[0].solve(), Some((80, 40)));
        assert_eq!(p[0].tokens(), Some(280));
        assert_eq!(p[1].solve(), None);
        assert_eq!(p[1].tokens(), None);
        assert_eq!(p[2].solve(), Some((38, 86)));
        assert_eq!(p[2].tokens(), Some(200));
    }

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(&input()), 480);
    }

    #[test]
    fn test_prob2() {
        let p: Vec<Machine> = parse(&input()).iter().map(|m| m.machine_prob_2()).collect();
        assert_eq!(
            p.iter().map(|m| m.solve().is_some()).collect::<Vec<bool>>(),
            vec![false, true, false, true]
        );
    }
}
