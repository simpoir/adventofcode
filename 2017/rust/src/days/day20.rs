use std::collections::BTreeMap;

use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Triplet(i32, i32, i32);

impl std::ops::AddAssign for Triplet {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl TryFrom<&str> for Triplet {
    type Error = anyhow::Error;
    fn try_from(s: &str) -> Result<Self> {
        let mut chunks = s.split(',');
        Ok(Triplet(
            chunks.next().unwrap().parse()?,
            chunks.next().unwrap().parse()?,
            chunks.next().unwrap().parse()?,
        ))
    }
}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<(Triplet, Triplet, Triplet)>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        data.lines()
            .map(|l| {
                let mut chunks = l.split(['<', '>']);
                Ok((
                    chunks.nth(1).unwrap().try_into()?,
                    chunks.nth(1).unwrap().try_into()?,
                    chunks.nth(1).unwrap().try_into()?,
                ))
            })
            .collect()
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let mut parts = input.clone();
        for _ in 0..1000 {
            for (ref mut pos, ref mut vel, acc) in parts.iter_mut() {
                *vel += *acc;
                *pos += *vel;
            }
        }

        Ok(parts
            .iter()
            .enumerate()
            .min_by_key(|(_, p)| p.0 .0.abs() + p.0 .1.abs() + p.0 .2.abs())
            .unwrap()
            .0
            .to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let mut parts = input.clone();
        for _ in 0..1000 {
            let mut agg = BTreeMap::<Triplet, usize>::new();
            for (ref mut pos, ref mut vel, acc) in parts.iter_mut() {
                *vel += *acc;
                *pos += *vel;
                *agg.entry(*pos).or_default() += 1;
            }
            parts.retain(|x| Some(&1) == agg.get(&x.0));
        }

        Ok(parts.len().to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let input = "\
p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>";
        let expected = "0";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let input = "\
p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>
p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>
p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>
p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>";
        let expected = "1";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
