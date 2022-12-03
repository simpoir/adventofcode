use std::{collections::HashMap, sync::mpsc::channel};

use super::day5::run;
use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<isize>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        super::day5::Day {}.gen(data)
    }

    fn part1(&mut self, code: &Self::Input) -> Result<String> {
        let mut grid = HashMap::new();
        let (output, out_data) = channel();
        std::thread::scope(|s| {
            s.spawn(|| run(code, [], output));
        });

        while let Ok(x) = out_data.recv() {
            let y = out_data.recv()?;
            let cell_type = out_data.recv()?;
            grid.insert((x, y), cell_type);
        }

        Ok(grid.values().filter(|x| **x == 2).count().to_string())
    }

    fn part2(&mut self, code: &Self::Input) -> Result<String> {
        let mem = {
            let mut code = code.to_vec();
            code[0] = 2;
            code
        };

        let mut score = 0;
        let (output, out_data) = channel();
        let (joystick, input) = channel();
        std::thread::scope(|s| {
            s.spawn(|| run(&mem, input, output));

            let mut grid = HashMap::new();
            let mut paddle = None;
            let mut ball = None;
            while let Ok(x) = out_data.recv() {
                let y = out_data.recv().expect("second arg");
                let cell_type = out_data.recv().expect("third arg");

                if x == -1 && y == 0 {
                    score = cell_type;
                }
                grid.insert((x, y), cell_type);

                match cell_type {
                    3 => paddle = Some((x, y)),
                    4 => ball = Some((x, y)),
                    _ => (),
                }
                if let (Some(paddle_pos), Some(ball_pos)) = (paddle, ball) {
                    joystick
                        .send(match paddle_pos.0.cmp(&ball_pos.0) {
                            std::cmp::Ordering::Less => 1,
                            std::cmp::Ordering::Equal => 0,
                            std::cmp::Ordering::Greater => -1,
                        })
                        .ok();
                    ball = None;
                }
            }
        });

        Ok(score.to_string())
    }
}

#[allow(unused)]
fn print_grid(grid: &HashMap<(isize, isize), isize>) {
    let (min_x, max_x, min_y, max_y) = grid.keys().fold(
        (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
        |fold, &(x, y)| (fold.0.min(x), fold.1.max(x), fold.2.min(y), fold.3.max(y)),
    );
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            match grid.get(&(x, y)) {
                Some(1) => print!("#"),
                Some(2) => print!("B"),
                Some(3) => print!("="),
                Some(4) => print!("o"),
                _ => print!(" "),
            }
        }
        println!();
    }
}
