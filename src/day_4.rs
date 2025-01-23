fn prob1(lines: Vec<&str>) -> usize {
    let lu8 = as_ints(lines);
    let mut count = 0usize;
    for (i, l) in lu8.iter().enumerate() {
        for (j, c) in l.iter().enumerate() {
            if *c == 0 {
                count += find_xmas(&lu8, i, j);
            }
        }
    }
    count
}

fn prob2(lines: Vec<&str>) -> usize {
    let lu8 = as_ints(lines);
    let mut count = 0usize;
    let maxi = lu8.len();
    let maxj = lu8[0].len();
    for (i, l) in lu8.iter().enumerate() {
        for (j, c) in l.iter().enumerate() {
            if *c == 2 {
                for ovs in ordered_vertices(i, j, maxi, maxj) {
                    if at(&lu8, ovs[0].0, ovs[0].1) == 1
                        && at(&lu8, ovs[1].0, ovs[1].1) == 1
                        && at(&lu8, ovs[2].0, ovs[2].1) == 3
                        && at(&lu8, ovs[3].0, ovs[3].1) == 3
                    {
                        count += 1;
                    }
                }
            }
        }
    }
    count
}

fn ordered_vertices(i: usize, j: usize, maxi: usize, maxj: usize) -> Vec<Vec<(isize, isize)>> {
    let (i, j) = (i as isize, j as isize);
    let (maxi, maxj) = (maxi as isize, maxj as isize);
    if i < 1 || i > maxi - 2 || j < 1 || j > maxj - 2 {
        vec![]
    } else {
        let ds = vec![
            (i - 1, j - 1),
            (i - 1, j + 1),
            (i + 1, j + 1),
            (i + 1, j - 1),
        ];
        vec![
            ds.clone(),
            ds[1..].iter().chain(ds[..1].iter()).copied().collect(),
            ds[2..].iter().chain(ds[..2].iter()).copied().collect(),
            ds[3..].iter().chain(ds[..3].iter()).copied().collect(),
        ]
    }
}

fn directions(i: isize, j: isize, maxi: usize, maxj: usize) -> Vec<(isize, isize)> {
    let mut ret: Vec<(isize, isize)> = vec![];
    let maxi = maxi as isize;
    let maxj = maxj as isize;
    for d1 in -1isize..2 {
        for d2 in -1isize..2 {
            if 0 <= i + 3 * d1 && i + 3 * d1 < maxi && 0 <= j + 3 * d2 && j + 3 * d2 < maxj {
                ret.push((d1, d2))
            }
        }
    }
    ret
}

fn at(lines: &[Vec<u8>], i: isize, j: isize) -> u8 {
    lines[i as usize][j as usize]
}

fn find_xmas(lines: &[Vec<u8>], i: usize, j: usize) -> usize {
    let i = i as isize;
    let j = j as isize;
    let maxi = lines.len();
    let maxj = lines[0].len();
    let mut count = 0usize;
    for (d1, d2) in directions(i, j, maxi, maxj) {
        if at(lines, i + d1, j + d2) == 1
            && at(lines, i + 2 * d1, j + 2 * d2) == 2
            && at(lines, i + 3 * d1, j + 3 * d2) == 3
        {
            count += 1;
        }
    }
    count
}

fn as_ints(lines: Vec<&str>) -> Vec<Vec<u8>> {
    lines
        .iter()
        .map(|l| {
            l.chars()
                .map(|c| match c {
                    'X' => 0,
                    'M' => 1,
                    'A' => 2,
                    'S' => 3,
                    _ => panic!("bad char"),
                })
                .collect()
        })
        .collect()
}

pub(crate) fn main() {
    let lines: Vec<&str> = include_str!("../input/day_4").trim().split("\n").collect();
    println!("prob1: {}", prob1(lines.clone()));
    println!("prob2: {}", prob2(lines.clone()));
}

#[cfg(test)]
pub mod tests {
    use super::{as_ints, prob1, prob2};

    fn input() -> Vec<&'static str> {
        vec![
            "MMMSXXMASM",
            "MSAMXMSMSA",
            "AMXSXMAAMM",
            "MSAMASMSMX",
            "XMASAMXAMM",
            "XXAMMXXAMA",
            "SMSMSASXSS",
            "SAXAMASAAA",
            "MAMMMXMMMM",
            "MXMXAXMASX",
        ]
    }

    #[test]
    fn test_prob1() {
        let lines = input();
        assert_eq!(prob1(lines.clone()), 18);
        let lu8 = as_ints(lines);
        assert_eq!(lu8.len(), 10);
        assert!(lu8.iter().map(|l| l.len()).all(|l| l == 10));
    }

    #[test]
    fn test_prob2() {
        assert_eq!(prob2(input()), 9);
    }
}
