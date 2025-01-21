fn parse_lines(lines: Vec<&str>) -> Vec<Vec<isize>> {
    lines
        .iter()
        .map(|ln| ln.split(' ').map(|i| i.parse().unwrap()).collect())
        .collect()
}

fn is_safe(line: Vec<isize>) -> bool {
    if line.len() <= 1 {
        return true;
    };
    let sig = line[1] - line[0];
    for (p, q) in line.iter().zip(line.iter().skip(1)) {
        if sig * (q - p) <= 0 {
            return false;
        }
        if (p - q).abs() > 3 {
            return false;
        }
    }
    true
}

fn is_safe_removing_one(line: Vec<isize>) -> bool {
    (0..line.len()).any(|i| {
        is_safe(
            line.get(..i)
                .unwrap()
                .iter()
                .chain(line.get(i + 1..).unwrap())
                .copied()
                .collect(),
        )
    })
}

fn prob1(lines: Vec<&str>) -> usize {
    let lines = parse_lines(lines);
    lines.iter().filter(|&v| is_safe(v.clone())).count()
}

fn prob2(lines: Vec<&str>) -> usize {
    let lines = parse_lines(lines);
    lines
        .iter()
        .filter(|&v| is_safe_removing_one(v.clone()))
        .count()
}

pub fn main() {
    let lines: Vec<&str> = include_str!("../input/day_2").trim().split("\n").collect();
    println!("prob1: {}", prob1(lines.clone()));
    println!("prob2: {}", prob2(lines.clone()));
}

#[cfg(test)]
pub mod tests {
    use crate::day_2::{parse_lines, prob1, prob2};

    fn input() -> Vec<&'static str> {
        vec![
            "7 6 4 2 1",
            "1 2 7 8 9",
            "9 7 6 2 1",
            "1 3 2 4 5",
            "8 6 4 4 1",
            "1 3 6 7 9",
        ]
    }

    #[test]
    fn test_parse() {
        let pl = parse_lines(input());
        assert_eq!(pl.len(), 6);
    }

    #[test]
    fn test_p1() {
        assert_eq!(prob1(input()), 2);
    }

    #[test]
    fn test_p2() {
        assert_eq!(prob2(input()), 4);
    }
}
