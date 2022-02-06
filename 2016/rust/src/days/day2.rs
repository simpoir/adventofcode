#[derive(Default)]
pub struct Day {}

const PAD: [[u8; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
const PAD2: [[u8; 5]; 5] = [
    [0, 0, 1, 0, 0],
    [0, 2, 3, 4, 0],
    [5, 6, 7, 8, 9],
    [0, 10, 11, 12, 0],
    [0, 0, 13, 0, 0],
];

impl crate::cli::Day for Day {
    type Input = String;

    fn gen(&self, data: &str) -> Self::Input {
        data.to_string()
    }

    fn part1(&self, input: &Self::Input) -> String {
        let mut pos = (1usize, 1usize);
        input
            .lines()
            .map(|l| {
                for b in l.bytes() {
                    match b {
                        b'L' => pos.0 = pos.0.saturating_sub(1),
                        b'R' => pos.0 = 2.min(pos.0 + 1),
                        b'U' => pos.1 = pos.1.saturating_sub(1),
                        b'D' => pos.1 = 2.min(pos.1 + 1),
                        _ => unreachable!(),
                    }
                }
                PAD[pos.1][pos.0].to_string()
            })
            .collect()
    }

    fn part2(&self, input: &Self::Input) -> String {
        let mut pos = (0usize, 2usize);
        input
            .lines()
            .map(|l| {
                for b in l.bytes() {
                    let prev_pos = pos;
                    match b {
                        b'L' => pos.0 = pos.0.saturating_sub(1),
                        b'R' => pos.0 = 4.min(pos.0 + 1),
                        b'U' => pos.1 = pos.1.saturating_sub(1),
                        b'D' => pos.1 = 4.min(pos.1 + 1),
                        _ => unreachable!(),
                    }
                    if PAD2[pos.1][pos.0] == 0 {
                        pos = prev_pos;
                    }
                }
                format!("{:X}", PAD2[pos.1][pos.0])
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let d: Day = Default::default();
        let input = "ULL
RRDDD
LURDL
UUUUD";
        let expected = "1985";
        assert_eq!(expected, d.part1(&d.gen(input)));
    }

    #[test]
    fn test_part2() {
        let d: Day = Default::default();
        let input = "ULL
RRDDD
LURDL
UUUUD";
        let expected = "5DB3";
        assert_eq!(expected, d.part2(&d.gen(input)));
    }
}
