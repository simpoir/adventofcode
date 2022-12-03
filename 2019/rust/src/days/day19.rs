use std::sync::mpsc::channel;

use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<isize>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        data.split(',').map(|x| Ok(x.parse()?)).collect()
    }

    fn part1(&mut self, code: &Self::Input) -> Result<String> {
        let (vm_out, _out) = channel();

        let mut tot = 0;
        for x in 0..50 {
            for y in 0..50 {
                tot += crate::days::day5::run(code, [x, y], vm_out.clone());
            }
        }

        Ok(tot.to_string())
    }

    fn part2(&mut self, code: &Self::Input) -> Result<String> {
        let (vm_out, _out) = channel();

        const W: isize = 99;

        let mut x = 0;
        let mut y = W;
        loop {
            if 0 == crate::days::day5::run(code, [x, y], vm_out.clone()) {
                x += 1;
            } else {
                if 1 == crate::days::day5::run(code, [x + W, y - W], vm_out.clone()) {
                    break;
                }
                y += 1;
            }
        }

        Ok((x * 10000 - W + y).to_string())
    }
}
