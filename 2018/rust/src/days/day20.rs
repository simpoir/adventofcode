use std::collections::VecDeque;

use crate::cli::Result;

#[derive(Default)]
pub struct Day {
    part2: usize,
}

const SIZE: usize = 200;
const N: u8 = 0b0001;
const S: u8 = 0b0010;
const E: u8 = 0b0100;
const W: u8 = 0b1000;

impl<'i> crate::cli::Day<'i> for Day {
    type Input = &'i [u8];

    fn need_part1() -> bool {
        true
    }

    fn gen(&mut self, data: &'i str) -> Result<Self::Input> {
        Ok(data.as_bytes())
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let mut grid = vec![vec![0u8; SIZE]; SIZE];
        let xy0 = SIZE / 2;
        let mut x = xy0;
        let mut y = xy0;
        let mut stack = vec![];
        input.iter().for_each(|c| match *c {
            b'N' => {
                grid[y][x] |= N;
                y -= 1;
                grid[y][x] |= S;
            }
            b'S' => {
                grid[y][x] |= S;
                y += 1;
                grid[y][x] |= N;
            }
            b'E' => {
                grid[y][x] |= E;
                x += 1;
                grid[y][x] |= W;
            }
            b'W' => {
                grid[y][x] |= W;
                x -= 1;
                grid[y][x] |= E;
            }
            b'(' | b'^' => stack.push((x, y)),
            b'|' => (x, y) = *stack.last().expect("match parenthesis"),
            b')' | b'$' => (x, y) = stack.pop().expect("match parenthesis"),
            _ => (),
        });

        // cheap BFS
        let mut q = VecDeque::from([(xy0, xy0, 0)]);
        let mut best = 0;
        let mut walked = vec![vec![false; SIZE]; SIZE];
        while let Some((x, y, steps)) = q.pop_front() {
            if walked[y][x] {
                continue;
            }
            walked[y][x] = true;
            if steps >= 1000 {
                self.part2 += 1;
            }
            best = steps;
            let point = grid[y][x];
            if point & N == N {
                q.push_back((x, y - 1, steps + 1));
            }
            if point & S == S {
                q.push_back((x, y + 1, steps + 1));
            }
            if point & E == E {
                q.push_back((x + 1, y, steps + 1));
            }
            if point & W == W {
                q.push_back((x - 1, y, steps + 1));
            }
        }

        Ok(best.to_string())
    }

    fn part2(&mut self, _input: &Self::Input) -> Result<String> {
        Ok(self.part2.to_string())
    }
}

#[allow(unused)]
fn dbg_grid(grid: &[Vec<u8>]) {
    println!();
    for line in grid {
        println!(
            "{}",
            line.iter()
                .flat_map(|c| ['.', if c & E != 0 { '|' } else { '#' }])
                .collect::<String>()
        );
        println!(
            "{}",
            line.iter()
                .flat_map(|c| [if c & S != 0 { '-' } else { '#' }, '#'])
                .collect::<String>()
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let input = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$";
        let expected = "31";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }
}
