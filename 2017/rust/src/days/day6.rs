use crate::cli::Result;
use std::collections::HashSet;

#[derive(Default)]
pub struct Day {
    p1: Option<Vec<u32>>,
}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<u32>;
    fn need_part1() -> bool {
        true
    }

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        data.split_whitespace().map(|x| Ok(x.parse()?)).collect()
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let mut state = input.clone();
        let res = loopy(&mut state)?;
        self.p1 = Some(state);
        Ok(res.to_string())
    }

    fn part2(&mut self, _input: &Self::Input) -> Result<String> {
        Ok(loopy(self.p1.as_mut().expect("part1 run"))?.to_string())
    }
}

fn loopy(state: &mut [u32]) -> Result<u32> {
    let mut seen = HashSet::new();
    let mut loops = 0;
    while seen.insert(state.to_vec()) {
        loops += 1;
        redistribute(state)?;
    }
    Ok(loops)
}

fn redistribute(banks: &mut [u32]) -> Result<()> {
    let (mut i, &x) = banks
        .iter()
        .enumerate()
        .rev()
        .max_by_key(|(_, x)| *x)
        .expect("some");
    banks[i] = 0;
    for _ in 0..x {
        i += 1;
        i %= banks.len();
        banks[i] += 1;
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let input = "0 2 7 0";
        let expected = "5";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let input = "0 2 7 0";
        let expected = "4";
        let data = d.gen(input).unwrap();
        d.part1(&data).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
