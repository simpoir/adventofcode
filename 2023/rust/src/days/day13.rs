use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<Vec<&'i [u8]>>;

    fn gen(&mut self, data: &'i str) -> Result<Self::Input> {
        Ok(data
            .split("\n\n")
            .map(|chunk| chunk.lines().map(|l| l.as_bytes()).collect())
            .collect())
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let mut result = 0;
        for block in input {
            if let Some(rows) = mirror_h(block) {
                result += 100 * rows;
            } else if let Some(cols) = mirror_v(block) {
                result += cols;
            } else {
                unimplemented!()
            }
        }
        Ok(result.to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let mut result = 0;
        for block in input {
            if let Some(rows) = mirrer_h(block) {
                result += 100 * rows;
            } else if let Some(cols) = mirrer_v(block) {
                result += cols;
            } else {
                unimplemented!()
            }
        }
        Ok(result.to_string())
    }
}

fn mirror_v(block: &[&[u8]]) -> Option<usize> {
    (1..block[1].len()).find(|&x| {
        block
            .iter()
            .all(|l| l[..x].iter().rev().zip(l[x..].iter()).all(|(a, b)| a == b))
    })
}

fn mirror_h(block: &[&[u8]]) -> Option<usize> {
    (1..block.len()).find(|&y| {
        block[..y]
            .iter()
            .rev()
            .zip(block[y..].iter())
            .all(|(a, b)| a == b)
    })
}

fn mirrer_v(block: &[&[u8]]) -> Option<usize> {
    for x in 1..block[1].len() {
        let mut errs = 0;
        let res = block.iter().all(|l| {
            l[..x].iter().rev().zip(l[x..].iter()).all(|(a, b)| {
                if a == b {
                    true
                } else {
                    errs += 1;
                    errs <= 1
                }
            })
        });
        if res && errs == 1 {
            return Some(x);
        }
    }
    None
}

fn mirrer_h(block: &[&[u8]]) -> Option<usize> {
    for y in 1..block.len() {
        let mut errs = 0;
        let res = block[..y]
            .iter()
            .rev()
            .zip(block[y..].iter())
            .all(|(a, b)| {
                a.iter().zip(b.iter()).all(|(a, b)| {
                    if a == b {
                        true
                    } else {
                        errs += 1;
                        errs <= 1
                    }
                })
            });
        if res && (errs == 1) {
            return Some(y);
        }
    }
    None
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "405";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "400";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
