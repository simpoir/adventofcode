#[derive(Default)]
pub struct Day {}

impl crate::cli::Day for Day {
    type Input = Vec<(usize, usize, usize)>;

    fn gen(&self, data: &str) -> Self::Input {
        data.lines()
            .map(|l| {
                let mut l = l.split('x');
                (
                    l.next().unwrap().parse().unwrap(),
                    l.next().unwrap().parse().unwrap(),
                    l.next().unwrap().parse().unwrap(),
                )
            })
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> String {
        let res: usize = input
            .iter()
            .map(|b| -> usize {
                let sides = [b.0 * b.1, b.0 * b.2, b.1 * b.2];
                2 * sides.iter().sum::<usize>() + sides.iter().min().unwrap()
            })
            .sum();
        res.to_string()
    }

    fn part2(&self, input: &Self::Input) -> String {
        let res: usize = input
            .iter()
            .map(|b| -> usize {
                let sides = [b.0 + b.1, b.0 + b.2, b.1 + b.2];
                2 * sides.iter().min().unwrap() + b.0 * b.1 * b.2
            })
            .sum();
        res.to_string()
    }
}
