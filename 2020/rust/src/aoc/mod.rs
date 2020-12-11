use crate::prelude::*;
use std::io::Read;

pub mod day10;
pub mod day11;

pub mod day1 {
    use super::*;

    pub struct Day {}

    impl Challenge for Day {
        type INPUT = Vec<u64>;
        const NAME: &'static str = "day01";

        fn gen(file: &mut impl Read) -> Result<Self::INPUT> {
            let mut res = String::new();
            file.read_to_string(&mut res)?;
            Ok(res.lines().map(|line| line.parse().unwrap()).collect())
        }

        fn part1(input: &Self::INPUT) -> Result<String> {
            for i in 0..input.len() {
                for j in i..input.len() {
                    if input[i] + input[j] == 2020 {
                        return Ok(format!("{}", input[i] * input[j]));
                    }
                }
            }
            unreachable!();
        }

        fn part2(input: &Self::INPUT) -> Result<String> {
            for i in 0..input.len() {
                for j in i..input.len() {
                    for k in i..input.len() {
                        if input[i] + input[j] + input[k] == 2020 {
                            return Ok(format!("{}", input[i] * input[j] * input[k]));
                        }
                    }
                }
            }
            unreachable!();
        }
    }

    #[test]
    fn test() {
        Day::test();
    }
}

pub mod day2 {
    use super::*;

    pub struct Day {}

    #[test]
    fn test() {
        Day::test();
    }

    impl Challenge for Day {
        const NAME: &'static str = "day02";
        type INPUT = Vec<(usize, usize, char, String)>;

        fn gen(file: &mut impl Read) -> Result<Self::INPUT> {
            let mut res = String::new();
            file.read_to_string(&mut res)?;
            Ok(res
                .lines()
                .map(|line| {
                    let dash = line.find('-').unwrap();
                    let sp = line.find(' ').unwrap();
                    (
                        line[..dash].parse().unwrap(),
                        line[dash + 1..sp].parse().unwrap(),
                        line.chars().nth(sp + 1).unwrap(),
                        line[sp + 3..].into(),
                    )
                })
                .collect())
        }

        fn part1(input: &Self::INPUT) -> Result<String> {
            Ok(format!(
                "{}",
                input
                    .iter()
                    .filter(|(min, max, c, entry)| {
                        let count = entry.chars().filter(|x| x == c).count();
                        *min <= count && count <= *max
                    })
                    .count()
            ))
        }

        fn part2(input: &Self::INPUT) -> Result<String> {
            Ok(format!(
                "{}",
                input
                    .iter()
                    .filter(|(min, max, c, entry)| {
                        (entry.chars().nth(*min).unwrap() == *c)
                            ^ (entry.chars().nth(*max).unwrap() == *c)
                    })
                    .count()
            ))
        }
    }
}

pub mod day3 {
    use super::*;

    pub struct Day {}

    #[test]
    fn test() {
        Day::test();
    }

    fn check(input: &Vec<String>, x: usize, y: usize) -> usize {
        let mut pos: (usize, usize) = (0, 0);
        let mut count = 0;
        let len = input[0].len();
        while pos.1 < input.len() {
            if input[pos.1].as_bytes()[pos.0 % len] == b'#' {
                count += 1;
            }
            pos = (pos.0 + x, pos.1 + y);
        }
        return count;
    }

    impl Challenge for Day {
        const NAME: &'static str = "day03";
        type INPUT = Vec<String>;

        fn gen(file: &mut impl Read) -> Result<Self::INPUT> {
            let mut res = String::new();
            file.read_to_string(&mut res)?;
            Ok(res.lines().map(String::from).collect())
        }

        fn part1(input: &Self::INPUT) -> Result<String> {
            Ok(format!("{}", check(&input, 3, 1)))
        }

        fn part2(input: &Self::INPUT) -> Result<String> {
            Ok(format!(
                "{}",
                [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
                    .iter()
                    .map(|(x, y)| check(input, *x, *y))
                    .fold(1, |a, b| a * b)
            ))
        }
    }
}
