use std::sync::mpsc::channel;

use crate::cli::Result;
use crate::days::day5::run;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<isize>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        data.split(',').map(|n| Ok(n.parse()?)).collect()
    }

    fn part1(&mut self, code: &Self::Input) -> Result<String> {
        let (code_out, out) = channel();
        let input = "\
NOT C J
NOT B T
OR T J
NOT A T
OR T J
AND D J
WALK
";
        run(code, input.bytes().map(|b| b.into()), code_out);

        for val in out {
            if val > 255 {
                return Ok(val.to_string());
            }
        }
        unimplemented!();
    }

    fn part2(&mut self, code: &Self::Input) -> Result<String> {
        let (code_out, out) = channel();
        let input = "\
NOT A T
NOT B J
OR T J
NOT C T
OR T J
AND D J
NOT E T
NOT T T
OR H T
AND T J
RUN
";
        run(code, input.bytes().map(|b| b.into()), code_out);

        for val in out {
            if val > 255 {
                return Ok(val.to_string());
            }
        }
        unimplemented!();
    }
}
