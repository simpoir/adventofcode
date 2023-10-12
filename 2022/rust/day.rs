use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = ();

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        todo!();
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        Ok("".to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        Ok("".to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
