use std::collections::VecDeque;

use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = (usize, usize);

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        let mut chunks = data.split_ascii_whitespace();
        Ok((
            chunks.next().unwrap().parse()?,
            chunks.nth_back(1).unwrap().parse()?,
        ))
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        Ok(play(input.0, input.1).to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        Ok(play(input.0, input.1 * 100).to_string())
    }
}

fn play(players: usize, last: usize) -> usize {
    let mut scores = vec![0; players];
    let mut board = VecDeque::from_iter([0]);
    for (player, marble) in (0..players).cycle().zip(1..=last) {
        if marble % 23 != 0 {
            board.rotate_left(1);
            board.push_back(marble);
        } else {
            board.rotate_right(8);
            scores[player] += marble + board.pop_front().unwrap();
            board.rotate_left(1);
        }
    }
    *scores.iter().max().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let input = "9 players; last marble is worth 25 points";
        let expected = "32";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());

        let input = "10 players; last marble is worth 1618 points";
        let expected = "8317";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }
}
