use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;

const NAME: &'static str = "day11";

type Pos = (usize, usize);

fn graph1(input: &Vec<Vec<u8>>) -> HashMap<Pos, RefCell<Vec<Pos>>> {
    let dirs: [(isize, usize); 4] = [(-1, 1), (0, 1), (1, 1), (1, 0)];
    let width = input[0].len();
    let height = input.len();
    let mut res = HashMap::new();
    for y in 0..height {
        for x in 0..width {
            if input[y][x] == b'L' {
                res.entry((x, y)).or_insert_with(|| RefCell::new(vec![]));
                for (dx, dy) in dirs.iter() {
                    let yy = y + dy;
                    let xx = match (x as isize + dx).try_into() {
                        Ok(xx) => xx,
                        _ => continue,
                    };
                    if yy >= height || xx >= width {
                        continue;
                    }
                    if input[yy][xx] == b'L' {
                        res[&(x, y)].borrow_mut().push((xx, yy));
                        res.entry((xx, yy))
                            .or_insert_with(|| RefCell::new(vec![]))
                            .borrow_mut()
                            .push((x, y));
                    }
                }
            }
        }
    }
    res
}

fn graph2(input: &Vec<Vec<u8>>) -> HashMap<Pos, RefCell<Vec<Pos>>> {
    let dirs: [(isize, usize); 4] = [(-1, 1), (0, 1), (1, 1), (1, 0)];
    let width = input[0].len();
    let height = input.len();
    let mut res = HashMap::new();
    for y in 0..height {
        for x in 0..width {
            if input[y][x] == b'L' {
                res.entry((x, y)).or_insert_with(|| RefCell::new(vec![]));
                'iter: for (dx, dy) in dirs.iter() {
                    let mut yy = y;
                    let mut xx = x;

                    loop {
                        yy += dy;
                        xx = match (xx as isize + dx).try_into() {
                            Ok(xx) => xx,
                            _ => continue 'iter,
                        };

                        if yy >= height || xx >= width {
                            continue 'iter;
                        }
                        if input[yy][xx] == b'L' {
                            res[&(x, y)].borrow_mut().push((xx, yy));
                            res.entry((xx, yy))
                                .or_insert_with(|| RefCell::new(vec![]))
                                .borrow_mut()
                                .push((x, y));
                            continue 'iter;
                        }
                    }
                }
            }
        }
    }
    res
}

fn solve(mut graph: HashMap<Pos, RefCell<Vec<Pos>>>, threshold: usize) -> usize {
    let mut settled: HashSet<Pos> = HashSet::new();

    while !graph.is_empty() {
        let mut prune = vec![];
        for (pos, branches) in graph.iter_mut() {
            if branches.borrow().len() < threshold {
                settled.insert(*pos);
                prune.push(*pos);
                prune.append(&mut *branches.borrow_mut());
            }
        }
        // pruning time!
        for p in prune.iter() {
            if let Some(prunable) = graph.get(p) {
                for branch in prunable.borrow().iter() {
                    if let Some(rbranches) = graph.get(branch) {
                        let mut rbranches = rbranches.borrow_mut();
                        if let Some(pos) = rbranches.iter().position(|x| x == p) {
                            rbranches.remove(pos);
                        }
                    }
                }
            }
            graph.remove(p);
        }
    }
    settled.len()
}

day! {
    type INPUT = Vec<Vec<u8>>;

    fn gen(file: &mut impl BufRead) -> Result<Self::INPUT> {
        let mut res = String::new();
        file.read_to_string(&mut res)?;
        Ok(res.lines().map(|x| x.into()).collect())
    }

    fn part1(input: &Self::INPUT) -> Result<String> {
        Ok(format!("{}", solve(graph1(input), 4)))
    }

    fn part2(input: &Self::INPUT) -> Result<String> {
        Ok(format!("{}", solve(graph2(input), 5)))
    }
}
