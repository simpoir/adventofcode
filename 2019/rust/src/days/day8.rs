use crate::cli::Result;

pub struct Day {
    h: usize,
    w: usize,
}

impl Default for Day {
    fn default() -> Self {
        Self { h: 6, w: 25 }
    }
}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<u8>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        Ok(data.bytes().map(|b| b - b'0').collect())
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let layer = input
            .chunks(self.w * self.h)
            .map(|chunk| chunk.iter().filter(|x| **x == 0).count())
            .min()
            .unwrap();
        let (a, b) = input
            .chunks(self.w * self.h)
            .nth(layer)
            .unwrap()
            .iter()
            .fold((0, 0), |(a, b), x| match x {
                1 => (a + 1, b),
                2 => (a, b + 1),
                _ => (a, b),
            });
        Ok((a * b).to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let mut img = vec![2; self.w * self.h];
        input.chunks(self.w * self.h).for_each(|other| {
            img.iter_mut().zip(other).for_each(|(a, b)| {
                if *a == 2 {
                    *a = *b
                }
            });
        });

        Ok(img
            .chunks(self.w)
            .map(|row| {
                row.iter()
                    .map(|c| if *c == 1 { '#' } else { ' ' })
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n"))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let mut d: Day = Day { h: 2, w: 3 };
        let input = "123456789012";
        let expected = "1";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Day { h: 2, w: 2 };
        let input = "0222112222120000";
        let expected = " #\n# ";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
