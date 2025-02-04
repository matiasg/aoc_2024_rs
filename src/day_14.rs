use regex::Regex;

use crate::utils::counter;

struct PV {
    pos: (isize, isize),
    vel: (isize, isize),
}
impl PV {
    fn from(line: &str) -> Self {
        let re = Regex::new("^p=([0-9-]+),([0-9-]+) v=([0-9-]+),([0-9-]+)$").unwrap();
        let cap = re.captures(line).unwrap();
        let pos: (isize, isize) = (
            cap.get(1).unwrap().as_str().parse().unwrap(),
            cap.get(2).unwrap().as_str().parse().unwrap(),
        );
        let vel: (isize, isize) = (
            cap.get(3).unwrap().as_str().parse().unwrap(),
            cap.get(4).unwrap().as_str().parse().unwrap(),
        );
        Self { pos, vel }
    }
    fn pos_after(&self, time: isize, width: isize, height: isize) -> (isize, isize) {
        shift_into(
            (
                self.pos.0 + time * self.vel.0,
                self.pos.1 + time * self.vel.1,
            ),
            width,
            height,
        )
    }
}
fn parse(lines: &[&str]) -> Vec<PV> {
    lines.iter().map(|l| PV::from(l)).collect()
}
fn shift_into(p: (isize, isize), width: isize, height: isize) -> (isize, isize) {
    (
        (p.0 % width + width) % width,
        (p.1 % height + height) % height,
    )
}

fn prob1(input: &[&str], width: isize, height: isize) -> usize {
    let pvs = parse(input);
    let pos_after: Vec<(isize, isize)> = pvs
        .iter()
        .map(|pv| pv.pos_after(100, width, height))
        .collect();
    let quadrants = counter(pos_after.iter().filter_map(|p| quadrant(p, width, height)));
    quadrants.values().product()
}

fn quadrant(p: &(isize, isize), width: isize, height: isize) -> Option<isize> {
    let mshor = (p.0 - (width - 1) / 2).signum();
    let msver = (p.1 - (height - 1) / 2).signum();
    if mshor * msver != 0 {
        Some(mshor + 1 + (msver + 1) / 2)
    } else {
        None
    }
}

fn display(ps: &[(isize, isize)], width: isize, height: isize) {
    let mut d: Vec<String> = vec![String::from_iter(vec![' '; width as usize]); height as usize];
    for p in ps {
        d[p.1 as usize].replace_range((p.0 as usize)..(p.0 as usize + 1), "*");
    }
    println!("{}", String::from_iter(vec!['='; width as usize]));
    d.iter().for_each(|l| println!("{l}"));
    println!("{}", String::from_iter(vec!['='; width as usize]));
}

fn prob2(input: &[&str], width: isize, height: isize) -> isize {
    // This is just many attempts of trial and error
    let pvs = parse(input);
    for t in 0..10000 {
        let pos_after: Vec<(isize, isize)> = pvs
            .iter()
            .map(|pv| pv.pos_after(t, width, height))
            .collect();

        let cps = counter(pos_after.iter().map(|(x, _)| x));
        let mxh = *cps.values().max().unwrap();

        let cps = counter(pos_after.iter().map(|(_, y)| y));
        let myh = *cps.values().max().unwrap();

        if mxh > 23 && myh > 20 {
            display(&pos_after, width, height);
            return t;
        }
    }
    0
}

pub(crate) fn main() {
    let input: Vec<&str> = include_str!("../input/day_14").trim().split("\n").collect();
    println!("prob1: {}", prob1(&input.clone(), 101, 103));
    println!("prob2: {}", prob2(&input.clone(), 101, 103));
}

#[cfg(test)]
pub mod tests {
    use crate::day_14::prob1;

    use super::{display, parse};

    fn input() -> Vec<&'static str> {
        vec![
            "p=0,4 v=3,-3",
            "p=6,3 v=-1,-3",
            "p=10,3 v=-1,2",
            "p=2,0 v=2,-1",
            "p=0,0 v=1,3",
            "p=3,0 v=-2,-2",
            "p=7,6 v=-1,-3",
            "p=3,0 v=-1,-2",
            "p=9,3 v=2,3",
            "p=7,3 v=-1,2",
            "p=2,4 v=2,-3",
            "p=9,5 v=-3,-3",
        ]
    }

    #[test]
    fn test_parse() {
        let pvs = parse(&input());
        assert_eq!(pvs.len(), 12);
        assert_eq!(pvs[1].pos, (6, 3));
        assert_eq!(pvs[1].vel, (-1, -3));
        assert_eq!(pvs[2].pos, (10, 3));
        assert_eq!(pvs[2].vel, (-1, 2));
    }

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(&input(), 11, 7), 12);
    }

    #[test]
    fn test_prob2() {
        let pvs = parse(&input());
        for t in 0..40 {
            let ps: Vec<(isize, isize)> = pvs.iter().map(|pv| pv.pos_after(t, 11, 7)).collect();
            display(&ps, 11, 7);
        }
    }
}
