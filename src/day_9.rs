use std::collections::{BTreeSet, HashMap};

fn parse(line: String) -> Vec<isize> {
    let mut ret: Vec<isize> = Vec::with_capacity(20 * line.len());
    for i in (0..line.len()).step_by(2) {
        let a: usize = line[i..i + 1].parse().unwrap();
        for _ in 0..a {
            ret.push((i / 2) as isize);
        }
        if i + 1 < line.len() {
            let b: usize = line[i + 1..i + 2].parse().unwrap();
            for _ in 0..b {
                ret.push(-1);
            }
        }
    }
    ret
}

fn compact(disk: &mut Vec<isize>) {
    let mut left = 0usize;
    while left < disk.len() {
        if disk[left] >= 0 {
            left += 1;
            continue;
        }
        disk[left] = disk.pop().unwrap();
        left += 1;
        while disk.last().unwrap() == &-1 {
            disk.pop();
        }
    }
}

fn prob1(line: String) -> usize {
    let mut disk = parse(line);
    compact(&mut disk);
    disk.iter()
        .enumerate()
        .map(|(pos, &id)| pos * (id as usize))
        .sum()
}

#[derive(Debug)]
struct Files {
    files: Vec<(usize, usize, isize)>,      // (start, length, id)
    holes: HashMap<usize, BTreeSet<usize>>, // length -> start
}

impl Files {
    fn from_string(line: String) -> Self {
        let mut files: Vec<(usize, usize, isize)> = vec![];
        let mut holes: HashMap<usize, BTreeSet<usize>> = HashMap::new();
        let mut pos: usize = 0;
        for i in (0..line.len()).step_by(2) {
            let a: usize = line[i..i + 1].parse().unwrap();
            files.push((pos, a, i as isize / 2));
            pos += a;
            if i + 1 < line.len() {
                let b: usize = line[i + 1..i + 2].parse().unwrap();
                holes.entry(b).or_default().insert(pos);
                pos += b;
            }
        }
        Self { files, holes }
    }

    fn lmh(&self, max_pos: usize, min_len: usize) -> Option<usize> {
        let leftmost_holes = self
            .holes
            .iter()
            .filter(|(&hlen, hpositions)| *hpositions.first().unwrap() < max_pos && hlen >= min_len)
            .min_by(|(_, posa), (_, posb)| posa.first().unwrap().cmp(posb.first().unwrap()));
        if let Some(em) = leftmost_holes {
            return Some(*em.0);
        }
        None
    }

    fn compact(&mut self) {
        let mut file_vec_pos = self.files.len() - 1;
        while file_vec_pos > 0 {
            let (pos, len, id) = self.files[file_vec_pos];
            let leftmost_holes = self.lmh(pos, len);
            if leftmost_holes.is_some() {
                let hlen = leftmost_holes.unwrap();
                let hpos = *self.holes.get(&hlen).unwrap().first().unwrap();
                self.holes.entry(hlen).and_modify(|hp| {
                    hp.pop_first();
                });
                if self.holes.get(&hlen).unwrap().is_empty() {
                    self.holes.remove(&hlen);
                }
                self.files.remove(file_vec_pos);
                let new_file_vec_pos = self
                    .files
                    .iter()
                    .enumerate()
                    .find(|&(_, &(fp, _, _))| fp >= hpos)
                    .unwrap();
                self.files.insert(new_file_vec_pos.0, (hpos, len, id));
                // new hole is hlen - len
                let left_hole_len = hlen - len;
                let left_hole_pos = hpos + len;
                self.holes
                    .entry(left_hole_len)
                    .or_default()
                    .insert(left_hole_pos);
            } else {
                file_vec_pos -= 1;
            }
        }
    }

    fn checksum(&self) -> usize {
        self.files
            .iter()
            .map(|&(pos, length, id)| (pos * length + (length * (length - 1) / 2)) * id as usize)
            .sum()
    }
}

fn prob2(line: String) -> usize {
    let mut files = Files::from_string(line);
    files.compact();
    files.checksum()
}

pub(crate) fn main() {
    let line = include_str!("../input/day_9").trim().to_string();
    println!("prob1: {}", prob1(line.clone()));
    println!("prob2: {}", prob2(line.clone()));
}

#[cfg(test)]
pub mod tests {
    use std::collections::BTreeSet;

    use super::{compact, parse, prob1, Files};

    fn input() -> String {
        "2333133121414131402".to_string()
    }

    #[test]
    fn test_prob1() {
        assert_eq!(prob1(input()), 1928);
    }

    #[test]
    fn test_parse() {
        let d = parse(input());
        assert_eq!(d.len(), 42);
    }

    #[test]
    fn test_compact() {
        let mut d = parse(input());
        compact(&mut d);
        assert_eq!(
            d,
            vec![
                0, 0, 9, 9, 8, 1, 1, 1, 8, 8, 8, 2, 7, 7, 7, 3, 3, 3, 6, 4, 4, 6, 5, 5, 5, 5, 6, 6,
            ]
        )
    }

    #[test]
    fn test_files() {
        let mut files = Files::from_string(input());
        assert_eq!(files.files.len(), 10);
        assert_eq!(files.files.last().unwrap(), &(40, 2, 9));
        assert_eq!(files.holes[&3], BTreeSet::from([2, 8, 12]));
        assert_eq!(files.holes[&0], BTreeSet::from([40]));
        assert_eq!(files.holes.get(&5), None);
        assert!(files.checksum() > 2858);
        files.compact();
        assert_eq!(files.checksum(), 2858);
    }
}
