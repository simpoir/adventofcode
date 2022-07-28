use std::collections::{BinaryHeap, HashSet};

#[derive(Default)]
pub struct Day {}

#[derive(PartialEq, Eq, Copy, Clone, Hash)]
pub struct Df {
    size: u16,
    used: u16,
    available: u16,
}
impl Df {
    fn new(size: u16, used: u16, available: u16) -> Self {
        Self {
            size,
            used,
            available,
        }
    }
}
pub type Grid = [[Df; 33]; 33];

impl crate::cli::Day for Day {
    type Input = Grid;

    fn gen(&self, data: &str) -> Self::Input {
        let mut res: Grid = [[Df::new(0, 99, 0); 33]; 33];
        data.lines().skip(2).for_each(|l| {
            let mut chunks = l.split(' ').filter(|c| !c.is_empty());
            let mut pos = chunks.next().unwrap().split('-').skip(1);
            let x: usize = pos.next().unwrap()[1..].parse().unwrap();
            let y: usize = pos.next().unwrap()[1..].parse().unwrap();
            res[y][x] = Df::new(
                chunks
                    .next()
                    .unwrap()
                    .trim_end_matches('T')
                    .parse()
                    .unwrap(),
                chunks
                    .next()
                    .unwrap()
                    .trim_end_matches('T')
                    .parse()
                    .unwrap(),
                chunks
                    .next()
                    .unwrap()
                    .trim_end_matches('T')
                    .parse()
                    .unwrap(),
            );
        });
        res
    }

    fn part1(&self, input: &Self::Input) -> String {
        let mut count = 0;
        for a in input.iter().flatten() {
            for b in input.iter().flatten() {
                if can_move_to(a, b) {
                    count += 1;
                }
            }
        }
        count.to_string()
    }

    fn part2(&self, input: &Self::Input) -> String {
        let init_goal = (
            input[0]
                .iter()
                .enumerate()
                .filter(|(_, node): &(usize, &Df)| node.size > 0)
                .max_by_key(|x| x.0)
                .unwrap()
                .0,
            0,
        );
        let init_blank = {
            let mut pos = (0, 0);
            'a: for (j, a) in input.iter().enumerate() {
                for (i, b) in a.iter().enumerate() {
                    if b.used == 0 && b.size > 0 {
                        pos = (i, j);
                        break 'a;
                    }
                }
            }
            pos
        };

        let mut q = BinaryHeap::new();
        let est = h(init_blank, init_goal);
        let state = State {
            est,
            h: est,
            total: 0,
            goal: init_goal,
            blank: init_blank,
            grid: *input,
        };
        q.push(state);

        #[allow(clippy::type_complexity)]
        fn step(
            q: &mut BinaryHeap<State>,
            state: &State,
            x: usize,
            y: usize,
            visited: &mut HashSet<((usize, usize), (usize, usize))>,
        ) {
            let State {
                goal,
                est: _,
                h: _,
                total,
                blank,
                grid,
            } = state;
            let total = total + 1;
            let src = (x, y);
            let goal = if *goal == src { *blank } else { *goal };
            if can_move_to(&grid[src.1][src.0], &grid[blank.1][blank.0]) {
                let grid = mov(grid, src, *blank);
                let blank = src;
                // Nodes are either (moslty) unmovable or movable everywhere.
                // Therefore, we consider a state as been visited based on just
                // the goal and blank location.
                if !visited.insert((blank, goal)) {
                    return;
                };
                let h = h(src, goal);
                let state = State {
                    est: total + h,
                    h,
                    total,
                    goal,
                    blank,
                    grid,
                };
                q.push(state);
            }
        }

        let mut visited = HashSet::new();

        // more A*
        while let Some(state) = q.pop() {
            crate::util::progress(&state.est);
            #[cfg(test)]
            {
                dbg_grid(&state);
                dbg!(&state.total);
                dbg!(&state.est);
                dbg!(&state.h);
                println!();
            }
            if state.goal == (0, 0) {
                return state.total.to_string();
            }
            // Looping around blank won't work if nodes can merge.
            if let Some(x) = state.blank.0.checked_sub(1) {
                step(&mut q, &state, x, state.blank.1, &mut visited);
            }
            if state.blank.0 < 32 {
                step(
                    &mut q,
                    &state,
                    state.blank.0 + 1,
                    state.blank.1,
                    &mut visited,
                );
            }
            if let Some(y) = state.blank.1.checked_sub(1) {
                step(&mut q, &state, state.blank.0, y, &mut visited);
            }
            if state.blank.1 < 32 {
                step(
                    &mut q,
                    &state,
                    state.blank.0,
                    state.blank.1 + 1,
                    &mut visited,
                );
            }
        }
        unimplemented!();
    }
}

#[derive(Eq, PartialEq)]
struct State {
    est: usize,
    h: usize,
    total: usize,
    goal: (usize, usize),
    blank: (usize, usize),
    grid: Grid,
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.est.cmp(&self.est)
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.est.cmp(&self.est))
    }
}

fn h(blank: (usize, usize), goal: (usize, usize)) -> usize {
    // wrapping around the space
    6 * (goal.0 + goal.1)
        + (blank.0 as isize - goal.0 as isize).abs() as usize
        // vert is further from the goal
        + 2 * (blank.1 as isize - goal.1 as isize).abs() as usize
}

fn can_move_to(a: &Df, b: &Df) -> bool {
    a != b && a.used != 0 && a.used <= b.available
}

fn mov(grid: &Grid, src: (usize, usize), dst: (usize, usize)) -> Grid {
    let mut out = *grid;
    {
        let src = &grid[src.1][src.0];
        let dst = &mut out[dst.1][dst.0];
        dst.available -= src.used;
        dst.used += src.used;
    }
    {
        let src = &mut out[src.1][src.0];
        src.available = src.size;
        src.used = 0;
    }
    out
}

#[cfg(test)]
fn dbg_grid(state: &State) {
    state.grid.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, col)| {
            if col.used > 100 {
                print!("#");
            } else if col.used == 0 {
                print!("_");
            } else if (x, y) == state.goal {
                print!("[");
            } else {
                print!(".")
            }
        });
        println!();
    });
    println!();
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part2() {
        let d: Day = Default::default();
        let input = "#df -h
Filesystem            Size  Used  Avail  Use%
/dev/grid/node-x0-y0   10T    8T     2T   80%
/dev/grid/node-x0-y1   11T    6T     5T   54%
/dev/grid/node-x0-y2   32T   28T     4T   87%
/dev/grid/node-x1-y0    9T    7T     2T   77%
/dev/grid/node-x1-y1    8T    0T     8T    0%
/dev/grid/node-x1-y2   11T    7T     4T   63%
/dev/grid/node-x2-y0   10T    6T     4T   60%
/dev/grid/node-x2-y1    9T    8T     1T   88%
/dev/grid/node-x2-y2    9T    6T     3T   66%";
        let grid = d.gen(input);
        let expected = "7";
        assert_eq!(expected, d.part2(&grid));
    }
}
