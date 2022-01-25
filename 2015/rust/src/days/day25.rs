#[derive(Default)]
pub struct Day {}

const FIRST: u64 = 20151125;
const MUL: u64 = 252533;
const MOD: u64 = 33554393;

impl crate::cli::Day for Day {
    type Input = (u64, u64);

    fn gen(&self, data: &str) -> Self::Input {
        let mut l = data.rsplit(' ');
        let col = l.next().unwrap().trim_end_matches('.').parse().unwrap();
        let row = l.nth(1).unwrap().trim_end_matches(',').parse().unwrap();
        (row, col)
    }

    fn part1(&self, &(row, col): &Self::Input) -> String {
        let mut val = FIRST;
        for _ in 1..idx(col, row) {
            val = val * MUL % MOD;
        }
        val.to_string()
    }

    fn part2(&self, _input: &Self::Input) -> String {
        "".to_string()
    }
}

fn idx(col: u64, row: u64) -> u64 {
    let diag = col + row - 1;
    diag * (diag - 1) / 2 + col
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ixd() {
        assert_eq!(1, idx(1, 1));
        assert_eq!(2, idx(1, 2));
        assert_eq!(3, idx(2, 1));
        assert_eq!(9, idx(3, 2));
    }
}
