#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<i32>;

    fn gen(&mut self, data: &str) -> Result<Self::Input, anyhow::Error> {
        data.lines().map(|l| Ok(l.parse()?)).collect()
    }

    fn part1(&mut self, input: &Self::Input) -> crate::cli::Output {
        let mut input = input.clone();
        let mut ptr = 0;
        for step in 0.. {
            if let Some(jmp) = input.get_mut(ptr) {
                ptr = (ptr as i32 + *jmp) as usize;
                *jmp += 1;
                continue;
            } else {
                return Ok(step.to_string());
            }
        }
        panic!();
    }

    fn part2(&mut self, input: &Self::Input) -> crate::cli::Output {
        let mut input = input.clone();
        let mut ptr = 0;
        for step in 0.. {
            if let Some(jmp) = input.get_mut(ptr) {
                ptr = (ptr as i32 + *jmp) as usize;
                if *jmp >= 3 {
                    *jmp -= 1;
                } else {
                    *jmp += 1;
                }
                continue;
            } else {
                return Ok(step.to_string());
            }
        }
        panic!();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    const INPUT: &str = "\
0
3
0
1
-3";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "5";
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
