use crate::cli::Result;

#[derive(Default)]
pub struct Day {
    the_pipe: Vec<Vec<u8>>,
}

const N: u8 = 1;
const S: u8 = 2;
const E: u8 = 4;
const W: u8 = 8;
fn flip(n: u8) -> u8 {
    match n {
        N => S,
        S => N,
        E => W,
        W => E,
        _ => unreachable!(),
    }
}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = ((usize, usize), Vec<Vec<u8>>);

    fn need_part1() -> bool {
        true
    }

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        let mut start = (0, 0);

        let mut grid: Vec<Vec<u8>> = data
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.bytes()
                    .enumerate()
                    .map(|(x, b)| match b {
                        b'-' => E + W,
                        b'|' => N + S,
                        b'J' => N + W,
                        b'L' => N + E,
                        b'7' => W + S,
                        b'F' => E + S,
                        b'S' => {
                            start = (x, y);
                            0
                        }
                        _ => 0,
                    })
                    .collect()
            })
            .collect();

        if start.0 > 0 && grid[start.1][start.0 - 1] & E > 0 {
            grid[start.1][start.0] += W;
        };
        if grid[start.1][start.0 + 1] & W > 0 {
            grid[start.1][start.0] += E;
        };
        if start.1 > 0 && grid[start.1 - 1][start.0] & S > 0 {
            grid[start.1][start.0] += N;
        };
        if grid[start.1 + 1][start.0] & N > 0 {
            grid[start.1][start.0] += S;
        };

        Ok((start, grid))
    }

    fn part1(&mut self, (start, grid): &Self::Input) -> Result<String> {
        self.the_pipe = vec![vec![0; grid[0].len()]; grid.len()];
        self.the_pipe[start.1][start.0] = grid[start.1][start.0];

        let dirs = grid[start.1][start.0];
        let first_dir = dirs & (1 << dirs.trailing_zeros());
        let mut points = [(*start, first_dir), (*start, dirs - first_dir)];

        for step in 1.. {
            for (p, dir) in points.iter_mut() {
                match *dir {
                    N => p.1 -= 1,
                    S => p.1 += 1,
                    W => p.0 -= 1,
                    E => p.0 += 1,
                    _ => unreachable!(),
                }
                *dir = grid[p.1][p.0] - flip(*dir);
                self.the_pipe[p.1][p.0] = grid[p.1][p.0];
            }

            if points[0].0 == points[1].0 {
                return Ok(step.to_string());
            }
        }
        unreachable!()
    }

    fn part2(&mut self, _: &Self::Input) -> Result<String> {
        let mut res = 0;
        let mut points = vec![];
        for (y, l) in self.the_pipe.iter().enumerate() {
            let mut cross = None;
            let mut inside = false;
            for (x, c) in l.iter().enumerate() {
                if *c == 0 {
                    if inside {
                        res += 1;
                        points.push((x, y));
                    }
                } else if *c == N + S {
                    inside = !inside;
                } else if *c == E + W {
                } else if let Some(prev) = cross {
                    if c - W != prev {
                        inside = !inside;
                    }
                    cross = None;
                } else {
                    cross = Some(*c - E);
                }
            }
        }
        Ok(res.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...
";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "8";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let input = "JF----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";
        let expected = "8";
        let data = d.gen(input).unwrap();
        d.part1(&data).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
