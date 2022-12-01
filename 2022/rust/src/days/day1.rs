use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<Vec<usize>>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        Ok(data
            .split("\n\n")
            .map(|elf| elf.split('\n').map(|n| n.parse().unwrap()).collect())
            .collect())
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let max: usize = input.iter().map(|e| e.iter().sum()).max().unwrap();

        Ok(max.to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let mut sums: Vec<usize> = input.iter().map(|e| e.iter().sum()).collect();
        sums.sort();
        Ok(sums.iter().rev().take(3).sum::<usize>().to_string())
    }
}
