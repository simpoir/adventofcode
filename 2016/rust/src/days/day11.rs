use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashSet},
    ops::Deref,
    rc::Rc,
};

#[derive(Default)]
pub struct Day {}

#[derive(Clone, Debug, Eq, Hash, PartialOrd, Ord, PartialEq)]
pub enum Thing {
    Chip(u8),
    Gen(u8),
}

// Cloning a list of vec is a terrible idea, but it does index fast for access.
type SubState = Rc<[Vec<Thing>; 4]>;

#[derive(Eq, PartialEq)]
pub struct State {
    est: usize,
    steps: usize,
    state: SubState,
    e: usize,
}

impl crate::cli::Day for Day {
    type Input = [Vec<Thing>; 4];

    fn gen(&self, data: &str) -> Self::Input {
        let mut res: Self::Input = Default::default();
        let mut elems = vec![];
        for (floor, l) in data.lines().enumerate() {
            let mut words = l.rsplit(' ').map(|w| w.trim_end_matches(&[',', '.']));
            while let Some(w) = words.next() {
                match w {
                    "microchip" => res[floor].push(Thing::Chip({
                        let elem = words.next().unwrap().split_once('-').unwrap().0;
                        if let Some(pos) = elems.iter().position(|x| x == &elem) {
                            pos as u8
                        } else {
                            elems.push(elem);
                            elems.len() as u8 - 1
                        }
                    })),
                    "generator" => res[floor].push(Thing::Gen({
                        let elem = words.next().unwrap();
                        if let Some(pos) = elems.iter().position(|x| x == &elem) {
                            pos as u8
                        } else {
                            elems.push(elem);
                            elems.len() as u8 - 1
                        }
                    })),
                    _ => (),
                }
            }
        }
        res
    }

    fn part1(&self, input: &Self::Input) -> String {
        astar(input, 0).to_string()
    }

    fn part2(&self, input: &Self::Input) -> String {
        use Thing::*;

        let mut input = input.clone();
        input[0].append(&mut vec![Chip(20), Gen(20), Chip(21), Gen(21)]);
        astar(&input, 0).to_string()
    }
}

fn h(state: &[Vec<Thing>; 4]) -> usize {
    // nb steps to 4th floor,
    state
        .iter()
        .enumerate()
        .map(|(i, v)| (3 - i) * v.len())
        .sum::<usize>()
        * 2 // wild overestimate
}

fn is_valid(state: &[Vec<Thing>; 4]) -> bool {
    use Thing::*;
    state.iter().all(|v| {
        for thing in v {
            if let Chip(n) = thing {
                if !v.contains(&Gen(*n)) && v.iter().any(|x| matches!(x, Gen(_))) {
                    return false;
                }
            }
        }
        true
    })
}

fn astar(state: &[Vec<Thing>; 4], e: usize) -> usize {
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();
    let mut state = state.clone();
    state.iter_mut().for_each(|l| l.sort());

    queue.push(Reverse(State {
        est: h(&state),
        steps: 0,
        state: Rc::new(state),
        e,
    }));
    while let Some(Reverse(state)) = queue.pop() {
        let State {
            steps: cost,
            state,
            e,
            ..
        } = state;
        // loop busting
        if !visited.insert((state.clone(), e)) {
            continue;
        }
        if state[0].is_empty() && state[1].is_empty() && state[2].is_empty() {
            return cost;
        }
        let new_cost = cost + 1;
        crate::util::progress(&cost);

        for (i, first) in state[e].iter().enumerate() {
            let mut new_state = state.deref().clone();
            new_state[e].remove(i);
            let mut dir = vec![];

            if e > 0 {
                dir.push(e - 1);
            }
            if e < 3 {
                dir.push(e + 1);
            }

            for new_e in dir {
                let mut new_state = new_state.clone();
                new_state[new_e].push(first.clone());
                new_state[new_e].sort();
                if is_valid(&new_state) {
                    queue.push(Reverse(State {
                        est: new_cost + h(&new_state),
                        steps: new_cost,
                        state: Rc::new(new_state.clone()),
                        e: new_e,
                    }));
                }
                for (i, second) in new_state[e].iter().enumerate() {
                    let mut new_state = new_state.clone();
                    new_state[e].remove(i);
                    new_state[new_e].push(second.clone());
                    new_state[new_e].sort();
                    if is_valid(&new_state) {
                        queue.push(Reverse(State {
                            est: new_cost + h(&new_state),
                            steps: new_cost,
                            state: Rc::new(new_state.clone()),
                            e: new_e,
                        }));
                    }
                }
            }
        }
    }
    unreachable!();
}

#[allow(dead_code)]
fn printify(e: usize, state: &[Vec<Thing>; 4]) {
    for (i, l) in state.iter().enumerate() {
        let mut ln = [b'.'; 10];
        for t in l {
            match t {
                Thing::Chip(n) => {
                    ln[*n as usize * 4] = b'C';
                    ln[*n as usize * 4 + 1] = *n as u8 + b'0'
                }
                Thing::Gen(n) => {
                    ln[*n as usize * 4 + 2] = b'G';
                    ln[*n as usize * 4 + 3] = *n as u8 + b'0'
                }
            }
        }
        if i == e {
            print!("E ");
        } else {
            print!("  ");
        }

        print!("{}", String::from_utf8_lossy(&ln[..]));
        println!();
    }
}

impl std::cmp::PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.est.partial_cmp(&other.est) {
            Some(std::cmp::Ordering::Equal) => self.steps.partial_cmp(&other.steps),
            e => e,
        }
    }
}

impl std::cmp::Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.est.cmp(&other.est) {
            std::cmp::Ordering::Equal => self.steps.cmp(&other.steps),
            e => e,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Thing::*;
    use super::*;
    use crate::days::Day as _;

    const INPUT: &str = "The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
The second floor contains a hydrogen generator.
The third floor contains a lithium generator.
The fourth floor contains nothing relevant.";

    #[test]
    fn test_gen() {
        let d: Day = Default::default();
        let expected = [vec![Chip(0), Chip(1)], vec![Gen(1)], vec![Gen(0)], vec![]];
        assert_eq!(expected, d.gen(INPUT));
    }

    #[test]
    fn test_solve_noop() {
        let d: Day = Default::default();
        let input = [vec![], vec![], vec![], vec![Chip(0), Gen(0)]];
        let expected = "0";
        assert_eq!(expected, &d.part1(&input));
    }

    #[test]
    fn test_valid() {
        let input = [vec![Chip(0), Chip(1), Gen(1)], vec![], vec![Gen(0)], vec![]];
        assert!(!is_valid(&input));
    }

    #[test]
    fn test_solve_trivial() {
        let d: Day = Default::default();
        let input = [vec![Chip(0)], vec![], vec![Gen(0)], vec![]];
        let expected = "3";
        assert_eq!(expected, &d.part1(&input));
    }

    #[test]
    fn test_backtrack() {
        let input = [vec![Chip(0)], vec![], vec![], vec![Gen(0)]];
        assert_eq!(6, astar(&input, 3));
    }

    #[test]
    fn test_part1() {
        let d: Day = Default::default();
        let expected = "11";
        assert_eq!(expected, d.part1(&d.gen(INPUT)));
    }
}
