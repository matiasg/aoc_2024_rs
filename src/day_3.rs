use regex::Regex;

fn prob1(line: &str) -> isize {
    let re = Regex::new("mul\\((\\d+),(\\d+)\\)").unwrap();
    re.captures_iter(line)
        .map(|c| {
            let (_, [m1, m2]) = c.extract();
            m1.parse::<isize>().unwrap() * m2.parse::<isize>().unwrap()
        })
        .sum()
}

fn prob2(line: &str) -> isize {
    let mulre = Regex::new("mul\\((\\d+),(\\d+)\\)").unwrap();
    let re = Regex::new("(mul\\((\\d+),(\\d+)\\)|do\\(\\)|don't\\(\\))").unwrap();
    let mut adding = true;
    let mut total = 0isize;
    for cap in re.find_iter(line) {
        let c = cap.as_str();
        if c == "do()" {
            adding = true;
        } else if c == "don't()" {
            adding = false;
        } else if adding {
            let groups = mulre.captures(c).unwrap();
            let m1: isize = groups.get(1).unwrap().as_str().parse().unwrap();
            let m2: isize = groups.get(2).unwrap().as_str().parse().unwrap();
            total += m1 * m2;
        }
    }
    total
}

pub(crate) fn main() {
    let line = include_str!("../input/day_3").trim();
    println!("prob1: {}", prob1(line));
    println!("prob2: {}", prob2(line));
}

#[cfg(test)]
pub mod tests {
    use crate::day_3::{prob1, prob2};

    fn input1() -> &'static str {
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
    }

    fn input2() -> &'static str {
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
    }

    #[test]
    fn test_prob_1() {
        assert_eq!(prob1(input1()), 161);
    }

    #[test]
    fn test_prob_2() {
        assert_eq!(prob2(input2()), 48);
    }
}
