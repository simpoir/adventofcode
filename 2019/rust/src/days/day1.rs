use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<usize>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        data.lines().map(|l| Ok(l.parse()?)).collect()
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        Ok(input.iter().map(|x| x / 3 - 2).sum::<usize>().to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        Ok(input
            .iter()
            .map(|&(mut x)| {
                let mut total = 0;
                while x > 0 {
                    x = (x / 3).saturating_sub(2);
                    total += x;
                }
                total
            })
            .sum::<usize>()
            .to_string())
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
        let expected = "2";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let input = "1969";
        let expected = "966";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
