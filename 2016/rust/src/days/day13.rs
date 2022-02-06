use std::collections::{BinaryHeap, HashSet};

#[derive(Default)]
pub struct Day {}

impl crate::cli::Day for Day {
    type Input = isize;

    fn gen(&self, data: &str) -> Self::Input {
        data.parse().unwrap()
    }

    fn part1(&self, fav: &Self::Input) -> String {
        solve(31, 39, *fav).to_string()
    }

    fn part2(&self, fav: &Self::Input) -> String {
        walk(*fav, 50).to_string()
    }
}

fn is_open(x: isize, y: isize, fav: isize) -> bool {
    x >= 0 && y >= 0 && (x * x + 3 * x + 2 * x * y + y + y * y + fav).count_ones() & 1 == 0
}

#[derive(Eq, PartialEq)]
struct State {
    est: isize,
    x: isize,
    y: isize,
    steps: usize,
}

impl std::cmp::PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match other.est.cmp(&self.est) {
            std::cmp::Ordering::Equal => other.steps.cmp(&self.steps),
            o => o,
        }
    }
}

/// more A*
fn solve(tgt_x: isize, tgt_y: isize, fav: isize) -> usize {
    let mut q = BinaryHeap::new();
    let mut visited = HashSet::new();
    q.push(State {
        x: 1,
        y: 1,
        steps: 0,
        est: (tgt_x - 1).abs() + (tgt_y - 1).abs(), // manhattan distance
    });
    while let Some(State { x, y, steps, .. }) = q.pop() {
        if !visited.insert((x, y)) {
            continue; // loop busting
        }
        if x == tgt_x && y == tgt_y {
            return steps;
        }
        let steps = steps + 1;
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let x = x + dx;
            let y = y + dy;
            if is_open(x, y, fav) {
                q.push(State {
                    x,
                    y,
                    steps,
                    est: (tgt_x - x).abs() + (tgt_y - y).abs(),
                });
            }
        }
    }
    unimplemented!();
}

fn walk(fav: isize, max: usize) -> usize {
    let mut q = BinaryHeap::new();
    let mut visited = HashSet::new();
    q.push(State {
        x: 1,
        y: 1,
        steps: 0,
        est: 0,
    });
    while let Some(State { x, y, steps, .. }) = q.pop() {
        if steps > max {
            continue;
        }
        if !visited.insert((x, y)) {
            continue; // loop busting
        }
        let steps = steps + 1;
        for (dx, dy) in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
            let x = x + dx;
            let y = y + dy;
            if is_open(x, y, fav) {
                q.push(State {
                    x,
                    y,
                    steps,
                    est: 0,
                });
            }
        }
    }
    visited.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(11, solve(7, 4, 10));
    }

    #[test]
    fn test_part2() {
        assert_eq!(5, walk(10, 2));
    }
}
