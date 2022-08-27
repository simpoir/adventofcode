use std::collections::BTreeSet;

use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<isize>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        data.lines()
            .map(|l| Ok(l.trim_start_matches('+').parse()?))
            .collect()
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        Ok(input.iter().sum::<isize>().to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let mut res = 0;
        let mut seen = BTreeSet::new();
        seen.insert(res);
        input.iter().cycle().any(|x| {
            res += *x;

            !seen.insert(res)
        });
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
        let input = "+1
+2
-1";
        let expected = "2";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let input = "+3
+3
+4
-2
-4";
        let expected = "10";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
