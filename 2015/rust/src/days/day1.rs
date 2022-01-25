#[derive(Default)]
pub struct Day {}

impl crate::cli::Day for Day {
    type Input = String;

    fn gen(&self, data: &str) -> Self::Input {
        data.into()
    }

    fn part1(&self, input: &Self::Input) -> String {
        let res: isize = input.chars().map(|c| if c == '(' { 1 } else { -1 }).sum();
        format!("{}", res)
    }

    fn part2(&self, input: &Self::Input) -> String {
        let mut pos: isize = 0;
        let res: usize = input
            .chars()
            .position(|c| {
                pos += if c == '(' { 1 } else { -1 };
                pos == -1
            })
            .unwrap();
        format!("{}", res + 1)
    }
}
