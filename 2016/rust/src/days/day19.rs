use std::collections::VecDeque;

#[derive(Default)]
pub struct Day {}

impl crate::cli::Day for Day {
    type Input = usize;

    fn gen(&self, data: &str) -> Self::Input {
        data.parse().unwrap()
    }

    fn part1(&self, input: &Self::Input) -> String {
        let mut q = (1..=*input).collect::<VecDeque<_>>();
        while q.len() > 1 {
            let e = q.pop_front().unwrap();
            q.push_back(e);
            q.pop_front();
        }
        q.pop_front().unwrap().to_string()
    }

    fn part2(&self, input: &Self::Input) -> String {
        let mut q = (1..=*input).collect::<VecDeque<_>>();
        q.rotate_left((q.len()) / 2);
        while q.len() > 1 {
            q.pop_front().unwrap();
            if q.len() & 1 == 0 {
                q.rotate_left(1);
            }
        }
        q.pop_front().unwrap().to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let d: Day = Default::default();
        let input = "5";
        let expected = "3";
        assert_eq!(expected, d.part1(&d.gen(input)));
    }

    #[test]
    fn test_part2() {
        let d: Day = Default::default();
        let input = "5";
        let expected = "2";
        assert_eq!(expected, d.part2(&d.gen(input)));
    }
}
