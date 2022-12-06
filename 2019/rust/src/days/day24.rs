use std::collections::HashSet;

use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = [[bool; 5]; 5];

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        let mut res: Self::Input = Default::default();
        data.lines().enumerate().for_each(|(y, l)| {
            l.bytes()
                .enumerate()
                .for_each(|(x, b)| res[y][x] = b == b'#');
        });
        Ok(res)
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let mut visited = HashSet::new();
        let mut this = *input;

        loop {
            let mut next = [[false; 5]; 5];

            for y in 0..5 {
                for x in 0..5 {
                    let mut around = 0;
                    if y > 0 {
                        around += usize::from(this[y - 1][x]);
                    }
                    if x > 0 {
                        around += usize::from(this[y][x - 1]);
                    }
                    if y < 4 {
                        around += usize::from(this[y + 1][x]);
                    }
                    if x < 4 {
                        around += usize::from(this[y][x + 1]);
                    }
                    next[y][x] = around == 1 || (!this[y][x] && around == 2);
                }
            }

            this = next;
            if !visited.insert(this) {
                break;
            }
        }

        Ok(bioscore(&this).to_string())
    }

    fn part2(&mut self, grid: &Self::Input) -> Result<String> {
        Ok(simulate(grid, 200).to_string())
    }
}

fn bioscore(grid: &[[bool; 5]; 5]) -> usize {
    grid.iter()
        .flatten()
        .rev()
        .fold(0, |acc, x| acc * 2 + usize::from(*x))
}

fn simulate(init: &[[bool; 5]; 5], rounds: usize) -> usize {
    let levels = rounds + 3; // grows every 2 round by 2
    let mut grids = vec![[[false; 5]; 5]; levels];
    grids[levels / 2] = *init;
    for _ in 0..rounds {
        let mut next = vec![[[false; 5]; 5]; levels];

        for (z, this) in grids.iter().enumerate().skip(1).take(levels - 2) {
            for y in 0..5 {
                for x in 0..5 {
                    let mut around = 0;
                    if x == 2 && y == 2 {
                        continue;
                    }
                    if y > 0 {
                        around += usize::from(this[y - 1][x]);
                    } else {
                        around += usize::from(grids[z - 1][1][2]);
                    }
                    if x > 0 {
                        around += usize::from(this[y][x - 1]);
                    } else {
                        around += usize::from(grids[z - 1][2][1]);
                    }
                    if y < 4 {
                        around += usize::from(this[y + 1][x]);
                    } else {
                        around += usize::from(grids[z - 1][3][2]);
                    }
                    if x < 4 {
                        around += usize::from(this[y][x + 1]);
                    } else {
                        around += usize::from(grids[z - 1][2][3]);
                    }

                    // FIXME 4 center
                    around += match (x, y) {
                        (1, 2) => grids[z + 1].iter().filter(|row| row[0]).count(),
                        (2, 1) => grids[z + 1][0].iter().filter(|col| **col).count(),
                        (3, 2) => grids[z + 1].iter().filter(|row| row[4]).count(),
                        (2, 3) => grids[z + 1][4].iter().filter(|col| **col).count(),
                        _ => 0,
                    };

                    next[z][y][x] = around == 1 || (!this[y][x] && around == 2);
                }
            }
        }

        grids = next;
    }

    grids.iter().flatten().flatten().filter(|x| **x).count()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let input = "\
....#
#..#.
#..##
..#..
#....";
        let expected = "2129920";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let input = "\
....#
#..#.
#..##
..#..
#....";
        let expected = 99;
        let data = d.gen(input).unwrap();
        assert_eq!(expected, simulate(&data, 10));
    }
}
