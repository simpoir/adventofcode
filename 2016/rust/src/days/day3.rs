#[derive(Default)]
pub struct Day {}

impl crate::cli::Day for Day {
    type Input = Vec<Vec<usize>>;

    fn gen(&self, data: &str) -> Self::Input {
        data.lines()
            .map(|l| {
                l.split(' ')
                    .filter(|n| !n.is_empty())
                    .map(|n| n.parse().unwrap())
                    .collect()
            })
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> String {
        input
            .iter()
            .filter(|t| {
                let (m, max) = t.iter().enumerate().max_by_key(|x| x.1).unwrap();
                t.iter()
                    .enumerate()
                    .filter_map(|(i, tt)| if i != m { Some(tt) } else { None })
                    .sum::<usize>()
                    > *max
            })
            .count()
            .to_string()
    }

    fn part2(&self, input: &Self::Input) -> String {
        input
            .chunks(3)
            .map(|x| (0..3).map(|i| (0..3).map(|j| x[j][i]).collect::<Vec<_>>()))
            .flatten()
            .filter(|t| {
                let (m, max) = t.iter().enumerate().max_by_key(|x| x.1).unwrap();
                t.iter()
                    .enumerate()
                    .filter_map(|(i, tt)| if i != m { Some(tt) } else { None })
                    .sum::<usize>()
                    > *max
            })
            .count()
            .to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let d: Day = Default::default();
        let input = "  5  10  25\n5 12 10";
        let expected = "1";
        assert_eq!(expected, d.part1(&d.gen(input)));
    }

    #[test]
    fn test_part2() {
        let d: Day = Default::default();
        let input = "1 1 2\n2 2 4\n2 2 4";
        let expected = "3";
        assert_eq!(expected, d.part2(&d.gen(input)));
    }
}
