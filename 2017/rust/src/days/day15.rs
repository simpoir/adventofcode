use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<usize>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        data.lines()
            .map(|l| Ok(l.rsplit_once(' ').unwrap().1.parse()?))
            .collect()
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let mut a = input[0];
        let mut b = input[1];
        let afact = 16807;
        let bfact = 48271;

        let mut matches = 0;
        for _ in 0..40_000_000 {
            if a & 0xffff == b & 0xffff {
                matches += 1;
            }
            a = gen(a, afact);
            b = gen(b, bfact);
        }
        Ok(matches.to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let mut a = input[0];
        let mut b = input[1];
        let afact = 16807;
        let bfact = 48271;

        let mut matches = 0;
        for _ in 0..5_000_000 {
            if a & 0xffff == b & 0xffff {
                matches += 1;
            }
            loop {
                a = gen(a, afact);
                if a & 3 == 0 {
                    break;
                }
            }
            loop {
                b = gen(b, bfact);
                if b & 7 == 0 {
                    break;
                }
            }
        }
        Ok(matches.to_string())
    }
}

fn gen(num: usize, fact: usize) -> usize {
    num * fact % 2147483647
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let input = " 65\n 8921";
        let expected = "588";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let input = " 65\n 8921";
        let expected = "309";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
