use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<(u64, u64)>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        let mut lines = data.lines();
        let t = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .trim_start()
            .split_ascii_whitespace()
            .map(|v| v.parse().unwrap());
        let d = lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .trim_start()
            .split_ascii_whitespace()
            .map(|v| v.parse().unwrap());
        Ok(t.into_iter().zip(d).collect())
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let mut counts = vec![];
        for &(t, d) in input {
            let half = t / 2;
            for i in half..=t {
                if i * (t - i) <= d {
                    let i = i - 1;
                    counts.push(i - (t - i) + 1);
                    break;
                }
            }
        }
        Ok(counts.iter().product::<u64>().to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let t: u64 = input
            .iter()
            .map(|v| v.0.to_string())
            .collect::<String>()
            .parse()
            .unwrap();
        let d: u64 = input
            .iter()
            .map(|v| v.1.to_string())
            .collect::<String>()
            .parse()
            .unwrap();

        // let half = t / 2;
        // Edit: not a required change, but the lowest set of factors leading to D is the square
        // root. Any factors built from T has to be higher. So this shortcut this bruteforce into a
        // single iteration.
        let half = t - (t as f64).sqrt() as u64;
        for i in half..=t {
            if i * (t - i) <= d {
                let i = i - 1;
                return Ok((i - (t - i) + 1).to_string());
            }
        }
        unimplemented!();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "288";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "71503";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
