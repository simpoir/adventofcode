use std::collections::BTreeSet;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<Vec<String>>;

    fn gen(&mut self, data: &str) -> Result<Self::Input, anyhow::Error> {
        Ok(data
            .lines()
            .map(|l| l.split_ascii_whitespace().map(|w| w.into()).collect())
            .collect())
    }

    fn part1(&mut self, input: &Self::Input) -> crate::cli::Output {
        Ok(input
            .iter()
            .filter(|p| p.len() == BTreeSet::from_iter(p.iter()).len())
            .count()
            .to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> crate::cli::Output {
        Ok(input
            .iter()
            .filter(|p| {
                let anagramed: BTreeSet<_> = p
                    .iter()
                    .map(|w| {
                        let mut word = w.bytes().collect::<Vec<_>>();
                        word.sort_unstable();
                        word
                    })
                    .collect();
                p.len() == anagramed.len()
            })
            .count()
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
aa bb cc
aa aaa
aa aa";
        let expected = "2";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let input = "\
ab ba
ab ac
ooo oo";
        let expected = "2";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
