use std::collections::{HashMap, VecDeque};

#[derive(Default)]
pub struct Day {}

pub type Grid = Vec<Vec<bool>>;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point(usize, usize);

impl crate::cli::Day for Day {
    type Input = (Vec<Point>, HashMap<(Point, Point), usize>);

    fn gen(&self, data: &str) -> Self::Input {
        let points = (0..10)
            .filter_map(|i| {
                let i = b'0' + i;
                for (y, l) in data.lines().enumerate() {
                    if let Some(x) = l.bytes().position(|x| x == i) {
                        return Some(Point(x, y));
                    }
                }
                None
            })
            .collect();
        let grid: Grid = data
            .lines()
            .map(|l| l.bytes().map(|b| b != b'#').collect())
            .collect();

        let mut paths: HashMap<(Point, Point), usize> = HashMap::new();
        for &a in &points {
            for &b in &points {
                let k = (a, b);
                if paths.contains_key(&k) {
                    continue;
                }
                let dist = amaze(a, b, &grid);
                paths.insert(k, dist);
                paths.insert((b, a), dist);
            }
        }
        (points, paths)
    }

    fn part1(&self, (points, paths): &Self::Input) -> String {
        let mut best = 999999;
        crate::util::permutations(points, &mut |perm| {
            if perm[0] != points[0] {
                return true;
            }
            let dist = perm
                .windows(2)
                .map(|k| paths.get(&(k[0], k[1])).unwrap())
                .sum();
            best = best.min(dist);
            true
        });
        best.to_string()
    }

    fn part2(&self, (points, paths): &Self::Input) -> String {
        let mut best = 999999;
        crate::util::permutations(&points[1..], &mut |perm| {
            let perm: Vec<_> = [points[0]]
                .into_iter()
                .chain(perm.iter().copied())
                .chain([points[0]])
                .collect();
            let dist = perm
                .windows(2)
                .map(|k| paths.get(&(k[1], k[0])).unwrap())
                .sum::<usize>();
            best = best.min(dist);
            true
        });
        best.to_string()
    }
}

/// Calculate the shortest path between 2 points.
fn amaze(a: Point, b: Point, grid: &Grid) -> usize {
    // laziest bfs impl
    fn bfs(grid: &mut Grid, froms: &mut VecDeque<(Point, usize)>, tgt: Point) -> usize {
        let moves = [(1isize, 0isize), (-1, 0), (0, 1), (0, -1)];
        while let Some((from, steps)) = froms.pop_front() {
            if from == tgt {
                return steps;
            }
            grid[from.1][from.0] = false;
            moves.iter().for_each(|&(dx, dy)| {
                let x = (from.0 as isize + dx) as usize;
                let y = (from.1 as isize + dy) as usize;
                if grid[y][x] {
                    grid[y][x] = false;
                    froms.push_back((Point(x, y), steps + 1));
                }
            });
        }
        panic!("no path");
    }

    bfs(&mut grid.clone(), &mut [(a, 0)].into(), b)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let d: Day = Default::default();
        let input = "\
###########
#0.1.....2#
#.#######.#
#4.......3#
###########";
        let expected = "14";
        assert_eq!(expected, d.part1(&d.gen(input)));
    }

    #[test]
    fn test_part2() {
        let d: Day = Default::default();
        let input = "\
###########
###3#######
#0.......2#
#.#######.#
#1.......4#
###########";
        let expected = "22";
        assert_eq!(expected, d.part2(&d.gen(input)));
    }
}
