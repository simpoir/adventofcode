use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<(u8, u8)>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        Ok(data
            .lines()
            .map(|l| (l.as_bytes()[0], l.as_bytes()[2]))
            .collect())
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let res: usize = input
            .iter()
            .map(|(them, you)| {
                let win = match (&them, &you) {
                    (b'C', b'X') | (b'A', b'Y') | (b'B', b'Z') => 6,
                    (b'A', b'X') | (b'B', b'Y') | (b'C', b'Z') => 3,
                    _ => 0,
                };
                win + match you {
                    b'X' => 1,
                    b'Y' => 2,
                    b'Z' => 3,
                    _ => unimplemented!("{you}"),
                }
            })
            .sum();
        Ok(res.to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let res: usize = input
            .iter()
            .map(|(them, you)| match (&them, &you) {
                (b'A', b'X') => 3,
                (b'B', b'X') => 1,
                (b'C', b'X') => 2,
                (b'A', b'Y') => 4,
                (b'B', b'Y') => 5,
                (b'C', b'Y') => 6,
                (b'A', b'Z') => 8,
                (b'B', b'Z') => 9,
                (b'C', b'Z') => 7,
                _ => 0,
            })
            .sum();
        Ok(res.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let input = "A Y
B X
C Z
";
        let expected = "15";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let input = "A Y
B X
C Z
";
        let expected = "12";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
