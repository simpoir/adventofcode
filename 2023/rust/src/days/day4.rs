use std::collections::HashSet;

use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<(HashSet<u8>, HashSet<u8>)>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        Ok(data
            .lines()
            .map(|l| {
                let (a, b) = l.split_once(": ").unwrap().1.split_once(" | ").unwrap();
                (
                    a.split_ascii_whitespace()
                        .map(|c| c.parse().unwrap())
                        .collect(),
                    b.split_ascii_whitespace()
                        .map(|c| c.parse().unwrap())
                        .collect(),
                )
            })
            .collect())
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        Ok(input
            .iter()
            .filter_map(|(a, b)| {
                let matches = a.intersection(b).count();
                if matches > 0 {
                    Some(2u32.pow(matches as u32 - 1))
                } else {
                    None
                }
            })
            .sum::<u32>()
            .to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let mut cache = vec![0; input.len()];
        for (i, (a, b)) in input.iter().enumerate().rev() {
            let win = a.intersection(b).count();
            cache[i] = 1 + cache[i + 1..i + 1 + win].iter().sum::<u32>();
        }
        Ok(cache.iter().sum::<u32>().to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "13";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "30";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
