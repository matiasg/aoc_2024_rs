fn parse(lines: &[&str]) -> Vec<(i64, Vec<i64>)> {
    let mut ret = vec![];
    for line in lines {
        let mut res_nrs = line.split(": ");
        let res: i64 = res_nrs.next().unwrap().parse().unwrap();
        let nrs: Vec<i64> = res_nrs
            .next()
            .unwrap()
            .split(" ")
            .map(|n| n.parse().unwrap())
            .collect();
        ret.push((res, nrs));
    }
    ret
}

fn expressable(eq: (i64, Vec<i64>)) -> bool {
    let (res, nrs) = eq;
    for ops in 0..(1 << (nrs.len() - 1)) {
        if apply(ops, &nrs) == res {
            return true;
        }
    }
    false
}

fn apply(mut ops: i32, nrs: &[i64]) -> i64 {
    let mut res = nrs[0];
    for n in nrs[1..].iter() {
        if ops & 1 == 0 {
            res += n;
        } else {
            res *= n;
        }
        ops >>= 1;
    }
    res
}

fn expressable2(eq: (i64, Vec<i64>)) -> bool {
    let (res, nrs) = eq;
    for ops in 0..(3i32.pow(nrs.len() as u32 - 1)) {
        if apply2(ops, &nrs) == res {
            return true;
        }
    }
    false
}
fn concat(a: i64, b: i64) -> i64 {
    let l = (b as f32).log10().floor() as u32;
    a * 10i64.pow(l + 1) + b
}

fn apply2(mut ops: i32, nrs: &[i64]) -> i64 {
    let mut res = nrs[0];
    for n in nrs[1..].iter() {
        res = match ops % 3 {
            0 => res + n,
            1 => res * n,
            2 => concat(res, *n),
            _ => panic!("oh, boy"),
        };
        ops /= 3;
    }
    res
}

fn prob1(lines: &[&str]) -> i64 {
    let eqs = parse(lines);
    eqs.iter()
        .filter(|&eq| expressable(eq.clone()))
        .map(|(r, _)| r)
        .sum()
}

fn prob2(lines: &[&str]) -> i64 {
    let eqs = parse(lines);
    eqs.iter()
        .filter(|&eq| expressable2(eq.clone()))
        .map(|(r, _)| r)
        .sum()
}

pub(crate) fn main() {
    let lines: Vec<&str> = include_str!("../input/day_7").trim().split("\n").collect();
    println!("prob1: {}", prob1(&lines));
    println!("prob2: {}", prob2(&lines));
}

#[cfg(test)]
pub mod tests {
    use super::{apply, apply2, expressable, expressable2, parse, prob1, prob2};

    fn input() -> Vec<&'static str> {
        vec![
            "190: 10 19",
            "3267: 81 40 27",
            "83: 17 5",
            "156: 15 6",
            "7290: 6 8 6 15",
            "161011: 16 10 13",
            "192: 17 8 14",
            "21037: 9 7 18 13",
            "292: 11 6 16 20",
        ]
    }

    #[test]
    fn test_parse() {
        let eqs = parse(&input());
        assert_eq!(eqs.len(), 9);
        assert_eq!(eqs[1].0, 3267);
        assert_eq!(eqs[1].1, vec![81, 40, 27]);
    }

    #[test]
    fn test_apply() {
        assert_eq!(apply(0b0, &[3, 4]), 7);
        assert_eq!(apply(0b1, &[3, 4]), 12);
        assert_eq!(apply(0b01, &[3, 4, 5]), 17);
        assert_eq!(apply(0b10, &[3, 4, 5]), 35);
    }

    #[test]
    fn test_expressable() {
        let eqs = parse(&input());
        assert!(expressable(eqs[0].clone()));
        assert!(expressable(eqs[1].clone()));
        assert!(!expressable(eqs[2].clone()));
        assert!(!expressable(eqs[3].clone()));
        assert!(!expressable(eqs[4].clone()));
        assert!(!expressable(eqs[5].clone()));
        assert!(!expressable(eqs[6].clone()));
        assert!(!expressable(eqs[7].clone()));
        assert!(expressable(eqs[8].clone()));
    }

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(&input()), 3749);
    }

    #[test]
    fn test_apply2() {
        assert_eq!(apply2(0, &[3, 4]), 7);
        assert_eq!(apply2(1, &[3, 4]), 12);
        assert_eq!(apply2(2, &[3, 4]), 34);
        assert_eq!(apply2(1 + 2 * 3 + 3i32.pow(2), &[6, 8, 6, 15]), 7290);
        assert_eq!(apply2(2 + 2 * 3 + 2 * 3i32.pow(2), &[1, 1, 1, 1]), 1111);
    }

    #[test]
    fn test_expressable2() {
        let eqs = parse(&input());
        let expre: Vec<i64> = eqs
            .iter()
            .filter(|&e| expressable2(e.clone()))
            .map(|&(n, _)| n)
            .collect();
        assert_eq!(expre, vec![190, 3267, 156, 7290, 192, 292]);
        assert!(expressable2((1111, vec![1, 1, 1, 1])));
    }

    #[test]
    fn test_prob2() {
        assert_eq!(prob2(&input()), 11387);
    }
}
