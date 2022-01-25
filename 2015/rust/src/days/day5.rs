#[derive(Default)]
pub struct Day {}

impl crate::cli::Day for Day {
    type Input = String;

    fn gen(&self, data: &str) -> Self::Input {
        data.into()
    }

    fn part1(&self, input: &Self::Input) -> String {
        input
            .lines()
            .filter(|l| {
                l.chars()
                    .filter(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
                    .count()
                    >= 3
                    && l.as_bytes().windows(2).any(|w| w[0] == w[1])
                    && !l
                        .as_bytes()
                        .windows(2)
                        .any(|w| matches!(w, b"ab" | b"cd" | b"pq" | b"xy"))
            })
            .count()
            .to_string()
    }

    fn part2(&self, input: &Self::Input) -> String {
        input
            .lines()
            .filter(|l| {
                l.as_bytes()
                    .windows(2)
                    .enumerate()
                    .any(|(i, c)| l.as_bytes()[(i + 2)..].windows(2).any(|w| w == c))
                    && l.as_bytes().windows(3).any(|w| w[0] == w[2])
            })
            .count()
            .to_string()
    }
}
