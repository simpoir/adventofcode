use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::sync::mpsc::channel;

use crate::cli::Result;
use crate::days::day5::run;

#[derive(Default)]
pub struct Day {
    res2: usize,
}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<isize>;

    fn need_part1() -> bool {
        true
    }

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        data.split(',').map(|chunk| Ok(chunk.parse()?)).collect()
    }

    fn part1(&mut self, code: &Self::Input) -> Result<String> {
        let (grid, dst) = map::<100>(code);
        let (res, _) = solve(&grid, (50, 50), dst);
        self.res2 = solve(&grid, dst, dst).1;
        Ok(res.to_string())
    }

    fn part2(&mut self, _code: &Self::Input) -> Result<String> {
        Ok(self.res2.to_string())
    }
}

// (x, y, robot)
const DIRS: [(isize, isize, isize); 4] = [(0, -1, 1), (1, 0, 4), (0, 1, 2), (-1, 0, 3)];

// walk in space to map returns a grid of walls
fn map<const S: usize>(code: &[isize]) -> ([[bool; S]; S], (usize, usize)) {
    let mut grid = [[0u8; S]; S];
    let start_pos = ((S / 2), (S / 2));
    let mut pos = start_pos;
    let mut objective = (0, 0);
    let mut res = [[false; S]; S];

    // let the robot loose
    let code = code.to_owned();
    let (input, robot_in) = channel();
    let (robot_out, output) = channel();
    std::thread::spawn(move || {
        run(&code, robot_in, robot_out);
    });

    loop {
        // pick the track we've been less on.
        let (dx, dy, dir) = DIRS
            .iter()
            .min_by_key(|(x, y, _)| {
                grid[(pos.1 as isize + *y) as usize][(pos.0 as isize + *x) as usize]
            })
            .unwrap();
        input.send(*dir).unwrap();
        let next = (
            (pos.0 as isize + dx) as usize,
            (pos.1 as isize + dy) as usize,
        );
        grid[pos.1][pos.0] += 1;
        match output.recv().unwrap() {
            0 => {
                res[next.1][next.0] = true;
                grid[next.1][next.0] = u8::MAX
            }
            1 => {
                pos = next;
            }
            2 => {
                pos = next;
                objective = pos;
            }
            _ => unimplemented!(),
        }

        // the fourth time we cross start, means we went around.
        if pos == start_pos && grid[pos.1][pos.0] >= 4 {
            break;
        }
    }
    (res, objective)
}

fn solve<const S: usize>(
    grid: &[[bool; S]; S],
    (srcx, srcy): (usize, usize),
    dst: (usize, usize),
) -> (usize, usize) {
    let mut visited = [[false; S]; S];
    let mut stack = BinaryHeap::new();
    stack.push(Reverse((0, srcx, srcy)));
    let mut steps_to_dst = 0;
    let mut steps_to_all = 0;

    while let Some(Reverse((mut steps, x, y))) = stack.pop() {
        if visited[y][x] {
            continue;
        }
        visited[y][x] = true;
        if (x, y) == dst {
            steps_to_dst = steps;
        }
        steps += 1;
        for (dx, dy, _) in DIRS {
            let x2 = (x as isize + dx) as usize;
            let y2 = (y as isize + dy) as usize;
            if grid[y2][x2] || visited[y2][x2] {
                continue;
            }
            stack.push(Reverse((steps, x2, y2)));
        }
        steps_to_all = steps;
    }
    (steps_to_dst, steps_to_all - 1)
}
