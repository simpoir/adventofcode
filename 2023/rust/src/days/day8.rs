use std::collections::HashMap;

use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = (Vec<bool>, HashMap<&'i str, (&'i str, &'i str)>);

    fn gen(&mut self, data: &'i str) -> Result<Self::Input> {
        let mut lines = data.lines();
        let a = lines.next().unwrap().bytes().map(|b| b == b'R').collect();

        let b = lines
            .skip(1)
            .map(|l| {
                let (key, tail) = l.split_once(" = (").unwrap();
                (key, tail.trim_end_matches(')').split_once(", ").unwrap())
            })
            .collect();

        Ok((a, b))
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let (steps, map) = input;
        let mut steps = steps.iter().cycle();
        let mut pos = "AAA";
        let mut moves = 0;
        while pos != "ZZZ" {
            moves += 1;
            pos = if *steps.next().unwrap() {
                map[pos].1
            } else {
                map[pos].0
            };
        }
        Ok(moves.to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let (steps, map) = input;
        let mut moves = 1;

        for pos in map.keys().filter(|k| k.ends_with('A')) {
            let mut steps = steps.iter().cycle();
            let mut pos = *pos;
            let mut move_count = 0;
            while !pos.ends_with('Z') {
                move_count += 1;
                pos = if *steps.next().unwrap() {
                    map[pos].1
                } else {
                    map[pos].0
                };
            }
            moves = crate::util::ppcm(moves, move_count);
        }

        Ok(moves.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "6";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        const INPUT: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";
        let mut d: Day = Default::default();
        let expected = "6";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
