#[derive(Default)]
pub struct Day {}

#[derive(Debug, PartialEq, Clone)]
pub enum Cuke {
    None,
    Down,
    Right,
}

impl crate::Day for Day {
    type Input = ((usize, usize), Vec<Cuke>);

    fn gen(&self, data: &str) -> Self::Input {
        let data = data.trim_end();
        let height = data.lines().count();
        let width = data.lines().next().unwrap().len();
        let rows = data
            .lines()
            .map(|l| {
                l.bytes().map(|b| match b {
                    b'v' => Cuke::Down,
                    b'>' => Cuke::Right,
                    _ => Cuke::None,
                })
            })
            .flatten()
            .collect();
        ((width, height), rows)
    }

    fn part1(&self, input: &Self::Input) -> String {
        let ((width, height), input) = input;
        let mut turns = 0;
        let mut part1 = input.clone();
        let mut part2 = input.clone();
        loop {
            let r = move_right(*width, *height, &part1, &mut part2);
            let d = move_down(*width, *height, &part2, &mut part1);
            turns += 1;
            if !d && !r {
                break;
            }
        }

        format!("{}", turns)
    }

    fn part2(&self, _input: &Self::Input) -> String {
        format!("")
    }
}

fn move_right(width: usize, height: usize, start: &[Cuke], end: &mut [Cuke]) -> bool {
    let mut moved = false;
    for j in 0..height {
        for i in 0..width {
            let idx = j * width + i;
            end[idx] = start[idx].clone();
        }
    }
    for j in 0..height {
        for i in 0..width {
            let next = j * width + ((i + 1) % width);
            let idx = j * width + i;
            if start[idx] == Cuke::Right && start[next] == Cuke::None {
                end[idx] = Cuke::None;
                end[next] = Cuke::Right;
                moved = true;
            }
        }
    }
    moved
}

fn move_down(width: usize, height: usize, start: &[Cuke], end: &mut [Cuke]) -> bool {
    let mut moved = false;
    for j in 0..height {
        for i in 0..width {
            let idx = j * width + i;
            end[idx] = start[idx].clone();
        }
    }
    for j in 0..height {
        for i in 0..width {
            let next = ((j + 1) % height) * width + i;
            let idx = j * width + i;
            if start[idx] == Cuke::Down && start[next] == Cuke::None {
                end[idx] = Cuke::None;
                end[next] = Cuke::Down;
                moved = true;
            }
        }
    }
    moved
}
