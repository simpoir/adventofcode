use crate::cli::Result;

#[derive(Default)]
pub struct Day {
    test: bool,
}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<usize>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        data.split(',').map(|chunk| Ok(chunk.parse()?)).collect()
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let mut code = input.clone();
        if !self.test {
            code[1] = 12;
            code[2] = 2;
        }
        Ok(run(&code).unwrap().to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let mut code = input.clone();
        for noun in 0..=99 {
            for verb in 0..=99 {
                code[1] = noun;
                code[2] = verb;
                if run(&code) == Some(19690720) {
                    return Ok((100 * noun + verb).to_string());
                }
            }
        }
        unimplemented!()
    }
}

fn run(code: &[usize]) -> Option<usize> {
    let mut code = code.to_vec();
    for i in (0..).step_by(4) {
        match code[i] {
            1 => {
                let tgt = code[i + 3];
                code[tgt] = code[code[i + 1]] + code[code[i + 2]];
            }
            2 => {
                let tgt = code[i + 3];
                code[tgt] = code[code[i + 1]] * code[code[i + 2]]
            }
            99 => break,
            _ => return None,
        }
    }
    Some(code[0])
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let mut d: Day = Day { test: true };
        let input = "1,1,1,4,99,5,6,0,99";
        let expected = "30";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());

        let data = d.gen("2,4,4,0,99,0").unwrap();
        assert_eq!("9801", d.part1(&data).unwrap());

        let data = d.gen("1,9,10,3,2,3,11,0,99,30,40,50").unwrap();
        assert_eq!("3500", d.part1(&data).unwrap());
    }
}
