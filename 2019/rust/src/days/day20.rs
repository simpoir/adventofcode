use std::collections::{HashMap, VecDeque};

use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

const START: (usize, usize) = (0, 0);
const END: (usize, usize) = (1, 1);

impl<'i> crate::cli::Day<'i> for Day {
    type Input = (HashMap<(usize, usize), (usize, usize)>, Vec<Vec<bool>>);

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        let mut portals: HashMap<[u8; 2], Vec<(usize, usize)>> = HashMap::new();
        let data: Vec<&[u8]> = data.lines().map(|l| l.as_bytes()).collect();
        let grid = data
            .iter()
            .enumerate()
            .map(|(y, l)| {
                l.iter()
                    .enumerate()
                    .map(|(x, b)| {
                        if *b == b'.' {
                            let port_name = if data[y - 2][x].is_ascii_uppercase()
                                && data[y - 1][x].is_ascii_uppercase()
                            {
                                Some([data[y - 2][x], data[y - 1][x]])
                            } else if data[y + 2][x].is_ascii_uppercase()
                                && data[y + 1][x].is_ascii_uppercase()
                            {
                                Some([data[y + 1][x], data[y + 2][x]])
                            } else if data[y][x - 2].is_ascii_uppercase()
                                && data[y][x - 1].is_ascii_uppercase()
                            {
                                Some([data[y][x - 2], data[y][x - 1]])
                            } else if data[y][x + 2].is_ascii_uppercase()
                                && data[y][x + 1].is_ascii_uppercase()
                            {
                                Some([data[y][x + 1], data[y][x + 2]])
                            } else {
                                None
                            };
                            if let Some(port_name) = port_name {
                                portals.entry(port_name).or_default().push((x, y));
                            }
                            true
                        } else {
                            false
                        }
                    })
                    .collect()
            })
            .collect();

        let portals = portals
            .iter()
            .flat_map(|(k, v)| {
                if k == b"AA" {
                    [(START, v[0]), (v[0], START)]
                } else if k == b"ZZ" {
                    [(END, v[0]), (v[0], END)]
                } else {
                    [(v[0], v[1]), (v[1], v[0])]
                }
            })
            .collect();

        Ok((portals, grid))
    }

    fn part1(&mut self, (portals, grid): &Self::Input) -> Result<String> {
        // the usual BFS.
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        let start = portals[&START];
        let end = portals[&END];

        let mut q = VecDeque::from([(start, 0)]);
        visited[start.1][start.0] = true;

        while let Some(((x, y), steps)) = q.pop_front() {
            let steps = steps + 1;
            for (dx, dy) in [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)] {
                let (x, y) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
                if visited[y][x] || !grid[y][x] {
                    continue;
                }
                if (x, y) == end {
                    return Ok(steps.to_string());
                }
                visited[y][x] = true;
                if let Some((x, y)) = portals.get(&(x, y)) {
                    visited[*y][*x] = true;
                    q.push_back(((*x, *y), steps + 1));
                } else {
                    q.push_back(((x, y), steps));
                }
            }
        }
        unimplemented!();
    }

    fn part2(&mut self, (portals, grid): &Self::Input) -> Result<String> {
        // the usual BFS.
        let mut visited = vec![vec![vec![false; grid[0].len()]; grid.len()]; 1000];
        let start = portals[&START];
        let end = portals[&END];
        let mut portals = portals.clone();
        portals.remove(&start);
        portals.remove(&end);

        let mut q = VecDeque::from([(start, 0, 0)]);
        visited[0][start.1][start.0] = true;

        let nesting =
            |&x, &y| x <= 3 || y <= 3 || x >= (grid[0].len() - 3) || y >= (grid.len() - 3);

        while let Some(((x, y), z, steps)) = q.pop_front() {
            let steps = steps + 1;
            for (dx, dy) in [(-1isize, 0isize), (1, 0), (0, -1), (0, 1)] {
                let (x, y) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
                if visited[z][y][x] || !grid[y][x] {
                    continue;
                }
                if z == 0 && (x, y) == end {
                    return Ok(steps.to_string());
                }
                visited[z][y][x] = true;
                if let Some((x, y)) = portals.get(&(x, y)) {
                    if nesting(x, y) {
                        visited[z + 1][*y][*x] = true;
                        q.push_back(((*x, *y), z + 1, steps + 1));
                    } else if z >= 1 {
                        visited[z - 1][*y][*x] = true;
                        q.push_back(((*x, *y), z - 1, steps + 1));
                    }
                } else {
                    q.push_back(((x, y), z, steps));
                }
            }
        }
        unimplemented!();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let input = "         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       ";
        let expected = "23";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let input = "             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     ";
        let expected = "396";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
