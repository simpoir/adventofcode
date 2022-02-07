#[derive(Default)]
pub struct Day {}

impl crate::cli::Day for Day {
    type Input = Vec<bool>;

    fn gen(&self, data: &str) -> Self::Input {
        data.bytes().map(|b| b == b'1').collect()
    }

    fn part1(&self, input: &Self::Input) -> String {
        run::<272>(input)
            .drain(..)
            .map(|b| if b { "1" } else { "0" })
            .collect()
    }

    fn part2(&self, input: &Self::Input) -> String {
        run::<35651584>(input)
            .drain(..)
            .map(|b| if b { "1" } else { "0" })
            .collect()
    }
}

fn run<const L: usize>(input: &[bool]) -> Vec<bool> {
    let mut data = vec![false; L];
    input.iter().enumerate().for_each(|(i, b)| data[i] = *b);
    let mut a = input.len();
    'chunk: loop {
        for (aa, ab) in (0..a).rev().zip(0..) {
            let val = !data[aa];
            if let Some(dst) = data.get_mut(ab + a + 1) {
                *dst = val;
            } else {
                break 'chunk;
            }
        }
        a = a + a + 1;
    }

    checksum(&data)
}

fn checksum(data: &[bool]) -> Vec<bool> {
    let res: Vec<_> = data.chunks_exact(2).map(|c| c[0] == c[1]).collect();
    if res.len() & 1 == 1 {
        return res;
    }
    checksum(&res)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_checksum() {
        let d: Day = Default::default();
        let expected = d.gen("100");
        assert_eq!(expected, checksum(&d.gen("110010110100")));
    }

    #[test]
    fn test_part1() {
        let d: Day = Default::default();
        let input = "10000";
        let expected = d.gen("01100");
        assert_eq!(expected, run::<20>(&d.gen(input)));
    }
}
