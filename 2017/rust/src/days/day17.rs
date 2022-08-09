use crate::cli::Result;

pub struct Day {
    loops2: usize,
}

impl Default for Day {
    fn default() -> Self {
        Self { loops2: 50_000_000 }
    }
}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = usize;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        Ok(data.parse()?)
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let mut pos = 0;
        let mut buf = vec![0];
        for i in 1..=2017 {
            pos = (pos + input) % buf.len() + 1;
            buf.insert(pos, i);
        }
        Ok(buf[pos + 1].to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let mut pos = 0;
        let mut first_elem = 0;
        let mut size = 1;
        for i in 1..=self.loops2 {
            pos = (pos + input) % size + 1;
            size += 1;
            if pos == 1 {
                first_elem = i;
            }
        }
        Ok(first_elem.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let input = "3";
        let expected = "638";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Day { loops2: 9 };
        let input = "3";
        let expected = "9";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
