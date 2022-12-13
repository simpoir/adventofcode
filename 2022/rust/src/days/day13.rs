use crate::cli::Result;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
#[serde(untagged)]
pub enum List {
    Num(u16),
    List(Vec<List>),
}

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<List>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        Ok(data
            .lines()
            .filter(|l| !l.is_empty())
            .map(|l| serde_json::from_str(l).unwrap())
            .collect())
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let res: usize = input
            .chunks(2)
            .enumerate()
            .filter_map(|(i, chunk)| {
                if chunk[0] < chunk[1] {
                    Some(i + 1)
                } else {
                    None
                }
            })
            .sum();
        Ok(res.to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let mut input = (*input).clone();
        let a = List::List(vec![List::List(vec![List::Num(2)])]);
        let b = List::List(vec![List::List(vec![List::Num(6)])]);
        input.push(a.clone());
        input.push(b.clone());
        input.sort();
        Ok(((input.iter().position(|x| x == &a).unwrap() + 1)
            * (1 + input.iter().position(|x| x == &b).unwrap()))
        .to_string())
    }
}

impl std::cmp::Ord for List {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl std::cmp::PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (List::Num(a), List::Num(b)) => a.partial_cmp(b),
            (List::List(a), List::List(b)) => {
                let mut a = a.iter();
                let mut b = b.iter();
                loop {
                    match (a.next(), b.next()) {
                        (None, None) => break Some(std::cmp::Ordering::Equal),
                        (Some(a), Some(b)) => match a.partial_cmp(b) {
                            Some(std::cmp::Ordering::Equal) => continue,
                            x => break x,
                        },
                        (None, Some(_)) => break Some(std::cmp::Ordering::Less),
                        (Some(_), None) => break Some(std::cmp::Ordering::Greater),
                    }
                }
            }
            (List::Num(a), List::List(_)) => List::List(vec![List::Num(*a)]).partial_cmp(other),
            (List::List(_), List::Num(b)) => self.partial_cmp(&List::List(vec![List::Num(*b)])),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
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
        let expected = "140";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
