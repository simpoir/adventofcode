use std::{iter::Cycle, slice::Iter};

use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = &'i str;

    fn gen(&mut self, data: &'i str) -> Result<Self::Input> {
        Ok(data)
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        Ok(fft::<100>(input).split_at(8).0.to_owned())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let offset = input.split_at(7).0.parse().unwrap();
        assert!(input.len() * 10000 / 2 <= offset);
        let tmp = input.repeat(10000);
        let mut data: Vec<u8> = tmp.split_at(offset).1.bytes().map(|b| b - b'0').collect();

        /*
         * There is this interesting phenomenon with our dataset. Since we repeat the pattern based
         * on the position, if we look at the second half of the data, we get a series of 0s
         * followed by a series of 1s, startin at the output position. Thus we can simply compute
         * from the end and add until the position of interest, since the earlier positions don't
         * affect the result.
         * e.g. 12345678
         *      00000001 -> 8
         *      00000011 -> 15
         *      00000111 -> 11
         */
        for _ in 0..100 {
            let mut prev = 0;
            data.iter_mut().rev().for_each(|x| {
                *x = (*x + prev) % 10;
                prev = *x;
            })
        }

        Ok(data.iter().take(8).map(|b| b.to_string()).collect())
    }
}

struct Pattern {
    x: usize,
    n: usize,
    inner: Cycle<Iter<'static, i16>>,
    prev: i16,
}

impl Pattern {
    fn new(n: usize) -> Self {
        const PATTERN: [i16; 4] = [0, 1, 0, -1];
        Self {
            x: usize::MAX,
            n,
            inner: PATTERN.iter().cycle(),
            prev: 0,
        }
    }
}

impl Iterator for Pattern {
    type Item = i16;
    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= self.n {
            self.x = 0;
            self.prev = *self.inner.next().unwrap();
        }
        self.x += 1;
        Some(self.prev)
    }
}

fn fft<const PHASES: usize>(input: &str) -> String {
    let mut buf: Vec<i16> = input.bytes().map(|b| (b - b'0') as i16).collect();

    for _ in 0..PHASES {
        let mut next = vec![0; buf.len()];
        for (i, dst) in next.iter_mut().enumerate() {
            *dst = Pattern::new(i + 1)
                .skip(1)
                .zip(buf.iter())
                .map(|(a, b)| a * b)
                .sum::<i16>()
                .abs()
                % 10;
        }
        buf = next;
    }
    buf.iter().map(|b| b.to_string()).collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::cli::Day as _;

    #[test]
    fn test_part1() {
        assert_eq!("01029498".to_string(), fft::<4>("12345678"));
    }

    #[test]
    fn test_part2() {
        let mut d = Day::default();
        let input = d.gen("03036732577212944063491565474664").unwrap();
        assert_eq!("84462026", d.part2(&input).unwrap());
    }
}
