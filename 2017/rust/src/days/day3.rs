#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = i64;

    fn gen(&mut self, data: &str) -> Result<Self::Input, anyhow::Error> {
        Ok(data.parse()?)
    }

    fn part1(&mut self, input: &Self::Input) -> crate::cli::Output {
        let input = *input;
        let circle = ((((input as f64).sqrt()) - 1.0) / 2.0).ceil() as i64;
        let circlestart = (((circle - 1) * 2) + 1).pow(2);

        let edge = (input - circlestart) % (circle * 2);
        let offset = edge - circle;
        Ok((circle + offset.abs()).to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> crate::cli::Output {
        let mut grid = [[0; 64]; 64];
        let mut x = 32;
        let mut y = 32;
        grid[x][y] = 1;
        for i in 1.. {
            let flip = if i & 1 == 1 { 1 } else { -1 };
            for (dx, dy) in [(1, 0), (0, 1)] {
                for _ in 0..i {
                    x = (x as i32 + flip * dx) as usize;
                    y = (y as i32 + flip * dy) as usize;
                    grid[x][y] = grid[x - 1][y]
                        + grid[x + 1][y]
                        + grid[x - 1][y - 1]
                        + grid[x][y - 1]
                        + grid[x + 1][y - 1]
                        + grid[x - 1][y + 1]
                        + grid[x][y + 1]
                        + grid[x + 1][y + 1];
                    if grid[x][y] > *input {
                        return Ok(grid[x][y].to_string());
                    }
                }
            }
        }
        panic!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let input = "12";
        let expected = "3";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let input = "800";
        let expected = "806";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
