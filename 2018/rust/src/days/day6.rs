use std::collections::HashMap;

use crate::cli::Result;

pub struct Day {
    max_size: usize,
}

impl Default for Day {
    fn default() -> Self {
        Self { max_size: 10_000 }
    }
}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<(usize, usize)>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        data.lines()
            .map(|l| {
                let (a, b) = l.split_once(", ").unwrap();
                Ok((a.parse()?, b.parse()?))
            })
            .collect()
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let min_x = input.iter().map(|x| x.0).min().unwrap();
        let max_x = input.iter().map(|x| x.0).max().unwrap();
        let min_y = input.iter().map(|x| x.1).min().unwrap();
        let max_y = input.iter().map(|x| x.1).max().unwrap();

        let mut grid = vec![vec![0u8; max_x - min_x + 1]; max_y - min_y + 1];
        let input: Self::Input = input.iter().map(|(x, y)| (x - min_x, y - min_y)).collect();
        grid.iter_mut().enumerate().for_each(|(y, l)| {
            l.iter_mut().enumerate().for_each(|(x, val)| {
                let mut best = 99999;
                let mut best_point = 99;
                let mut many = false;
                input.iter().enumerate().for_each(|(n, &(x2, y2))| {
                    let this_dist = x.abs_diff(x2) + y.abs_diff(y2);
                    match this_dist.cmp(&best) {
                        std::cmp::Ordering::Less => {
                            best = this_dist;
                            best_point = n;
                            many = false;
                        }
                        std::cmp::Ordering::Equal => {
                            many = true;
                        }
                        _ => (),
                    }
                });
                *val = if many { 99 } else { best_point as u8 };
            })
        });
        let mut counts = HashMap::new();
        grid.iter()
            .flatten()
            .filter(|x| **x != 99)
            .for_each(|x| *counts.entry(x).or_insert(0) += 1);
        grid[0].iter().chain(grid.last().unwrap()).for_each(|x| {
            counts.remove(x);
        });
        grid.iter().for_each(|l| {
            counts.remove(&l[0]);
            counts.remove(l.last().unwrap());
        });

        Ok(counts.values().max().unwrap().to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let min_x = input.iter().map(|x| x.0).min().unwrap();
        let max_x = input.iter().map(|x| x.0).max().unwrap();
        let min_y = input.iter().map(|x| x.1).min().unwrap();
        let max_y = input.iter().map(|x| x.1).max().unwrap();

        let res = (min_x..=max_x)
            .flat_map(|x| {
                (min_y..=max_y).map(move |y| {
                    input
                        .iter()
                        .map(|&(x2, y2)| x.abs_diff(x2) + y.abs_diff(y2))
                        .sum::<usize>()
                })
            })
            .filter(|&n| n < self.max_size)
            .count();
        Ok(res.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "\
1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "17";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Day { max_size: 32 };
        let expected = "16";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
