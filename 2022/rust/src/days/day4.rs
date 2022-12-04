use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<Vec<usize>>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        Ok(data
            .lines()
            .map(|l| l.split(&[',', '-']).map(|c| c.parse().unwrap()).collect())
            .collect())
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        Ok(input
            .iter()
            .filter(|chunk| {
                let a = chunk[0]..=chunk[1];
                let b = chunk[2]..=chunk[3];
                a.contains(&chunk[2]) && a.contains(&chunk[3])
                    || b.contains(&chunk[0]) && b.contains(&chunk[1])
            })
            .count()
            .to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        Ok(input
            .iter()
            .filter(|chunk| {
                let a = chunk[0]..=chunk[1];
                let b = chunk[2]..=chunk[3];
                a.contains(&chunk[2])
                    || a.contains(&chunk[3])
                    || b.contains(&chunk[0])
                    || b.contains(&chunk[1])
            })
            .count()
            .to_string())
    }
}
