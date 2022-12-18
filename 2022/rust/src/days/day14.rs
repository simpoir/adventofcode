use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<Vec<(usize, usize)>>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        data.lines()
            .map(|l| {
                l.split(" -> ")
                    .map(|chunk| {
                        let (a, b) = chunk.split_once(',').unwrap();
                        Ok((a.parse()?, b.parse()?))
                    })
                    .collect()
            })
            .collect()
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let mut grid = paint(input);

        'pouring: for sand in 0.. {
            let mut x = 500;

            for y in 0..grid.len() {
                if !grid[y][x] {
                    continue;
                } else if !grid[y][x - 1] {
                    x -= 1;
                    continue;
                } else if !grid[y][x + 1] {
                    x += 1;
                    continue;
                } else {
                    grid[y - 1][x] = true;
                    continue 'pouring;
                }
            }
            return Ok(sand.to_string());
        }

        unreachable!()
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let mut grid = paint(input);
        grid.push([false; 1000]);
        grid.push([true; 1000]);

        'pouring: for sand in 0.. {
            let mut x = 500;

            for y in 0..grid.len() {
                if !grid[y][x] {
                    continue;
                } else if y == 0 {
                    return Ok(sand.to_string());
                } else if !grid[y][x - 1] {
                    x -= 1;
                    continue;
                } else if !grid[y][x + 1] {
                    x += 1;
                    continue;
                } else {
                    grid[y - 1][x] = true;
                    continue 'pouring;
                }
            }
        }

        unreachable!()
    }
}

fn paint(input: &[Vec<(usize, usize)>]) -> Vec<[bool; 1000]> {
    let max_y = input.iter().flatten().map(|(_, y)| y).max().unwrap();
    let mut grid = vec![[false; 1000]; max_y + 1];
    for l in input {
        for win in l.windows(2) {
            match win[0].0.cmp(&win[1].0) {
                std::cmp::Ordering::Less => {
                    for x in win[0].0..=win[1].0 {
                        grid[win[0].1][x] = true;
                    }
                }
                std::cmp::Ordering::Greater => {
                    for x in win[1].0..=win[0].0 {
                        grid[win[0].1][x] = true;
                    }
                }
                std::cmp::Ordering::Equal => {
                    match win[0].1.cmp(&win[1].1) {
                        std::cmp::Ordering::Less => {
                            (win[0].1..=win[1].1).for_each(|y| {
                                grid[y][win[0].0] = true;
                            });
                        }
                        std::cmp::Ordering::Greater => {
                            (win[1].1..=win[0].1).for_each(|y| {
                                grid[y][win[0].0] = true;
                            });
                        }
                        std::cmp::Ordering::Equal => (),
                    };
                }
            }
        }
    }
    grid
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "24";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "93";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
