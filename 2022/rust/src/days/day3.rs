use std::collections::HashSet;

use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<Vec<usize>>;

    fn gen(&mut self, data: &'i str) -> Result<Self::Input> {
        Ok(data
            .lines()
            .map(|l| {
                l.bytes()
                    .map(|item| {
                        if item >= b'a' {
                            (item - b'a' + 1) as usize
                        } else {
                            (item - b'A' + 27) as usize
                        }
                    })
                    .collect()
            })
            .collect())
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let res: usize = input
            .iter()
            .map(|l| {
                let (first, second) = l.split_at(l.len() / 2);
                let first: HashSet<usize> = HashSet::from_iter(first.iter().copied());
                let second: HashSet<usize> = HashSet::from_iter(second.iter().copied());
                *Iterator::next(&mut first.intersection(&second)).unwrap()
            })
            .sum();

        Ok(res.to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let res: usize = input
            .chunks(3)
            .map(|group| {
                // ugh! HashSet randomstate param inference is annoying.
                let a = HashSet::<usize>::from_iter(group[0].iter().copied());
                let b = HashSet::from_iter(group[1].iter().copied());
                let c = HashSet::from_iter(group[2].iter().copied());
                let intersect: HashSet<usize> = a.intersection(&b).copied().collect();
                *intersect.intersection(&c).next().unwrap()
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
        let input = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";
        let expected = "157";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let input = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";
        let expected = "70";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
