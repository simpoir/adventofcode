use std::{cmp::Reverse, collections::BinaryHeap};

use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = (usize, (usize, usize));

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        let mut chunks = data.split(['\n', ' ', ',']);
        Ok((
            chunks.nth(1).unwrap().parse()?,
            (
                chunks.nth(1).unwrap().parse()?,
                chunks.next().unwrap().parse()?,
            ),
        ))
    }

    fn part1(&mut self, (depth, tgt): &Self::Input) -> Result<String> {
        let mut grid = vec![vec![0usize; tgt.0 + 20]; tgt.1 + 20];

        (0..=tgt.1).for_each(|y| {
            (0..=tgt.0).for_each(|x| {
                grid[y][x] = (match (x, y) {
                    (0, 0) => 0,
                    _ if (x, y) == *tgt => 0,
                    (x, 0) => x * 16807,
                    (0, y) => y * 48271,
                    (x, y) => grid[y - 1][x] * grid[y][x - 1],
                } + *depth)
                    % 20183;
            })
        });

        Ok(grid
            .iter()
            .flatten()
            .map(|c| c % 3)
            .sum::<usize>()
            .to_string())
    }

    fn part2(&mut self, (depth, tgt): &Self::Input) -> Result<String> {
        const INC: usize = 200;
        let mut grid = vec![vec![0usize; tgt.0 + INC]; tgt.1 + INC];
        (0..grid.len()).for_each(|y| {
            (0..grid[0].len()).for_each(|x| {
                grid[y][x] = (match (x, y) {
                    (0, 0) => 0,
                    _ if (x, y) == *tgt => 0,
                    (x, 0) => x * 16807,
                    (0, y) => y * 48271,
                    (x, y) => grid[y - 1][x] * grid[y][x - 1],
                } + *depth)
                    % 20183;
            })
        });
        grid.iter_mut().flatten().for_each(|p| *p %= 3);

        let mut visited = vec![vec![[false; 3]; grid[0].len()]; grid.len()];

        let est = |x: usize, y: usize, tool| {
            (x.abs_diff(tgt.0) + y.abs_diff(tgt.1))
                + matches!(tool, Tool::Gear | Tool::Neither) as usize * 7
        };
        let mut q = BinaryHeap::from([(Reverse(est(0, 0, Tool::Torch)), 0, (0, 0, Tool::Torch))]);

        while let Some((Reverse(_est), total, (x, y, tool))) = q.pop() {
            if visited[y][x][tool as usize] {
                continue;
            }
            visited[y][x][tool as usize] = true;
            if (x, y) == *tgt {
                if matches!(tool, Tool::Torch) {
                    return Ok(total.to_string());
                } else {
                    q.push((Reverse(0), total + 7, (x, y, Tool::Torch)));
                    continue;
                }
            }
            for (dx, dy) in [(0, 1), (0, -1), (-1, 0), (1, 0)] {
                let x1: usize = if let Ok(x1) = (x as isize + dx).try_into() {
                    x1
                } else {
                    continue;
                };
                let y1: usize = if let Ok(y1) = (y as isize + dy).try_into() {
                    y1
                } else {
                    continue;
                };

                if tool_goes(grid[y1][x1], tool) {
                    let est = total + 1 + est(x1, y1, tool);
                    q.push((Reverse(est), total + 1, (x1, y1, tool)));
                }
                if let Some(tool) = alt_tool(grid[y1][x1], grid[y][x], tool) {
                    let est = total + 8 + est(x1, y1, tool);
                    q.push((Reverse(est), total + 8, (x1, y1, tool)));
                }
            }
        }
        unimplemented!()
    }
}

fn tool_goes(to: usize, tool: Tool) -> bool {
    matches!(
        (to, tool),
        (0, Tool::Gear | Tool::Torch)
            | (1, Tool::Gear | Tool::Neither)
            | (2, Tool::Torch | Tool::Neither)
    )
}

fn alt_tool(a: usize, b: usize, tool: Tool) -> Option<Tool> {
    match (a, b, tool) {
        (0 | 1, 0 | 1, Tool::Neither | Tool::Torch) => Some(Tool::Gear),
        (1 | 2, 1 | 2, Tool::Gear | Tool::Torch) => Some(Tool::Neither),
        (2 | 0, 2 | 0, Tool::Gear | Tool::Neither) => Some(Tool::Torch),
        _ => None,
    }
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Copy, Clone, Debug)]
enum Tool {
    Torch = 0,
    Gear,
    Neither,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let input = "depth: 510\ntgt: 10,10";
        let expected = "114";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let input = "depth: 510\ntgt: 10,10";
        let expected = "45";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
