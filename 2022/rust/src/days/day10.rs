use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<Option<i32>>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        data.lines()
            .map(|l| {
                if let Some((_a, b)) = l.split_once(' ') {
                    Ok(Some(b.parse()?))
                } else {
                    Ok(None)
                }
            })
            .collect()
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let mut res = 0;
        let mut x = 1;
        let mut cycle = 1;
        for op in input {
            if let Some(rhs) = op {
                cycle += 1;
                if (cycle + 20) % 40 == 0 {
                    res += cycle * x;
                }
                x += rhs;
            }
            cycle += 1;

            if (cycle + 20) % 40 == 0 {
                res += cycle * x;
            }
        }
        Ok(res.to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let mut grid = String::new();
        let mut x: i32 = 0;
        let mut cycle = 0;

        let mut print = |x: i32, cycle| {
            let pos: i32 = cycle % 40;
            grid.push(if x.abs_diff(pos - 1) <= 1 { '#' } else { '.' });
            if pos == 39 {
                grid.push('\n');
            }
        };

        for op in input {
            print(x, cycle);
            cycle += 1;
            if let Some(rhs) = op {
                print(x, cycle);
                cycle += 1;
                x += rhs;
            }
        }
        Ok(grid)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "13140";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
