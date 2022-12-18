use std::collections::VecDeque;

use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

const W: usize = 64;

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<(usize, usize, usize)>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        data.lines()
            .map(|l| {
                let mut chunk = l.split(',');
                Ok((
                    chunk.next().unwrap().parse()?,
                    chunk.next().unwrap().parse()?,
                    chunk.next().unwrap().parse()?,
                ))
            })
            .collect()
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let mut grid = [[[false; W]; W]; W];
        let mut faces = 0;
        for &(x, y, z) in input {
            if x == 0 || !grid[z][y][x - 1] {
                faces += 1;
            } else {
                faces -= 1;
            }
            if y == 0 || !grid[z][y - 1][x] {
                faces += 1;
            } else {
                faces -= 1;
            }
            if z == 0 || !grid[z - 1][y][x] {
                faces += 1;
            } else {
                faces -= 1;
            }
            if !grid[z + 1][y][x] {
                faces += 1;
            } else {
                faces -= 1;
            }
            if !grid[z][y + 1][x] {
                faces += 1;
            } else {
                faces -= 1;
            }
            if !grid[z][y][x + 1] {
                faces += 1;
            } else {
                faces -= 1;
            }
            grid[z][y][x] = true;
        }
        Ok(faces.to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let mut grid = [[[false; W]; W]; W];
        for &(x, y, z) in input {
            grid[z + 1][y + 1][x + 1] = true;
        }

        let mut faces = 0;
        let mut visited = [[[false; W]; W]; W];
        // inflate the exterior and count collisions;
        let mut q = VecDeque::new();
        q.push_back((0, 0, 0));
        while let Some((x, y, z)) = q.pop_front() {
            if grid[z][y][x] {
                faces += 1;
                continue;
            }
            if visited[z][y][x] {
                continue;
            }
            visited[z][y][x] = true;

            if x > 0 {
                q.push_back((x - 1, y, z));
            }
            if y > 0 {
                q.push_back((x, y - 1, z));
            }
            if z > 0 {
                q.push_back((x, y, z - 1));
            }
            if x < W - 1 {
                q.push_back((x + 1, y, z));
            }
            if y < W - 1 {
                q.push_back((x, y + 1, z));
            }
            if z < W - 1 {
                q.push_back((x, y, z + 1));
            }
        }

        Ok(faces.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "64";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "58";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
