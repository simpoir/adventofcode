use std::collections::HashMap;

#[derive(Default)]
pub struct Day {}

impl crate::cli::Day for Day {
    type Input = String;

    fn gen(&self, data: &str) -> Self::Input {
        data.to_string()
    }

    fn part1(&self, input: &Self::Input) -> String {
        let mut counts = vec![HashMap::new(); input.lines().next().unwrap().len()];
        for l in input.lines() {
            for (i, c) in l.chars().enumerate() {
                counts[i].entry(c).and_modify(|e| *e += 1).or_insert(1);
            }
        }
        counts
            .iter()
            .map(|counts| counts.iter().max_by_key(|e| e.1).map(|e| e.0).unwrap())
            .collect()
    }

    fn part2(&self, input: &Self::Input) -> String {
        let mut counts = vec![HashMap::new(); input.lines().next().unwrap().len()];
        for l in input.lines() {
            for (i, c) in l.chars().enumerate() {
                counts[i].entry(c).and_modify(|e| *e += 1).or_insert(1);
            }
        }
        counts
            .iter()
            .map(|counts| counts.iter().min_by_key(|e| e.1).map(|e| e.0).unwrap())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    const INPUT: &str = "eedadn
drvtee
eandsr
raavrd
atevrs
tsrnev
sdttsa
rasrtv
nssdts
ntnada
svetve
tesnvt
vntsnd
vrdear
dvrsen
enarar";

    #[test]
    fn test_part1() {
        let d: Day = Default::default();
        let expected = "easter";
        assert_eq!(expected, d.part1(&d.gen(INPUT)));
    }

    #[test]
    fn test_part2() {
        let d: Day = Default::default();
        let expected = "advent";
        assert_eq!(expected, d.part2(&d.gen(INPUT)));
    }
}
