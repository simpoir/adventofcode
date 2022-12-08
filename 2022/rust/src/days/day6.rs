use std::collections::HashSet;

use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = &'i str;

    fn gen(&mut self, data: &'i str) -> Result<Self::Input> {
        Ok(data)
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        Ok((input
            .as_bytes()
            .windows(4)
            .position(|w| !w[0..3].contains(&w[3]) && !w[0..2].contains(&w[2]) && w[0] != w[1])
            .unwrap()
            + 4)
        .to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        Ok((input
            .as_bytes()
            .windows(14)
            .position(|w| HashSet::<_>::from_iter(w).len() == 14)
            .unwrap()
            + 14)
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
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let expected = "7";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        let expected = "19";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
