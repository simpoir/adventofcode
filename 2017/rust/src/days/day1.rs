#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<u32>;

    fn gen(&mut self, data: &str) -> Result<Self::Input, anyhow::Error> {
        Ok(data.bytes().map(|b| (b - b'0') as u32).collect())
    }

    fn part1(&mut self, input: &Self::Input) -> crate::cli::Output {
        Ok((input
            .windows(2)
            .filter_map(|w| if w[0] == w[1] { Some(w[0]) } else { None })
            .sum::<u32>()
            + if input[0] == *input.last().expect("some") {
                input[0]
            } else {
                0
            })
        .to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> crate::cli::Output {
        let len = input.len();
        Ok(input
            .iter()
            .enumerate()
            .map(|(i, &n)| {
                if n == input[(i + len / 2) % len] {
                    n
                } else {
                    0
                }
            })
            .sum::<u32>()
            .to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let input = "11221";
        let expected = "4";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let input = "123123";
        let expected = "12";
        let data = &d.gen(input).unwrap();
        assert_eq!(expected, d.part2(data).unwrap());
    }
}
