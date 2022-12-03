use std::sync::mpsc::channel;
use std::thread;

use super::day5::{run, Day as Day5};
use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<isize>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        Day5 {}.gen(data)
    }

    fn part1(&mut self, code: &Self::Input) -> Result<String> {
        let mut best = 0;
        crate::util::permutations(&[0isize, 1, 2, 3, 4], &mut |phases| {
            let (sender, input) = channel();
            sender.send(0).unwrap();
            let result = phases.iter().fold(input, |prev, phase| {
                let (output, receiver) = channel();
                run(code, [*phase].into_iter().chain(prev), output);
                receiver
            });
            best = best.max(result.recv().unwrap());
            true
        });
        Ok(best.to_string())
    }

    fn part2(&mut self, code: &Self::Input) -> Result<String> {
        let mut best = 0;
        crate::util::permutations(&[5isize, 6, 7, 8, 9], &mut |phases| {
            let (sender, receiver) = channel();
            sender.send(0).unwrap();
            thread::scope(|s| {
                let last = phases.iter().fold(receiver, |prev, phase| {
                    let (output, receiver) = channel();

                    s.spawn(|| {
                        run(code, [*phase].into_iter().chain(prev), output);
                    });

                    receiver
                });
                loop {
                    let val = last.recv().expect("some amp output");
                    if sender.send(val).is_err() {
                        // First thread has hung-up. This is our output.
                        best = best.max(val);
                        break;
                    }
                }
            });

            true
        });
        Ok(best.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let input = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0";
        let expected = "43210";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let input = "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,\
27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        let expected = "139629729";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
