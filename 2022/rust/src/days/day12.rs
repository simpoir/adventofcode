use std::{cmp::Reverse, collections::BinaryHeap, vec};

use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = (Vec<Vec<u8>>, (usize, usize), (usize, usize));

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        let mut start = (0, 0);
        let mut end = (0, 0);
        let grid = data
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.bytes()
                    .enumerate()
                    .map(|(x, b)| match b {
                        b'S' => {
                            start = (x, y);
                            0
                        }
                        b'E' => {
                            end = (x, y);
                            25
                        }
                        _ => b - b'a',
                    })
                    .collect()
            })
            .collect();
        Ok((grid, start, end))
    }

    fn part1(&mut self, (map, start, end): &Self::Input) -> Result<String> {
        let mut visited = vec![vec![false; map[0].len()]; map.len()];
        let mut q = BinaryHeap::new();
        q.push(Reverse((0, *start)));
        while let Some(Reverse((steps, (x, y)))) = q.pop() {
            if &(x, y) == end {
                return Ok(steps.to_string());
            }
            let steps = steps + 1;
            for (dx, dy) in crate::util::DIRS {
                // overflows. doesn't matter.
                let pos = ((x as isize + dx) as usize, (y as isize + dy) as usize);
                if let Some(tgt) = map.get(pos.1).and_then(|row| row.get(pos.0)) {
                    if !visited[pos.1][pos.0] && map[y][x] + 1 >= *tgt {
                        visited[pos.1][pos.0] = true;
                        q.push(Reverse((steps, pos)))
                    }
                }
            }
        }

        unimplemented!()
    }

    fn part2(&mut self, (map, _, end): &Self::Input) -> Result<String> {
        let mut best = usize::MAX;
        let starts = map.iter().enumerate().flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(x, h)| if *h == 0 { Some((x, y)) } else { None })
        });

        for start in starts {
            let mut visited = vec![vec![false; map[0].len()]; map.len()];
            let mut q = BinaryHeap::new();
            q.push(Reverse((0, start)));
            while let Some(Reverse((steps, (x, y)))) = q.pop() {
                if &(x, y) == end {
                    best = best.min(steps);
                }
                let steps = steps + 1;
                for (dx, dy) in crate::util::DIRS {
                    // overflows. doesn't matter.
                    let pos = ((x as isize + dx) as usize, (y as isize + dy) as usize);
                    if let Some(tgt) = map.get(pos.1).and_then(|row| row.get(pos.0)) {
                        if !visited[pos.1][pos.0] && map[y][x] + 1 >= *tgt {
                            visited[pos.1][pos.0] = true;
                            q.push(Reverse((steps, pos)))
                        }
                    }
                }
            }
        }

        Ok(best.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "31";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "29";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
