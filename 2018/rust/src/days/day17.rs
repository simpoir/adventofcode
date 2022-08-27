use std::ops::RangeInclusive;

use crate::cli::Result;

#[derive(Default)]
pub struct Day {
    part2: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Clay,
    Sand,
    Water,
    Flow,
}
use self::Cell::*;

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<(RangeInclusive<usize>, RangeInclusive<usize>)>;

    fn need_part1() -> bool {
        true
    }

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        data.lines()
            .map(|l| {
                let mut x = 0..=0;
                let mut y = 0..=0;
                for chunk in l.split(", ") {
                    let range = if let Some((start, end)) = chunk.get(2..).unwrap().split_once("..")
                    {
                        let start = start.parse()?;
                        let end = end.parse()?;
                        start..=end
                    } else {
                        let point = chunk.get(2..).unwrap().parse()?;
                        point..=point
                    };
                    if chunk.starts_with('x') {
                        x = range;
                    } else {
                        y = range;
                    }
                }
                Ok((x, y))
            })
            .collect()
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let bottom = input.iter().map(|n| n.1.end()).max().unwrap() + 1;
        let top = input.iter().map(|n| n.1.start()).min().unwrap();
        let min_x = input.iter().map(|(x, _)| x.start()).min().unwrap() - 1;
        let max_x = input.iter().map(|(x, _)| x.start()).max().unwrap() + 2;
        let mut grid = vec![vec![Cell::Sand; max_x - min_x]; bottom - top];
        for (xrange, yrange) in input {
            for x in xrange.clone() {
                for y in yrange.clone() {
                    grid[y - top][x - min_x] = Cell::Clay;
                }
            }
        }
        fludify(&mut grid, (500 - min_x, 0));

        self.part2 = grid.iter().flatten().filter(|c| **c == Cell::Water).count();
        Ok(grid
            .iter()
            .flatten()
            .filter(|c| matches!(c, Cell::Water | Cell::Flow))
            .count()
            .to_string())
    }

    fn part2(&mut self, _input: &Self::Input) -> Result<String> {
        Ok(self.part2.to_string())
    }
}

/// Move fluid point around. Returns true if fluid point is draining.
fn fludify(grid: &mut [Vec<Cell>], (x0, y0): (usize, usize)) -> bool {
    let (x, mut y) = (x0, y0);
    grid[y][x] = Flow;

    while y + 1 < grid.len() && matches!(grid[y + 1][x], Sand | Flow) {
        y += 1;
        grid[y][x] = Flow;
    }
    if y + 1 == grid.len() {
        return true;
    }

    for y in ((y0 + 1)..(y + 1)).rev() {
        let slicea = &grid[y];
        let sliceb = &grid[y + 1];
        let lside = walled(slicea, sliceb, x, false);
        let rside = walled(slicea, sliceb, x, true);
        let slice = &mut grid[y];
        if lside.0 && rside.0 {
            slice[lside.1..=rside.1].fill(Water);
            continue;
        }
        slice[lside.1..=rside.1].fill(Flow);
        let a = lside.0 || fludify(grid, (lside.1, y));
        let b = rside.0 || fludify(grid, (rside.1, y));
        if a || b {
            return true;
        }
    }
    false
}

#[allow(unused)]
fn dbg_grid(grid: &[Vec<Cell>]) {
    println!();
    for line in grid {
        for c in line[..].iter() {
            print!(
                "{}",
                match c {
                    Cell::Clay => '#',
                    Cell::Sand => '.',
                    Cell::Water => '~',
                    Cell::Flow => '|',
                }
            );
        }
        println!();
    }
}

fn walled(
    slicea: &[Cell],
    sliceb: &[Cell],
    mut x: usize,
    right: bool,
) -> (/*is walled*/ bool, usize) {
    loop {
        if matches!(sliceb[x], Sand | Flow) {
            return (false, x);
        }
        let prev_x = x;
        if right {
            x += 1;
        } else {
            x -= 1;
        }
        if slicea[x] == Clay {
            return (true, prev_x);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "\
x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "57";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "29";
        let data = d.gen(INPUT).unwrap();
        d.part1(&data).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
