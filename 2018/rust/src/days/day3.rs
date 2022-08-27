use crate::cli::Result;

pub struct Day {
    grid: Vec<[u16; 1000]>,
}

impl Default for Day {
    fn default() -> Self {
        Self {
            grid: vec![[0u16; 1000]; 1000],
        }
    }
}
pub struct Claim(usize, usize, usize, usize);

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<Claim>;
    fn need_part1() -> bool {
        true
    }

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        data.lines()
            .map(|l| {
                let mut chunks = l.split([' ', ',', 'x', ':']);
                Ok(Claim(
                    chunks.nth(2).unwrap().parse()?,
                    chunks.next().unwrap().parse()?,
                    chunks.nth(1).unwrap().parse()?,
                    chunks.next().unwrap().parse()?,
                ))
            })
            .collect()
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        for Claim(x, y, w, h) in input {
            for slice in self.grid.iter_mut().skip(*y).take(*h) {
                slice.iter_mut().skip(*x).take(*w).for_each(|x| *x += 1);
            }
        }
        Ok(self
            .grid
            .iter()
            .flatten()
            .filter(|&&cell| cell > 1)
            .count()
            .to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        Ok(input
            .iter()
            .enumerate()
            .find_map(|(i, Claim(x, y, w, h))| {
                for slice in self.grid.iter().skip(*y).take(*h) {
                    for x in slice.iter().skip(*x).take(*w) {
                        if *x != 1 {
                            return None;
                        }
                    }
                }
                Some(i + 1)
            })
            .unwrap()
            .to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "\
#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "4";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "3";
        let data = d.gen(INPUT).unwrap();
        d.part1(&data).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
