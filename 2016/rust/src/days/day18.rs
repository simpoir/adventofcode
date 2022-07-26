#[derive(Default)]
pub struct Day {}

impl crate::cli::Day for Day {
    type Input = String;

    fn gen(&self, data: &str) -> Self::Input {
        data.to_string()
    }

    fn part1(&self, input: &Self::Input) -> String {
        lay::<40>(input.as_bytes()).to_string()
    }

    fn part2(&self, input: &Self::Input) -> String {
        lay::<400000>(input.as_bytes()).to_string()
    }
}

fn lay<const L: usize>(first: &[u8]) -> usize {
    let mut count = 0;
    let mut row = first.to_vec();
    for row_num in (0..L).rev() {
        crate::util::progress(&row_num);
        count += row.iter().filter(|b| **b == b'.').count();
        let mut next = vec![b'^'; first.len()];
        row.insert(0, b'.');
        row.push(b'.');
        row.windows(3).enumerate().for_each(|(i, w)| {
            if w[0] == w[2] {
                next[i] = b'.'
            }
        });
        row = next;
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let d: Day = Default::default();
        let input = ".^^.^.^^^^";
        let expected = 38;
        assert_eq!(expected, lay::<10>(d.gen(input).as_bytes()));
    }
}
