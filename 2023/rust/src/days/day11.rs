use std::collections::BTreeSet;

use crate::cli::Result;

pub struct Day {
    expansion: usize,
}

impl Default for Day {
    fn default() -> Self {
        Self {
            expansion: 1_000_000,
        }
    }
}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<(usize, usize)>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        Ok(data
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.bytes()
                    .enumerate()
                    .filter_map(move |(x, b)| if b == b'#' { Some((x, y)) } else { None })
            })
            .collect())
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let rows: BTreeSet<usize> = input.iter().map(|v| v.1).collect();
        let cols: BTreeSet<usize> = input.iter().map(|v| v.0).collect();

        let mut dist = 0;
        crate::util::subsets(input, &mut [(0, 0); 2], &mut |[a, b]| {
            let x0 = a.0.min(b.0);
            let x1 = a.0.max(b.0);
            let y0 = a.1.min(b.1);
            let y1 = a.1.max(b.1);
            dist += ((x1 - x0) * 2) + ((y1 - y0) * 2)
                - rows.range(y0..y1).count()
                - cols.range(x0..x1).count();
            true
        });
        Ok(dist.to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let rows: BTreeSet<usize> = input.iter().map(|v| v.1).collect();
        let cols: BTreeSet<usize> = input.iter().map(|v| v.0).collect();

        let mut dist = 0;
        crate::util::subsets(input, &mut [(0, 0); 2], &mut |[a, b]| {
            let x0 = a.0.min(b.0);
            let x1 = a.0.max(b.0);
            let y0 = a.1.min(b.1);
            let y1 = a.1.max(b.1);
            dist += ((x1 - x0) * self.expansion) + ((y1 - y0) * self.expansion)
                - rows.range(y0..y1).count() * (self.expansion - 1)
                - cols.range(x0..x1).count() * (self.expansion - 1);
            true
        });
        Ok(dist.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "374";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Day { expansion: 10 };
        let data = d.gen(INPUT).unwrap();
        assert_eq!("1030", d.part2(&data).unwrap());
        d.expansion = 100;
        assert_eq!("8410", d.part2(&data).unwrap());
    }
}
