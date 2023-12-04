use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

const DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<&'i str>;

    fn gen(&mut self, data: &'i str) -> Result<Self::Input> {
        Ok(data.lines().collect())
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        Ok(input
            .iter()
            .map(|l| {
                let mut chars = l.chars().filter(char::is_ascii_digit);
                let first = chars.next().unwrap().to_digit(10).unwrap() * 10;
                // can overlap
                let mut chars = l.chars().filter(char::is_ascii_digit);
                let last = chars.next_back().unwrap().to_digit(10).unwrap();
                first + last
            })
            .sum::<u32>()
            .to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let parser = |v: &str| {
            let c = v.chars().next().unwrap();
            if c.is_ascii_digit() {
                return c.to_digit(10);
            }
            for (i, word) in DIGITS.iter().enumerate() {
                if v.starts_with(word) {
                    return Some(i as u32);
                }
            }
            None
        };
        Ok(input
            .iter()
            .map(|l| {
                let mut i = 0;
                let first = loop {
                    if let Some(v) = parser(&l[i..]) {
                        break v;
                    }
                    i += 1;
                };
                i = l.len() - 1;
                let last = loop {
                    if let Some(v) = parser(&l[i..]) {
                        break v;
                    }
                    i -= 1;
                };
                10 * first + last
            })
            .sum::<u32>()
            .to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "142";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        let mut d: Day = Default::default();
        let expected = "281";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
