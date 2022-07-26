#[derive(Default)]
pub struct Day {}

impl crate::cli::Day for Day {
    type Input = Vec<(usize, usize)>;

    fn gen(&self, data: &str) -> Self::Input {
        let mut res: Self::Input = data
            .lines()
            .map(|l| {
                let (a, b) = l.split_once('-').unwrap();
                (a.parse().unwrap(), b.parse().unwrap())
            })
            .collect();
        res.sort_unstable();
        res
    }

    fn part1(&self, input: &Self::Input) -> String {
        let mut prev = 0;
        for (i, j) in input {
            if i <= &prev {
                prev = j + 1;
                continue;
            }
            break;
        }
        prev.to_string()
    }

    fn part2(&self, input: &Self::Input) -> String {
        let mut prev = 0;
        let mut count = 0;
        for (i, j) in input {
            count += i.saturating_sub(prev);
            if j > &prev {
                prev = j + 1;
            }
            continue;
        }
        count += 4294967295usize.saturating_sub(prev);
        count.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let d: Day = Default::default();
        let input = "5-8
0-2
4-7";
        let expected = "3";
        assert_eq!(expected, d.part1(&d.gen(input)));
    }

    #[test]
    fn test_part2() {
        let d: Day = Default::default();
        let input = "5-8
0-2
4-7";
        let expected = "4294967288";
        assert_eq!(expected, d.part2(&d.gen(input)));
    }
}
