use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

use crate::cli::Result;

pub struct Day {
    workers: usize,
    delay: usize,
}

impl Default for Day {
    fn default() -> Self {
        Self {
            workers: 5,
            delay: 60,
        }
    }
}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = HashMap<&'i str, Vec<&'i str>>;

    fn gen(&mut self, data: &'i str) -> Result<Self::Input> {
        let mut res: Self::Input = HashMap::new();
        data.lines().for_each(|l| {
            let mut chunks = l.split_ascii_whitespace();
            res.entry(chunks.nth(1).unwrap())
                .or_default()
                .push(chunks.nth_back(2).unwrap());
        });
        Ok(res)
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let mut input = input.clone();
        let mut q = BinaryHeap::new();
        input
            .keys()
            .filter(|k| !input.values().any(|v| v.contains(k)))
            .for_each(|root| q.push(Reverse(*root)));
        let mut res = String::new();
        while let Some(k) = q.pop() {
            res.push_str(k.0);
            for child in input.remove(k.0).unwrap_or_default().iter() {
                if !input.values().any(|v| v.contains(child)) {
                    q.push(Reverse(child));
                }
            }
        }

        Ok(res)
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let mut input = input.clone();
        let mut q = BinaryHeap::new();
        input
            .keys()
            .filter(|k| !input.values().any(|v| v.contains(k)))
            .for_each(|root| q.push(Reverse(*root)));
        let mut workers = vec![0usize; self.workers];
        let mut work_node = vec![None; self.workers];
        for duration in 0.. {
            workers
                .iter_mut()
                .zip(work_node.iter_mut())
                .for_each(|(w, n)| {
                    *w = w.saturating_sub(1);
                    if *w == 0 && n.is_some() {
                        for child in input.remove(n.unwrap()).unwrap_or_default().iter() {
                            if !input.values().any(|v| v.contains(child)) {
                                q.push(Reverse(child));
                            }
                        }
                        *n = None;
                    }
                });
            workers
                .iter_mut()
                .zip(work_node.iter_mut())
                .filter(|(_, n)| n.is_none())
                .for_each(|(w, n)| {
                    if let Some(k) = q.pop() {
                        *n = Some(k.0);
                        *w = self.delay + (k.0.as_bytes()[0] + 1 - b'A') as usize;
                    }
                });
            if workers.iter().all(|w| *w == 0) {
                return Ok(duration.to_string());
            }
        }
        unimplemented!();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "\
Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "CABDFE";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Day {
            workers: 2,
            delay: 0,
        };
        let expected = "15";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
