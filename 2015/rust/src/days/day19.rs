use std::collections::BinaryHeap;
use std::{collections::HashSet, rc::Rc};

#[derive(Default)]
pub struct Day {}

impl crate::cli::Day for Day {
    type Input = String;

    fn gen(&self, data: &str) -> Self::Input {
        data.to_owned()
    }

    fn part1(&self, data: &String) -> String {
        let mut lines = data.lines().rev();
        let line = lines.next().unwrap();
        let mut h = HashSet::new();

        swaps(&mut lines.skip(1), &mut |from, to| {
            for p in line.match_indices(from) {
                // poor man's splicer
                h.insert(format!(
                    "{}{}{}",
                    &line[..p.0],
                    to,
                    &line[(p.0 + from.len())..]
                ));
            }
        });

        h.len().to_string()
    }

    fn part2(&self, data: &Self::Input) -> String {
        let map = gen_mol_set(data);
        let mut lines = data.lines();
        let expected = mol_map(lines.next_back().unwrap(), &map);
        lines.next_back();

        let mut start = vec![];
        let substs = {
            let mut v = vec![];
            swaps(&mut lines, &mut |from, to| {
                if from == "e" {
                    start.push(mol_map(to, &map));
                    return;
                }
                v.push((mol_map(from, &map)[0], mol_map(to, &map)))
            });
            v
        };

        let mut queue = BinaryHeap::new();
        let line = Rc::new(expected);
        queue.push(State {
            score: h(&line),
            line,
            steps: 0,
        });
        rev(&mut queue, &substs, &mut HashSet::new(), &start)
            .unwrap()
            .to_string()
    }
}

fn h(line: &[u8]) -> usize {
    // Shot in the dark with heuristic lengh as worst case estimate cost.
    line.len()
}

#[derive(PartialEq, Eq)]
struct State {
    score: usize,
    line: Rc<Vec<u8>>,
    steps: usize,
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.score.cmp(&self.score))
    }
}
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

fn rev(
    queue: &mut BinaryHeap<State>,
    swaps: &[(u8, Vec<u8>)],
    cache: &mut HashSet<Rc<Vec<u8>>>,
    start: &[Vec<u8>],
) -> Option<usize> {
    while let Some(state) = queue.pop() {
        let step = state.steps + 1;
        if start.contains(&state.line) {
            return Some(step);
        }
        crate::util::progress(&state.score);
        for (to, from) in swaps {
            for (idx, subslice) in state.line.windows(from.len()).enumerate() {
                if subslice == from {
                    let mut line = state.line[..idx].to_vec();
                    line.push(*to);
                    line.extend_from_slice(&state.line[(idx + from.len())..]);
                    let line = Rc::new(line);
                    if cache.insert(line.clone()) {
                        let new_state = State {
                            score: step + h(&line),
                            line,
                            steps: step,
                        };
                        queue.push(new_state);
                    }
                }
            }
        }
    }
    None
}

fn swaps<'i, I, F>(lines: &mut I, func: &mut F)
where
    I: Iterator<Item = &'i str>,
    F: FnMut(&'i str, &'i str),
{
    for l in lines {
        if l.is_empty() {
            break;
        }
        let (a, b) = l.split_once(" => ").unwrap();
        func(a, b);
    }
}

fn gen_mol_set(text: &str) -> Vec<(u8, Option<u8>)> {
    let mut res = vec![];
    let mut it = text.bytes().peekable();
    while let Some(b) = it.next() {
        if !b.is_ascii_uppercase() {
            continue;
        }
        let b2 = it.peek();
        let b2 = if let Some(b2) = b2.copied() {
            if b2.is_ascii_lowercase() {
                Some(b2)
            } else {
                None
            }
        } else {
            None
        };
        if !res.contains(&(b, b2)) {
            res.push((b, b2));
        }
    }
    res
}

/// translate mol to bytes
fn mol_map(line: &str, map: &[(u8, Option<u8>)]) -> Vec<u8> {
    let mut res = vec![];
    let mut it = line.bytes().peekable();
    while let Some(b) = it.next() {
        if !b.is_ascii_uppercase() {
            continue;
        }
        let b2 = it.peek();
        let b2 = if let Some(b2) = b2.copied() {
            if b2.is_ascii_lowercase() {
                Some(b2)
            } else {
                None
            }
        } else {
            None
        };
        res.push(map.iter().position(|x| x == &(b, b2)).unwrap() as u8);
    }
    res
}
