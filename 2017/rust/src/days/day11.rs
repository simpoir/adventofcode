use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

pub enum Dir {
    N(bool),
    NE(bool),
    SE(bool),
}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<Dir>;

    fn gen(&mut self, data: &'i str) -> Result<Self::Input> {
        Ok(data
            .split(',')
            .map(|s| match s {
                "n" => Dir::N(true),
                "s" => Dir::N(false),
                "ne" => Dir::NE(true),
                "sw" => Dir::NE(false),
                "se" => Dir::SE(true),
                "nw" => Dir::SE(false),
                _ => unimplemented!("'{}'", s),
            })
            .collect())
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let (mut y2, mut x) = (0i32, 0i32);
        input.iter().for_each(|ref dir| match dir {
            Dir::N(true) => y2 += 2,
            Dir::N(false) => y2 -= 2,
            Dir::NE(true) => {
                y2 += 1;
                x += 1
            }
            Dir::NE(false) => {
                y2 -= 1;
                x -= 1
            }
            Dir::SE(true) => {
                y2 -= 1;
                x += 1
            }
            Dir::SE(false) => {
                y2 += 1;
                x -= 1
            }
        });
        Ok(dist(y2, x).to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let mut best = 0;
        let (mut y2, mut x) = (0i32, 0i32);
        input.iter().for_each(|ref dir| {
            match dir {
                Dir::N(true) => y2 += 2,
                Dir::N(false) => y2 -= 2,
                Dir::NE(true) => {
                    y2 += 1;
                    x += 1
                }
                Dir::NE(false) => {
                    y2 -= 1;
                    x -= 1
                }
                Dir::SE(true) => {
                    y2 -= 1;
                    x += 1
                }
                Dir::SE(false) => {
                    y2 += 1;
                    x -= 1
                }
            }
            best = best.max(dist(y2, x));
        });
        Ok(best.to_string())
    }
}

fn dist(y2: i32, x: i32) -> i32 {
    let mut y2 = y2.abs();
    let x = x.abs();

    let diag = x.min(y2);
    y2 = (y2 - diag) / 2;

    y2 + x
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        for (input, expected) in [
            ("ne,ne,ne", "3"),
            ("ne,ne,sw,sw", "0"),
            ("ne,ne,s,s", "2"),
            ("se,sw,se,sw,sw", "3"),
        ] {
            let data = d.gen(input).unwrap();
            assert_eq!(expected, d.part1(&data).unwrap());
        }
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        for (input, expected) in [("ne,ne,ne", "3"), ("ne,ne,sw,sw", "2")] {
            let data = d.gen(input).unwrap();
            assert_eq!(expected, d.part2(&data).unwrap());
        }
    }
}
