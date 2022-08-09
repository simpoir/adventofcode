use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<(usize, usize)>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        data.lines()
            .map(|l| {
                let (a, b) = l
                    .split_once('/')
                    .ok_or_else(|| anyhow::anyhow!("port is not tuple"))?;
                Ok((a.parse()?, b.parse()?))
            })
            .collect()
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let best = perm(input, 0);

        Ok(best.to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let best = longperm(input, 0);

        Ok(best.0.to_string())
    }
}

fn perm(cables: &[(usize, usize)], prev: usize) -> usize {
    cables
        .iter()
        .enumerate()
        .filter(|(_i, cable)| cable.0 == prev || cable.1 == prev)
        .map(|(i, cable)| {
            let remaining: Vec<(usize, usize)> = cables[0..i]
                .iter()
                .chain(cables[(i + 1)..].iter())
                .copied()
                .collect();
            cable.0 + cable.1 + perm(&remaining, if cable.0 == prev { cable.1 } else { cable.0 })
        })
        .max()
        .unwrap_or(0)
}

fn longperm(cables: &[(usize, usize)], prev: usize) -> (usize, usize) {
    cables
        .iter()
        .enumerate()
        .filter(|(_i, cable)| cable.0 == prev || cable.1 == prev)
        .map(|(i, cable)| {
            let remaining: Vec<(usize, usize)> = cables[0..i]
                .iter()
                .chain(cables[(i + 1)..].iter())
                .copied()
                .collect();
            let sub = longperm(&remaining, if cable.0 == prev { cable.1 } else { cable.0 });
            (cable.0 + cable.1 + sub.0, sub.1 + 1)
        })
        .max_by(|(astr, alen), (bstr, blen)| alen.cmp(blen).then_with(|| astr.cmp(bstr)))
        .unwrap_or((0, 0))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "\
0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "31";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "19";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
