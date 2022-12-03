use super::day5::run;
use crate::cli::Result;
use std::sync::mpsc::channel;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<isize>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        super::day5::Day {}.gen(data)
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let (snd, rcv) = channel();
        let result = run(input, [1], snd).to_string();
        let outputs = rcv.into_iter().collect::<Vec<isize>>();
        assert_eq!(1, outputs.len(), "should only output code: {:?}", outputs);
        Ok(result)
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let (snd, _rcv) = channel();
        Ok(run(input, [2], snd).to_string())
    }
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let (snd, rcv) = channel();
        run(&input, [], snd);
        let result = rcv.into_iter().collect::<Vec<isize>>();
        assert_eq!(input, result);

        let (snd, _rcv) = channel();
        let result = run(&[1102, 34915192, 34915192, 7, 4, 7, 99, 0], [], snd);
        assert_eq!(1219070632396864, result);

        let (snd, _rcv) = channel();
        let result = run(&[104, 1125899906842624, 99], [], snd);
        assert_eq!(1125899906842624, result);
    }
}
