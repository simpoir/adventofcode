use std::collections::VecDeque;

use crate::cli::Result;

pub struct Day {
    size: usize,
}

impl<'i> Default for Day {
    fn default() -> Self {
        Day { size: 256 }
    }
}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = &'i str;

    fn gen(&mut self, data: &'i str) -> Result<Self::Input> {
        Ok(data)
    }

    fn part1(&mut self, data: &Self::Input) -> Result<String> {
        let input: Vec<usize> = data
            .split(',')
            .map(|s| Result::Ok(s.parse::<usize>()?))
            .flatten()
            .collect();
        let mut rope = (0..self.size).collect::<VecDeque<usize>>();
        let mut skip = 0;
        let mut offset = 0;
        for length in input {
            let mut buf = vec![];
            for _ in 0..length {
                buf.push(rope.pop_front().unwrap());
            }
            for x in buf.drain(..).rev() {
                rope.push_back(x);
            }
            rope.rotate_left(skip);
            offset += length + skip;
            skip += 1;
        }
        rope.rotate_right(offset % self.size);
        Ok(rope.iter().take(2).product::<usize>().to_string())
    }

    fn part2(&mut self, data: &Self::Input) -> Result<String> {
        let mut input: Vec<usize> = data.bytes().map(|b| b as usize).collect();
        input.append(&mut vec![17, 31, 73, 47, 23]);
        let mut rope = (0..self.size).collect::<VecDeque<usize>>();
        let mut skip = 0;
        let mut offset = 0;
        for _ in 0..64 {
            input.iter().for_each(|length| {
                let mut buf = vec![];
                for _ in 0..*length {
                    buf.push(rope.pop_front().unwrap());
                }
                for x in buf.drain(..).rev() {
                    rope.push_back(x);
                }
                rope.rotate_left(skip);
                offset += length + skip;
                skip = (skip + 1) % self.size;
            });
        }
        rope.rotate_right(offset % self.size);
        Ok(rope
            .iter()
            .collect::<Vec<_>>()
            .chunks(16)
            .map(|w| {
                format!(
                    "{:02x}",
                    w.iter().copied().copied().reduce(|a, b| a ^ b).unwrap()
                )
            })
            .collect())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let input = "3,4,1,5";
        let expected = "12";
        let data = d.gen(input).unwrap();
        d.size = 5;
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let input = "AoC 2017";
        let expected = "33efeb34ea91902bb2f59c9920caa6cd";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());

        let input = "1,2,3";
        let expected = "3efbe78a8d82f29979031a4aa0b16a9d";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
