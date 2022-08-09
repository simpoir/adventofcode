use std::collections::{BTreeSet, VecDeque};

use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<Vec<usize>>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        Ok(data
            .lines()
            .map(|l| {
                l.split_once(" <-> ")
                    .unwrap()
                    .1
                    .split(", ")
                    .map(|to| to.parse().unwrap())
                    .collect()
            })
            .collect())
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let mut collection = BTreeSet::new();
        let mut q = VecDeque::new();
        q.push_back(0);
        while let Some(i) = q.pop_front() {
            if collection.insert(i) {
                input[i].iter().for_each(|x| q.push_back(*x))
            }
        }

        Ok(collection.len().to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let mut groups = 0;
        let mut remaining: BTreeSet<usize> = (0..input.len()).collect();
        let mut collection = BTreeSet::new();
        let mut q = VecDeque::new();
        while let Some(&start) = remaining.iter().next() {
            groups += 1;
            q.push_back(start);
            while let Some(i) = q.pop_front() {
                remaining.remove(&i);
                if collection.insert(i) {
                    input[i].iter().for_each(|&x| q.push_back(x))
                }
            }
        }

        Ok(groups.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "\
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "6";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "2";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
