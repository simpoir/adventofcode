use std::collections::BinaryHeap;

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
        format!("{}", solve(input))
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

        format!("{}", solve(&input))
    }
}

fn solve(input: &[Vec<isize>]) -> isize {
    let end = (input[0].len() - 1, input.len() - 1);
    let mut explored = vec![vec![false; input[0].len()]; input.len()];
    let mut explorable: BinaryHeap<(isize, (usize, usize))> = BinaryHeap::new();
    explorable.push((0, (0, 0)));
    loop {
        let here = explorable.pop().unwrap();
        let (lvl, (x, y)) = here;
        if explored[y][x] {
            continue;
        }
        explored[y][x] = true;
        if here.1 == end {
            return -here.0;
        }
        if x > 0 {
            explorable.push((lvl - input[y][x - 1], (x - 1, y)));
        }
        if y > 0 {
            explorable.push((lvl - input[y - 1][x], (x, y - 1)));
        }
        if x < end.0 {
            explorable.push((lvl - input[y][x + 1], (x + 1, y)));
        }
        if y < end.1 {
            explorable.push((lvl - input[y + 1][x], (x, y + 1)));
        }
    }
}
