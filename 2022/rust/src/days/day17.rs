use std::collections::{HashMap, VecDeque};

use crate::{cli::Result, util::progress};

#[derive(Default)]
pub struct Day {}

type Shape = [(usize, usize); 5];
static SHAPES: [Shape; 5] = [
    [(0, 0), (1, 0), (2, 0), (3, 0), (3, 0)],
    [(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
    [(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
    [(0, 0), (0, 1), (0, 2), (0, 3), (0, 3)],
    [(0, 0), (1, 0), (0, 1), (1, 1), (1, 1)],
];

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<bool>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        Ok(data.bytes().map(|b| b == b'<').collect())
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let height = sim(input, 2022);

        Ok(height.to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let height = sim(input, 1000000000000);

        Ok(height.to_string())
    }
}

fn sim(input: &[bool], rounds: usize) -> usize {
    let mut visited = HashMap::new();

    const W: usize = 7;
    const H: usize = 40;
    // I'm lazy, so I'll use a ring buf to keep things tidy.
    let mut grid: VecDeque<[bool; W]> = (0..H).map(|_| [false; W]).collect();
    grid[H - 1] = [true; W]; // floor
    let mut height = 0;
    let mut input = input.iter().cycle();
    let mut shapes = SHAPES.iter().cycle();

    let mut i = 0;
    while i < rounds {
        progress(&i);
        let is_valid = |x: usize, y: usize, shape: &Shape| -> bool {
            for (dx, dy) in shape {
                let x = x + dx;
                let y = y + dy;
                if x >= W || y < H && grid[y][x] {
                    return false;
                }
            }
            true
        };

        let shape = shapes.next().unwrap();
        let mut y: usize = H + 3;
        let mut x: usize = 2;
        loop {
            let next_x = if let Some(true) = input.next() {
                x.saturating_sub(1)
            } else {
                x + 1
            };
            if is_valid(next_x, y, shape) {
                x = next_x;
            }

            let next_y = y - 1;
            if !is_valid(x, next_y, shape) {
                break;
            }
            y = next_y;
        }
        // commit
        for &(dx, dy) in shape {
            let xx = x + dx;
            let yy = y + dy;
            if let Some(row) = grid.get_mut(yy) {
                row[xx] = true
            } else {
                grid.pop_front();
                height += 1;
                y -= 1;
                grid.push_back([false; W]);
                grid[yy - 1][xx] = true;
            };
        }
        // fast-forward cycles. Conveniently, data cycle is aligned with input.
        if let Some((prev_h, prev_i)) = visited.insert(grid.clone(), (height, i)) {
            let remaining = rounds - i;
            let cycle_len = i - prev_i;
            let cycles = remaining.div_euclid(cycle_len);
            height += (height - prev_h) * (cycles);
            i += cycles * cycle_len;
            visited.clear();
        }
        i += 1;
    }
    height
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "3068";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "1514285714288";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
