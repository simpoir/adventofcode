pub struct Day {}

impl crate::Day for Day {
    type Input = Vec<isize>;

    fn gen(&self, data: &str) -> Self::Input {
        data.trim_end()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> String {
        let mut input = input.clone();
        input.sort_unstable();
        let median: isize = input[input.len() / 2];
        let res: isize = input.iter().map(|n| (n - median).abs()).sum();

        format!("{}", res)
    }
    fn part2(&self, input: &Self::Input) -> String {
        let avg = input.iter().map(|n| *n as f32).sum::<f32>() / (input.len() as f32);
        let avg = (avg - 0.1).round();
        let res = |avg: f32| -> usize {
            input
                .iter()
                .map(|n| {
                    let x = ((*n as f32) - avg).abs();
                    x * (x + 1.0) / 2.0
                })
                .sum::<f32>() as usize
        };

        format!("{}", std::cmp::min(res(avg), res(avg + 1.0)))
    }
}

#[cfg(test)]
mod test {
    use crate::Day;

    #[test]
    fn test_more_middle_split() {
        // a case where the ceil is correct.
        let res = super::Day {}.part2(&vec![1, 10, 11, 12]);
        assert_eq!("46", res);
    }
}
