use std::collections::{HashMap, HashSet};

use crate::cli::Result;

pub struct Day {
    part1: (isize, isize),
    nth: usize,
}

impl Default for Day {
    fn default() -> Self {
        Self {
            part1: (0, 0),
            nth: 200,
        }
    }
}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<(isize, isize)>;

    fn need_part1() -> bool {
        true
    }

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        Ok(data
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.bytes().enumerate().filter_map(move |(x, b)| {
                    if b == b'#' {
                        Some((x as isize, y as isize))
                    } else {
                        None
                    }
                })
            })
            .collect())
    }

    fn part1(&mut self, points: &Self::Input) -> Result<String> {
        let mut best = 0;
        for a in points.iter() {
            let mut seen = HashSet::new();
            for b in points.iter() {
                let dx = b.0 - a.0;
                let dy = b.1 - a.1;
                seen.insert(factor(dx, dy));
            }
            if seen.len() > best {
                best = seen.len() -1 /* self */;
                self.part1 = *a;
            }
        }
        Ok(best.to_string())
    }

    fn part2(&mut self, points: &Self::Input) -> Result<String> {
        let mut stacks = HashMap::new();
        let a = self.part1;
        for b in points {
            let dx = b.0 - a.0;
            let dy = b.1 - a.1;
            stacks
                .entry(factor(dx, dy))
                .or_insert_with(Vec::new)
                .push((dx, dy));
        }
        stacks.remove(&(0, 0));
        stacks.values_mut().for_each(|stack| {
            stack.sort_by(|a, b| a.0.abs().cmp(&b.0.abs()).then(a.1.abs().cmp(&b.1.abs())));
            stack.reverse();
        });

        let mut ordered_stacks: Vec<_> = stacks
            .drain()
            .map(|kv| {
                (
                    {
                        let (x, y) = kv.0;
                        if x >= 0 && y < 0 {
                            (0, (x * 1000).saturating_div(-y))
                        } else if x > 0 && y >= 0 {
                            (1, (y * 1000).saturating_div(x))
                        } else if x <= 0 && y > 0 {
                            (2, (-x * 1000).saturating_div(y))
                        } else {
                            (3, (y * 1000).saturating_div(x))
                        }
                    },
                    kv.1,
                )
            })
            .collect();
        ordered_stacks.sort();

        let mut i = 0;
        while !ordered_stacks.is_empty() {
            for (_, stack) in ordered_stacks.iter_mut() {
                i += 1;
                let (dx, dy) = stack.pop().unwrap();

                if i == self.nth {
                    return Ok(((self.part1.0 + dx) * 100 + self.part1.1 + dy).to_string());
                }
            }

            ordered_stacks.retain(|(_, stack)| !stack.is_empty());
        }

        unimplemented!("not enough points");
    }
}

fn factor(a: isize, b: isize) -> (isize, isize) {
    for div in (1..=(a.abs().max(b.abs()))).rev() {
        if a % div == 0 && b % div == 0 {
            return (a / div, b / div);
        }
    }
    (a, b)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let input = ".#..#
.....
#####
....#
...##";
        let expected = "8";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Day {
            part1: (8, 3),
            nth: 5,
        };
        let input = ".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....X...###..
..#.#.....#....##";
        let expected = "902";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());

        let mut d = Day {
            nth: 200,
            part1: (0, 0),
        };
        let data = d
            .gen(
                ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##",
            )
            .unwrap();
        d.part1(&data).unwrap();
        assert_eq!((11, 13), d.part1);
        assert_eq!("802", d.part2(&data).unwrap());
    }
}
