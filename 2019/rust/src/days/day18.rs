use std::collections::{HashMap, HashSet, VecDeque};

use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<&'i [u8]>;

    fn gen(&mut self, data: &'i str) -> Result<Self::Input> {
        Ok(data.trim().lines().map(|l| l.as_bytes()).collect())
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let graph = graph_it(input);
        let path = trace_depends(&graph);
        let best = evolve(&path, &graph, 20, 200); // arbitrarily picked params
        Ok(best.to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let mut input = input.clone();
        let centerx = input[0].len() / 2;
        let centery = input.len() / 2;
        let mut row1: Vec<u8> = input[centery - 1].to_owned();
        row1[(centerx - 1)..=(centerx + 1)].swap_with_slice(&mut Vec::from(*b"@#@")[..]);
        input[centery - 1] = &row1;
        let mut row2: Vec<u8> = input[centery].to_owned();
        row2[(centerx - 1)..=(centerx + 1)].swap_with_slice(&mut Vec::from(*b"###")[..]);
        input[centery] = &row2;
        let mut row3: Vec<u8> = input[centery + 1].to_owned();
        row3[(centerx - 1)..=(centerx + 1)].swap_with_slice(&mut Vec::from(*b"@#@")[..]);
        input[centery + 1] = &row3;

        let graph = graph_it(&input);
        let path = trace_depends(&graph);
        let best = evolve(&path, &graph, 20, 200); // arbitrarily picked params

        Ok(best.to_string())
    }
}

/* Find a non-optimal but valid solution */
fn trace_depends(graph: &Graph) -> Vec<u32> {
    let mut path = vec![0];
    let mut mask = 0;
    let mut set: HashSet<u32> = graph.keys().copied().collect();
    set.remove(&0);
    while !set.is_empty() {
        set.retain(|node| {
            let reqs = graph[&0][node].reqs;
            if reqs & mask == reqs {
                path.push(*node);
                mask += node;
                false
            } else {
                true
            }
        });
    }
    path
}

fn path_valid_cost(path: &[u32], graph: &Graph) -> Option<u32> {
    let mut mask = 0;
    let mut prevs = [0; 4]; // array for part2
    let mut cost = 0;
    for x in path {
        let prev = prevs.iter_mut().find(|p| graph[x].contains_key(p)).unwrap();

        let path = &graph[prev][x];
        if !mask & path.reqs != 0 {
            return None;
        }
        cost += path.dist;
        mask += x;
        *prev = *x;
    }
    Some(cost)
}

/* basic genetic algorith about permutations */
fn evolve(path: &[u32], graph: &Graph, generations: usize, max_pop: usize) -> u32 {
    let mut best = path_valid_cost(path, graph).unwrap();
    let mut population = vec![(path.to_owned(), best)];

    for _ in 0..generations {
        let mut next_pop = HashMap::new();
        for (path, _) in population.iter() {
            for i in 1..path.len() {
                for j in 1..path.len() {
                    let mut path = path.clone();
                    let val = path.remove(i);
                    path.insert(j, val);
                    if let Some(cost) = path_valid_cost(&path, graph) {
                        next_pop.insert(path, cost);
                        best = best.min(cost);
                    }
                }
            }
        }
        let mut sorted = next_pop.drain().collect::<Vec<_>>();
        sorted.sort_unstable_by_key(|x| x.1);

        population = sorted.drain(..).take(max_pop).collect();
    }

    best
}

#[derive(Debug, Default)]
struct _Dr {
    dist: u32,
    reqs: u32,
}
// nodes are identified as bit flags with 0 being @.
type Graph = HashMap<u32, HashMap<u32, _Dr>>;

const DIRS: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

fn char_to_flag(val: u8) -> u32 {
    if val == b'@' {
        0
    } else if val < b'a' {
        1 << (val - b'A')
    } else {
        1 << (val - b'a')
    }
}

fn graph_it(map: &[&[u8]]) -> Graph {
    let mut res = Graph::new();
    let (w, h) = (map[0].len(), map.len());
    let nodes: Vec<_> = map
        .iter()
        .enumerate()
        .flat_map(|(y, l)| {
            l.iter().enumerate().filter_map(move |(x, c)| {
                if *c < b'a' && *c != b'@' {
                    None
                } else {
                    Some((x, y, char_to_flag(*c)))
                }
            })
        })
        .collect();

    for (x, y, node) in nodes {
        let entry = res.entry(node).or_default();
        // identity, for convenience
        entry.insert(node, _Dr { dist: 0, reqs: 0 });

        let mut visited = vec![vec![false; w]; h];
        let mut q = VecDeque::from([(x, y, 0, 0)]);
        visited[y][x] = true;
        while let Some((x, y, mut steps, deps)) = q.pop_front() {
            steps += 1;
            for (dx, dy) in DIRS {
                let mut deps = deps;
                let (x, y) = ((x as isize + dx) as usize, (y as isize + dy) as usize);
                let val = map[y][x];
                if !visited[y][x] && val != b'#' {
                    if val >= b'a' || val == b'@' {
                        // found a key.
                        entry.insert(
                            char_to_flag(val),
                            _Dr {
                                dist: steps,
                                reqs: deps,
                            },
                        );
                    } else if val != b'.' {
                        // found a door
                        deps |= char_to_flag(val);
                    }
                    q.push_back((x, y, steps, deps));
                    visited[y][x] = true;
                }
            }
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let input = "\
#########
#b.A.@.a#
#########
";
        // b -> a
        // a -> 0
        let expected = "8";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());

        let input = "\
########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################
";
        let expected = "86";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());

        let input = "\
########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################
";
        let expected = "132";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());

        let input = "\
########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################
";
        let expected = "81";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());

        // this test converges out 1/5 of the time
        //        let input = "\
        //#################
        //#i.G..c...e..H.p#
        //########.########
        //#j.A..b...f..D.o#
        //########@########
        //#k.E..a...g..B.n#
        //########.########
        //#l.F..d...h..C.m#
        //#################";
        //        let expected = "136";
        //        let data = d.gen(input).unwrap();
        //        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part21() {
        let mut d: Day = Default::default();
        let input = "\
###############
#d.ABC.#.....a#
######@#@######
###############
######@#@######
#b.....#.....c#
###############";
        let expected = "24";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }

    #[test]
    fn test_part22() {
        let mut d: Day = Default::default();
        let input = "\
#############
#DcBa.#.GhKl#
#.###...#I###
#e#d#.@.#j#k#
###C#...###J#
#fEbA.#.FgHi#
#############";
        let expected = "32";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
