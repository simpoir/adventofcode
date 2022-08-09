use std::collections::HashSet;

use crate::cli::Result;

pub struct Day {
    bursts: isize,
}
impl Default for Day {
    fn default() -> Self {
        Self { bursts: 10_000 }
    }
}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = HashSet<(isize, isize)>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        let offset = data.lines().next().unwrap().len() as isize / 2;
        let grid: HashSet<(isize, isize)> = data
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars().enumerate().filter_map(move |(x, n)| {
                    if n == '#' {
                        Some((x as isize - offset, y as isize - offset))
                    } else {
                        None
                    }
                })
            })
            .collect();
        Ok(grid)
    }

    fn part1(&mut self, grid: &Self::Input) -> Result<String> {
        let mut grid = grid.clone();
        let mut infections = 0;
        let (mut x, mut y) = (0isize, 0isize);
        // up is negative y
        let (mut dx, mut dy) = (0, -1);
        for _ in 0..self.bursts {
            if grid.remove(&(x, y)) {
                (dx, dy) = (-dy, dx);
            } else {
                grid.insert((x, y));
                (dx, dy) = (dy, -dx);
                infections += 1;
            }
            x += dx;
            y += dy;
        }
        Ok(infections.to_string())
    }

    fn part2(&mut self, grid: &Self::Input) -> Result<String> {
        let mut infected = grid.clone();
        let mut weak = HashSet::new();
        let mut flagged = HashSet::new();
        let mut infections = 0;
        let (mut x, mut y) = (0isize, 0isize);
        // up is negative y
        let (mut dx, mut dy) = (0, -1);
        for _ in 0..10_000_000 {
            if infected.remove(&(x, y)) {
                (dx, dy) = (-dy, dx);
                flagged.insert((x, y));
            } else if weak.remove(&(x, y)) {
                infections += 1;
                infected.insert((x, y));
            } else if flagged.remove(&(x, y)) {
                (dx, dy) = (-dx, -dy);
            } else {
                (dx, dy) = (dy, -dx);
                weak.insert((x, y));
            }
            x += dx;
            y += dy;
        }
        Ok(infections.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "\
..#
#..
...";

    #[test]
    fn test_part1() {
        let mut d: Day = Day { bursts: 70 };
        let expected = "41";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "2511944";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
