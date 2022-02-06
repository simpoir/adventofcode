#[derive(Default)]
pub struct Day {}

impl crate::cli::Day for Day {
    type Input = ();

    fn gen(&self, data: &str) -> Self::Input {
        todo!();
    }

    fn part1(&self, _input: &Self::Input) -> String {
        "".to_string()
    }

    fn part2(&self, _input: &Self::Input) -> String {
        "".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let d: Day = Default::default();
        let input = "";
        let expected = "";
        assert_eq!(expected, d.part1(&d.gen(input)));
    }

    #[test]
    fn test_part2() {
        let d: Day = Default::default();
        let input = "";
        let expected = "";
        assert_eq!(expected, d.part2(&d.gen(input)));
    }
}
