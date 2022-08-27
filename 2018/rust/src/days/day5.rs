use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<u8>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        Ok(data.into())
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        Ok(reduce(input).to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let upper = b'a' - b'A';
        Ok((b'a'..=b'z')
            .map(|f| {
                reduce(
                    &input
                        .iter()
                        .copied()
                        .filter(|c| *c != f && *c != f - upper)
                        .collect::<Vec<u8>>(),
                )
            })
            .min()
            .unwrap()
            .to_string())
    }
}

fn reduce(data: &[u8]) -> usize {
    let mut res: Vec<u8> = data.to_vec();
    let mut prev = 0;
    while let Some(pos) = res
        .windows(2)
        .skip(prev)
        .position(|w| w[0].abs_diff(w[1]) == b'a'.abs_diff(b'A'))
    {
        res.splice((prev + pos)..=(prev + pos + 1), []);
        prev = pos.saturating_sub(1);
    }
    res.len()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "dabAcCaCBAcCcaDA";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "10";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "4";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
