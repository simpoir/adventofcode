use std::collections::BTreeSet;

pub struct Day {}

impl crate::Day for Day {
    type Input = Vec<Vec<isize>>;

    fn gen(&self, data: &str) -> Self::Input {
        data.trim_end()
            .lines()
            .map(|l| l.bytes().map(|c| (c - b'0') as isize).collect())
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> String {
        let end = (input[0].len() - 1, input.len() - 1);
        let mut explored = vec![vec![false; input[0].len()]; input.len()];
        let mut explorable: BTreeSet<(isize, (usize, usize))> = BTreeSet::new();
        explorable.insert((input[0][0], (0, 0)));

        loop {
            let here = *explorable.iter().min_by(|(x, _), (y, _)| x.cmp(y)).unwrap();
            if here.1 == end {
                return format!("{}", here.0 - input[0][0]);
            }
            explorable.remove(&here);
            let (lvl, (x, y)) = here;
            if explored[y][x] {
                continue;
            }
            explored[y][x] = true;
            if x > 0 {
                explorable.insert((lvl + input[y][x - 1], (x - 1, y)));
            }
            if y > 0 {
                explorable.insert((lvl + input[y - 1][x], (x, y - 1)));
            }
            if x < end.0 {
                explorable.insert((lvl + input[y][x + 1], (x + 1, y)));
            }
            if y < end.1 {
                explorable.insert((lvl + input[y + 1][x], (x, y + 1)));
            }
        }
    }
    fn part2(&self, input: &Self::Input) -> String {
        let input: Self::Input = input
            .iter()
            .map(|l| {
                (0..5)
                    .map(|i| l.iter().map(move |n| ((n - 1 + i) % 9) + 1))
                    .flatten()
                    .collect()
            })
            .collect();
        let input: Self::Input = (0..5)
            .map(|i| {
                input
                    .iter()
                    .map(move |l| l.iter().map(|n| ((n - 1 + i) % 9) + 1).collect())
            })
            .flatten()
            .collect();

        let end = (input[0].len() - 1, input.len() - 1);
        let mut explored = vec![vec![false; input[0].len()]; input.len()];
        let mut explorable: BTreeSet<(isize, (usize, usize))> = BTreeSet::new();
        explorable.insert((input[0][0], (0, 0)));

        loop {
            let here = *explorable.iter().min_by(|(x, _), (y, _)| x.cmp(y)).unwrap();
            if here.1 == end {
                return format!("{}", here.0 - input[0][0]);
            }
            explorable.remove(&here);
            let (lvl, (x, y)) = here;
            if explored[y][x] {
                continue;
            }
            explored[y][x] = true;
            if x > 0 {
                explorable.insert((lvl + input[y][x - 1], (x - 1, y)));
            }
            if y > 0 {
                explorable.insert((lvl + input[y - 1][x], (x, y - 1)));
            }
            if x < end.0 {
                explorable.insert((lvl + input[y][x + 1], (x + 1, y)));
            }
            if y < end.1 {
                explorable.insert((lvl + input[y + 1][x], (x, y + 1)));
            }
        }
    }
}
