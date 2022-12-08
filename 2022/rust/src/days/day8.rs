use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<Vec<u8>>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        Ok(data
            .lines()
            .map(|l| l.bytes().map(|c| c - b'0').collect())
            .collect())
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let mut visible = vec![vec![false; input[0].len()]; input.len()];
        for (y, row) in input.iter().enumerate() {
            let mut max = 0;
            for (x, &h) in row.iter().enumerate() {
                if h > max {
                    visible[y][x] = true;
                    max = h;
                }
            }
            let mut max = 0;
            for (x, &h) in row.iter().enumerate().rev() {
                if h > max {
                    visible[y][x] = true;
                    max = h;
                }
            }
        }

        for x in 0..input[0].len() {
            let mut max = 0;
            for (y, row) in input.iter().enumerate() {
                if row[x] > max {
                    visible[y][x] = true;
                    max = row[x]
                }
            }
            let mut max = 0;
            for (y, row) in input.iter().enumerate().rev() {
                if row[x] > max {
                    visible[y][x] = true;
                    max = row[x]
                }
            }
        }
        visible[0].iter_mut().for_each(|x| *x = true);
        visible[input.len() - 1].iter_mut().for_each(|x| *x = true);
        visible.iter_mut().for_each(|row| {
            row[0] = true;
            *row.last_mut().unwrap() = true;
        });

        Ok(visible.iter().flatten().filter(|b| **b).count().to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let mut best = 0;
        for x in 0..input[0].len() {
            for y in 0..input.len() {
                let limit = input[y][x];
                let score = [(0isize, 1isize), (0, -1), (1, 0), (-1, 0)]
                    .iter()
                    .map(|(dx, dy)| {
                        let mut x = x as isize + dx;
                        let mut y = y as isize + dy;
                        let mut i = 0;
                        while let Some(val) =
                            input.get(y as usize).and_then(|row| row.get(x as usize))
                        {
                            if *val >= limit {
                                // the tree blocking your view counts.
                                i += 1;
                                break;
                            }
                            i += 1;
                            x += dx;
                            y += dy;
                        }
                        i
                    })
                    .product();
                best = best.max(score);
            }
        }
        Ok(best.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "30373
25512
65332
33549
35390
";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "21";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "8";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
