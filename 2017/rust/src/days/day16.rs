use std::collections::HashSet;

use crate::cli::Result;

pub struct Day {
    size: u8,
    loops: u64,
}

impl Default for Day {
    fn default() -> Self {
        Self {
            size: 16,
            loops: 1_000_000_000,
        }
    }
}

pub enum Ops {
    Spin(usize),
    Exchange(usize, usize),
    Partner(u8, u8),
}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<Ops>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        data.split(',')
            .map(|op| match op.chars().next() {
                Some('s') => Ok(Ops::Spin(op.get(1..).unwrap().parse()?)),
                Some('x') => {
                    let (a, b) = op.get(1..).unwrap().split_once('/').unwrap();
                    Ok(Ops::Exchange(a.parse()?, b.parse()?))
                }
                Some('p') => {
                    let op = op.as_bytes();
                    Ok(Ops::Partner(op[1], op[3]))
                }
                n => Result::Err(anyhow::anyhow!("bad move: {n:?}")),
            })
            .collect()
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let mut dancers = mkdancers(self.size);
        input.iter().for_each(|op| {
            match op {
                Ops::Spin(n) => dancers.rotate_right(*n),
                Ops::Exchange(a, b) => dancers.swap(*a, *b),
                Ops::Partner(a, b) => {
                    let a = dancers.iter().position(|x| x == a).unwrap();
                    let b = dancers.iter().position(|x| x == b).unwrap();
                    dancers.swap(a, b)
                }
            };
        });

        Ok(String::from_utf8(dancers)?)
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let mut seen = HashSet::new();
        let mut dancers = mkdancers(self.size);
        let mut loops_at = self.loops;
        for i in 0..self.loops {
            if !seen.insert(dancers.clone()) {
                loops_at = i;
                break;
            }
            input.iter().for_each(|op| {
                match op {
                    Ops::Spin(n) => dancers.rotate_right(*n),
                    Ops::Exchange(a, b) => dancers.swap(*a, *b),
                    Ops::Partner(a, b) => {
                        let a = dancers.iter().position(|x| x == a).unwrap();
                        let b = dancers.iter().position(|x| x == b).unwrap();
                        dancers.swap(a, b)
                    }
                };
            });
        }
        for _ in 0..(self.loops % loops_at) {
            input.iter().for_each(|op| {
                match op {
                    Ops::Spin(n) => dancers.rotate_right(*n),
                    Ops::Exchange(a, b) => dancers.swap(*a, *b),
                    Ops::Partner(a, b) => {
                        let a = dancers.iter().position(|x| x == a).unwrap();
                        let b = dancers.iter().position(|x| x == b).unwrap();
                        dancers.swap(a, b)
                    }
                };
            });
        }

        Ok(String::from_utf8(dancers)?)
    }
}

fn mkdancers(size: u8) -> Vec<u8> {
    (0u8..size).map(|n| b'a' + n).collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let mut d: Day = Day { size: 5, loops: 1 };
        let data = d.gen("s1,x3/4,pe/b").unwrap();
        assert_eq!("baedc", d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Day { size: 5, loops: 2 };
        let data = d.gen("s1,x3/4,pe/b").unwrap();
        assert_eq!("ceadb", d.part2(&data).unwrap());
    }
}
