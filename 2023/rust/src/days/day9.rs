use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<Vec<i64>>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        Ok(data
            .lines()
            .map(|l| {
                l.split_ascii_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect()
            })
            .collect())
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let mut res = 0;
        for line in input {
            let mut line = line.to_vec();
            // track len so we keep a nocopy chain of last values
            for j in (1..line.len()).rev() {
                for i in 0..j {
                    line[i] = line[i + 1] - line[i]
                }
            }
            res += line.iter().sum::<i64>();
        }
        Ok(res.to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let input = input
            .iter()
            .map(|l| {
                let mut l = l.to_owned();
                l.reverse();
                l
            })
            .collect();
        self.part1(&input)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "114";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "2";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
