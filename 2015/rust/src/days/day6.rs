#[derive(Default)]
pub struct Day {}

pub enum Op {
    On,
    Off,
    Toggle,
}

impl crate::cli::Day for Day {
    type Input = Vec<(Op, usize, usize, usize, usize)>;

    fn gen(&self, data: &str) -> Self::Input {
        data.lines()
            .map(|l| {
                let mut chunks = l.rsplitn(4, ' ');
                let part = chunks.next().unwrap().split_once(',').unwrap();
                let part21 = part.0.parse().unwrap();
                let part22 = part.1.parse().unwrap();
                let part = chunks.nth(1).unwrap().split_once(',').unwrap();
                let part11 = part.0.parse().unwrap();
                let part12 = part.1.parse().unwrap();
                let op = match chunks.next().unwrap() {
                    "turn on" => Op::On,
                    "turn off" => Op::Off,
                    _ => Op::Toggle,
                };
                (op, part11, part12, part21, part22)
            })
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> String {
        let mut grid = [[false; 1000]; 1000];
        for (op, x1, y1, x2, y2) in input {
            for row in grid.iter_mut().take(*x2 + 1).skip(*x1) {
                for item in row.iter_mut().take(*y2 + 1).skip(*y1) {
                    match op {
                        Op::On => *item = true,
                        Op::Off => *item = false,
                        Op::Toggle => *item = !*item,
                    }
                }
            }
        }
        grid.iter().flatten().filter(|c| **c).count().to_string()
    }

    fn part2(&self, input: &Self::Input) -> String {
        let mut grid = [[0usize; 1000]; 1000];
        for (op, x1, y1, x2, y2) in input {
            for row in grid.iter_mut().take(*x2 + 1).skip(*x1) {
                for item in row.iter_mut().take(*y2 + 1).skip(*y1) {
                    match op {
                        Op::On => *item += 1,
                        Op::Off => *item = item.saturating_sub(1),
                        Op::Toggle => *item += 2,
                    }
                }
            }
        }
        grid.iter().flatten().sum::<usize>().to_string()
    }
}
