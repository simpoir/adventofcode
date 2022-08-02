#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<Vec<u32>>;

    fn gen(&mut self, data: &str) -> Result<Self::Input, anyhow::Error> {
        Ok(data
            .lines()
            .map(|l| {
                l.split_ascii_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect()
            })
            .collect())
    }

    fn part1(&mut self, input: &Self::Input) -> crate::cli::Output {
        Ok(input
            .iter()
            .map(|l| l.iter().max().expect("some") - l.iter().min().expect("some"))
            .sum::<u32>()
            .to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> crate::cli::Output {
        Ok(input
            .iter()
            .map(|l| {
                for a in l {
                    for b in l {
                        if a == b {
                            continue;
                        }
                        if a % b == 0 {
                            return a / b;
                        }
                    }
                }
                panic!();
            })
            .sum::<u32>()
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
        let input = "\
5 1 9 5
7 5 3
2 4 6 8";
        let expected = "18";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let input = "\
5 9 2 8
9 4 7 3
3 8 6 5";
        let expected = "9";
        let data = &d.gen(input).unwrap();
        assert_eq!(expected, d.part2(data).unwrap());
    }
}
