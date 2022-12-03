use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = (usize, usize);

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        let (a, b) = data.split_once('-').unwrap();
        Ok((a.parse()?, b.parse()?))
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let mut candidates = 0;
        for i in input.0..=input.1 {
            if is_valid(i.to_string().as_bytes()) {
                candidates += 1;
            }
        }
        Ok(candidates.to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let mut candidates = 0;
        for i in input.0..=input.1 {
            let word = i.to_string();
            if is_valid(word.as_bytes()) && is_valid2(word.as_bytes()) {
                candidates += 1;
            }
        }
        Ok(candidates.to_string())
    }
}

fn is_valid(word: &[u8]) -> bool {
    let mut has_pair = false;
    word.windows(2).all(|w| {
        has_pair |= w[1] == w[0];
        w[1] >= w[0]
    }) && has_pair
}

fn is_valid2(word: &[u8]) -> bool {
    let mut prev = 0;
    let mut count = 0;
    for c in word {
        if *c != prev {
            prev = *c;
            if count == 2 {
                return true;
            }
            count = 1;
        } else {
            count += 1;
        }
    }
    count == 2
}
