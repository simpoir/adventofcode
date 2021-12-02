pub struct Day {}

impl crate::Day for Day {
    type Input = Vec<isize>;

    fn gen(&self, data: &str) -> Self::Input {
        data.split_ascii_whitespace()
            .map(|x| x.parse::<isize>().unwrap())
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> String {
        let res: usize = input[1..]
            .iter()
            .zip(&input[..])
            .filter(|(a, b)| a > b)
            .count();

        format!("{}", res)
    }

    fn part2(&self, input: &Self::Input) -> String {
        // Inner values of avg are nulled out (overlapped), so it's basically a compare with an
        // offset.
        let res = input[3..]
            .iter()
            .zip(&input[..])
            .filter(|(a, b)| a > b)
            .count();

        format!("{}", res)
    }
}
