use std::collections::HashSet;

use crate::utils::counter;

#[derive(PartialEq, Eq, Hash, Clone, Debug, Copy)]
enum Orientation {
    Horizontal,
    Vertical,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Side {
    orientation: Orientation,
    fixed: isize,
    range: (isize, isize),
}

impl Side {
    #[allow(dead_code)]
    fn len(&self) -> usize {
        (self.range.1 - self.range.0) as usize
    }
    fn is_contiguous(&self, other: &Side) -> bool {
        if self.orientation != other.orientation {
            return false;
        }
        self.fixed == other.fixed
            && (self.range.1 == other.range.0 || self.range.0 == other.range.1)
    }
    fn bounds_same_side(&self, other: &Self, region: &Region) -> bool {
        // we assume self.is_contiguous(other) and both bound region
        match self.orientation {
            Orientation::Vertical => {
                region.plots.contains(&(self.range.0, self.fixed))
                    == region.plots.contains(&(other.range.0, other.fixed))
            }
            Orientation::Horizontal => {
                region.plots.contains(&(self.fixed, self.range.0))
                    == region.plots.contains(&(other.fixed, other.range.0))
            }
        }
    }
    fn join(&self, other: &Side) -> Self {
        assert!(self.is_contiguous(other));
        let range = if self.range.1 == other.range.0 {
            (self.range.0, other.range.1)
        } else {
            (other.range.0, self.range.1)
        };
        Self {
            orientation: self.orientation,
            fixed: self.fixed,
            range,
        }
    }

    fn southeast(&self) -> Side {
        Self {
            orientation: self.orientation,
            fixed: self.fixed,
            range: (self.range.1, self.range.1 + 1),
        }
    }

    fn northwest(&self) -> Side {
        Self {
            orientation: self.orientation,
            fixed: self.fixed,
            range: (self.range.0 - 1, self.range.0),
        }
    }
}
struct Region {
    _plant: char,
    plots: HashSet<(isize, isize)>,
    height: isize,
    width: isize,
}
impl Region {
    fn from_plot(garden: &[Vec<char>], plot: (isize, isize)) -> Self {
        let height = garden.len() as isize;
        let width = garden[0].len() as isize;
        let plant = garden[plot.0 as usize][plot.1 as usize];
        let mut plots: HashSet<(isize, isize)> = HashSet::new();
        let mut frontier: HashSet<(isize, isize)> = HashSet::from([plot]);
        while !frontier.is_empty() {
            plots = plots.union(&frontier).copied().collect();
            let nf: HashSet<(isize, isize)> = frontier
                .iter()
                .flat_map(|&p| {
                    Self::adjacent(p, height, width)
                        .iter()
                        .filter(|&p| garden[p.0 as usize][p.1 as usize] == plant)
                        .copied()
                        .collect::<HashSet<(isize, isize)>>()
                })
                .collect();
            frontier = nf.difference(&plots).copied().collect();
        }
        Self {
            _plant: plant,
            plots,
            height,
            width,
        }
    }
    fn adjacent(plot: (isize, isize), height: isize, width: isize) -> HashSet<(isize, isize)> {
        let mut ret: HashSet<(isize, isize)> = HashSet::new();
        if plot.0 > 0 {
            ret.insert((plot.0 - 1, plot.1));
        }
        if plot.0 < height - 1 {
            ret.insert((plot.0 + 1, plot.1));
        }
        if plot.1 > 0 {
            ret.insert((plot.0, plot.1 - 1));
        }
        if plot.1 < width - 1 {
            ret.insert((plot.0, plot.1 + 1));
        }
        ret
    }

    fn area(&self) -> usize {
        self.plots.len()
    }

    fn perimeter(&self) -> usize {
        let internal = self
            .plots
            .iter()
            .map(|&p| {
                Self::adjacent(p, self.height, self.width)
                    .intersection(&self.plots)
                    .collect::<Vec<&(isize, isize)>>()
                    .len()
            })
            .sum();
        (4 * self.area()).checked_sub(internal).unwrap()
    }

    fn price(&self) -> usize {
        self.area() * self.perimeter()
    }

    fn sides(&self) -> usize {
        let all_borders: Vec<Side> = self.plots.iter().flat_map(|&p| Self::borders(p)).collect();
        let border_count = counter(&all_borders);
        let mut boundary: HashSet<Side> = border_count
            .iter()
            .filter(|&(_, &n)| n == 1)
            .map(|(&b, _)| b)
            .copied()
            .collect();
        let mut all_sides: Vec<Side> = vec![];
        while !boundary.is_empty() {
            let v: Vec<Side> = boundary.iter().copied().collect();
            let mut side = v[0];
            boundary.remove(&side);
            loop {
                let next_bound = side.southeast();
                if boundary.contains(&next_bound) && side.bounds_same_side(&next_bound, self) {
                    boundary.remove(&next_bound);
                    side = side.join(&next_bound);
                } else {
                    break;
                }
            }
            loop {
                let next_bound = side.northwest();
                if boundary.contains(&next_bound) && side.bounds_same_side(&next_bound, self) {
                    boundary.remove(&next_bound);
                    side = side.join(&next_bound);
                } else {
                    break;
                }
            }
            all_sides.push(side);
        }
        all_sides.len()
    }

    fn borders(plot: (isize, isize)) -> Vec<Side> {
        vec![
            Side {
                orientation: Orientation::Horizontal,
                fixed: plot.0,
                range: (plot.1, plot.1 + 1),
            },
            Side {
                orientation: Orientation::Horizontal,
                fixed: plot.0 + 1,
                range: (plot.1, plot.1 + 1),
            },
            Side {
                orientation: Orientation::Vertical,
                fixed: plot.1,
                range: (plot.0, plot.0 + 1),
            },
            Side {
                orientation: Orientation::Vertical,
                fixed: plot.1 + 1,
                range: (plot.0, plot.0 + 1),
            },
        ]
    }

    fn price_discount(&self) -> usize {
        self.area() * self.sides()
    }
}

fn regions(garden: &[Vec<char>]) -> Vec<Region> {
    let height = garden.len() as isize;
    let width = garden[0].len() as isize;
    let mut remaining: HashSet<(isize, isize)> = (0..height)
        .flat_map(|i| (0..width).map(move |j| (i, j)))
        .collect();
    let mut regions: Vec<Region> = vec![];
    while !remaining.is_empty() {
        let &plot = remaining.iter().next().unwrap();
        let region = Region::from_plot(garden, plot);
        remaining = remaining.difference(&region.plots).copied().collect();
        regions.push(region);
    }
    regions
}

fn prob1(input: Vec<&str>) -> usize {
    let rs = regions(&parse(&input));
    rs.iter().map(|r| r.price()).sum()
}

fn prob2(input: Vec<&str>) -> usize {
    let rs = regions(&parse(&input));
    rs.iter().map(|r| r.price_discount()).sum()
}

fn parse(input: &[&str]) -> Vec<Vec<char>> {
    input.iter().map(|line| line.chars().collect()).collect()
}

pub(crate) fn main() {
    let input: Vec<&str> = include_str!("../input/day_12").trim().split("\n").collect();
    println!("prob1: {}", prob1(input.clone()));
    println!("prob2: {}", prob2(input.clone()));
}

#[cfg(test)]
pub mod tests {
    use std::collections::HashSet;

    use super::{parse, prob1, prob2, regions, Orientation, Region, Side};

    fn input1() -> Vec<&'static str> {
        vec!["AAAA", "BBCD", "BBCC", "EEEC"]
    }
    fn input3() -> Vec<&'static str> {
        vec!["OOOOO", "OXOXO", "OOOOO", "OXOXO", "OOOOO"]
    }
    fn input4() -> Vec<&'static str> {
        vec!["EEEEE", "EXXXX", "EEEEE", "EXXXX", "EEEEE"]
    }
    fn input5() -> Vec<&'static str> {
        vec!["AAAAAA", "AAABBA", "AAABBA", "ABBAAA", "ABBAAA", "AAAAAA"]
    }

    fn input2() -> Vec<&'static str> {
        vec![
            "RRRRIICCFF",
            "RRRRIICCCF",
            "VVRRRCCFFF",
            "VVRCCCJFFF",
            "VVVVCJJCFE",
            "VVIVCCJJEE",
            "VVIIICJJEE",
            "MIIIIIJJEE",
            "MIIISIJEEE",
            "MMMISSJEEE",
        ]
    }

    #[test]
    fn test_from_plot() {
        let g = parse(&input1());
        let r = Region::from_plot(&g.clone(), (0, 0));
        assert_eq!(r._plant, 'A');
        assert_eq!(r.plots.len(), 4);
        let r = Region::from_plot(&g.clone(), (1, 0));
        assert_eq!(r.plots.len(), 4);
        assert_eq!(r.plots, HashSet::from([(1, 0), (1, 1), (2, 0), (2, 1)]));
    }

    #[test]
    fn test_regions() {
        let g = parse(&input1());
        let rs = regions(&g);
        assert_eq!(rs.len(), 5);
        assert_eq!(
            rs.iter().map(|r| r._plant).collect::<HashSet<char>>(),
            "ABCDE".chars().collect::<HashSet<char>>()
        );
        let g = parse(&input2());
        let rs = regions(&g);
        assert_eq!(rs.len(), 11);
        let g = parse(&input3());
        let rs = regions(&g);
        assert_eq!(rs.len(), 5);
        assert_eq!(
            rs.iter().map(|r| r._plant).collect::<HashSet<char>>(),
            "XO".chars().collect::<HashSet<char>>()
        );
    }

    #[test]
    fn test_price() {
        let g = parse(&input2());
        let r = Region::from_plot(&g, (0, 0));
        assert_eq!(r.area(), 12);
        assert_eq!(r.perimeter(), 18);
        assert_eq!(r.price(), 216);
    }

    #[test]
    fn test_prob1() {
        let g = input1();
        assert_eq!(prob1(g), 140);
        let g = input2();
        assert_eq!(prob1(g), 1930);
        let g = input3();
        assert_eq!(prob1(g), 772);
    }

    #[test]
    fn test_side() {
        let s = Side {
            orientation: Orientation::Horizontal,
            fixed: 3,
            range: (5, 7),
        };
        let t = Side {
            orientation: Orientation::Horizontal,
            fixed: 3,
            range: (7, 9),
        };
        assert!(s.is_contiguous(&t));
        assert!(t.is_contiguous(&s));
        let st = s.join(&t);
        assert!(!s.is_contiguous(&st));
        assert!(!t.is_contiguous(&st));
        let ts = t.join(&s);
        assert_eq!(st, ts);
        assert_eq!(st.range, (5, 9));
        let u = Side {
            orientation: Orientation::Vertical,
            fixed: 3,
            range: (7, 9),
        };
        assert!(!s.is_contiguous(&u));
    }

    #[test]
    fn test_prob2() {
        assert_eq!(prob2(input1()), 80);
        assert_eq!(prob2(input2()), 1206);
        assert_eq!(prob2(input4()), 236);
        assert_eq!(prob2(input5()), 368);
    }
}
