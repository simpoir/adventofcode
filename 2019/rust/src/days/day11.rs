use super::day5::run;
use crate::cli::Result;
use std::{
    collections::{HashMap, HashSet},
    sync::mpsc::channel,
    thread::scope,
};

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<isize>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        super::day5::Day {}.gen(data)
    }

    fn part1(&mut self, code: &Self::Input) -> Result<String> {
        let mut painted = HashSet::new();
        scope(|s| {
            let mut grid = HashMap::new();
            let (in_color, input) = channel();
            let (output, color_turn) = channel();
            let mut x = 0isize;
            let mut y = 0isize;
            let mut dir = (0, 1);
            s.spawn(move || {
                run(code, input, output);
            });

            in_color.send(0).unwrap();

            while let Ok(out_color) = color_turn.recv() {
                grid.insert((x, y), out_color);
                if out_color == 1 {
                    painted.insert((x, y));
                }
                if color_turn.recv().expect("color and dir as a pair") == 0 {
                    dir = (-dir.1, dir.0)
                } else {
                    dir = (dir.1, -dir.0)
                }
                x += dir.0;
                y += dir.1;
                if in_color.send(*grid.get(&(x, y)).unwrap_or(&0)).is_err() {
                    break;
                }
            }
        });

        Ok(painted.len().to_string())
    }

    fn part2(&mut self, code: &Self::Input) -> Result<String> {
        let mut grid: HashMap<(isize, isize), isize> = HashMap::new();
        scope(|s| {
            let (in_color, input) = channel();
            let (output, color_turn) = channel();
            let mut x = 0isize;
            let mut y = 0isize;
            let mut dir = (0, -1);
            s.spawn(move || {
                run(code, input, output);
            });

            in_color.send(1).unwrap();

            while let Ok(out_color) = color_turn.recv() {
                grid.insert((x, y), out_color);
                if color_turn.recv().expect("color and dir as a pair") == 0 {
                    dir = (dir.1, -dir.0)
                } else {
                    dir = (-dir.1, dir.0)
                }
                x += dir.0;
                y += dir.1;
                if in_color.send(*grid.get(&(x, y)).unwrap_or(&0)).is_err() {
                    break;
                }
            }
        });

        let (min_x, max_x, min_y, max_y) = grid.keys().fold(
            (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
            |fold, &(x, y)| (fold.0.min(x), fold.1.max(x), fold.2.min(y), fold.3.max(y)),
        );

        let mut out = vec![vec![' '; (max_x - min_x + 1) as usize]; (max_y - min_y + 1) as usize];
        for ((x, y), color) in grid {
            if color == 1 {
                out[(y - min_y) as usize][(x - min_x) as usize] = '#';
            }
        }

        Ok(out.join(&'\n').iter().collect())
    }
}
