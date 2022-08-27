use std::collections::BTreeMap;

use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<&'i str>;

    fn gen(&mut self, data: &'i str) -> Result<Self::Input> {
        Ok(data.lines().collect())
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let res = input
            .iter()
            .map(|l| {
                let mut counts = BTreeMap::new();
                l.chars()
                    .for_each(|c| *counts.entry(c).or_insert(0usize) += 1);
                (
                    counts.values().any(|x| *x == 2) as usize,
                    counts.values().any(|x| *x == 3) as usize,
                )
            })
            .reduce(|a, b| (a.0 + b.0, a.1 + b.1))
            .unwrap();

        Ok((res.0 * res.1).to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let mut res = String::new();
        crate::util::subsets(input, &mut [""; 2], &mut |buf: &[&'i str]| {
            res = buf[0]
                .chars()
                .zip(buf[1].chars())
                .filter_map(|(a, b)| if a == b { Some(a) } else { None })
                .collect();
            res.len() != buf[0].len() - 1
        });
        Ok(res)
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
abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab";
        let expected = "12";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let input = "\
abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz";
        let expected = "fgij";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
