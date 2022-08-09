use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<(usize, usize)>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        data.lines()
            .map(|l| {
                let (a, b) = l.split_once(": ").expect("colon separator");
                Ok((a.parse()?, b.parse()?))
            })
            .collect()
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        Ok(severity(input, 0).to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        // XXX fortunately, depths are small enough that we can brute-force
        // instead of factoring.
        for t0 in 0.. {
            if !input.iter().any(|(dt, size)| {
                let cycle = 2 * size - 2;
                (dt + t0) % cycle == 0
            }) {
                return Ok(t0.to_string());
            }
        }
        panic!()
    }
}

fn severity(fw: &[(usize, usize)], t0: usize) -> usize {
    fw.iter()
        .map(|(dt, size)| {
            let cycle = 2 * size - 2;
            if (dt + t0) % cycle == 0 {
                size * dt
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "\
0: 3
1: 2
4: 4
6: 4";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "24";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "10";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
