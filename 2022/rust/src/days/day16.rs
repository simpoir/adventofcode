use std::collections::{BinaryHeap, HashMap, HashSet};

use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = (HashMap<&'i str, u64>, HashMap<u64, (u32, Vec<u64>)>);

    fn gen(&mut self, data: &'i str) -> Result<Self::Input> {
        // dataset allows 64bit mapping. So we'll do that.
        let bitmap: HashMap<&'i str, u64> = data
            .lines()
            .enumerate()
            .map(|(i, l)| (l.split_ascii_whitespace().nth(1).unwrap(), 1 << i))
            .collect();
        let valves: HashMap<u64, (u32, Vec<u64>)> = data
            .lines()
            .map(|l| {
                let mut chunks = l.split(&[' ', '=', ';', ',']);
                let k = chunks.nth(1).unwrap();
                let rate: u32 = chunks.nth(3).unwrap().parse()?;
                Ok((
                    bitmap[k],
                    (rate, chunks.skip(5).step_by(2).map(|v| bitmap[v]).collect()),
                ))
            })
            .collect::<Result<_>>()?;
        Ok((bitmap, valves))
    }

    fn part1(&mut self, (bitmap, input): &Self::Input) -> Result<String> {
        let mut visited = HashSet::new();
        visited.insert((bitmap["AA"], 0, 0));
        let mut heap = BinaryHeap::new();
        heap.push((u32::MAX, 0, 0, 30, bitmap["AA"], 0u64));

        while let Some((_est, dist, flow, time_left, from, opened)) = heap.pop() {
            if _est == dist {
                // We use an optimistic heuristic, so if we are done and this
                // is the top of the heap, this has to be the optimal path.
                return Ok(dist.to_string());
            }

            let estimate = |time_left: u32, opened: u64| -> u32 {
                let mut remainings: Vec<_> = input
                    .iter()
                    .filter_map(|(valve, (flow, _))| {
                        if valve & opened == 0 {
                            Some(*flow)
                        } else {
                            None
                        }
                    })
                    .collect();
                remainings.sort();
                remainings
                    .iter()
                    .rev()
                    .enumerate()
                    .map(|(i, f)| f * time_left.saturating_sub(2 * i as u32 + 1))
                    .sum::<u32>()
            };

            let (valve_flow, connections) = &input[&from];
            let time_left = time_left - 1;
            let dist = dist + flow;
            if time_left == 0 {
                heap.push((dist, dist, flow, time_left, from, opened));
                continue;
            }
            let est = dist + estimate(time_left, opened) + time_left * flow;
            for &next in connections {
                if visited.insert((next, est, opened)) {
                    heap.push((est, dist, flow, time_left, next, opened));
                }
            }
            if *valve_flow > 0 && opened & from == 0 {
                // this one could be opened
                let opened = opened | from;
                let flow = flow + *valve_flow;
                let est = dist + estimate(time_left, opened) + flow * time_left;
                if visited.insert((from, est, opened)) {
                    heap.push((est, dist, flow, time_left, from, opened));
                }
            }
        }

        unimplemented!()
    }

    fn part2(&mut self, (bitmap, input): &Self::Input) -> Result<String> {
        let mut visited = HashSet::new();
        let mut heap = BinaryHeap::new();
        heap.push((u32::MAX, 0, 0, 26, bitmap["AA"], bitmap["AA"], 0u64));
        visited.insert((bitmap["AA"], bitmap["AA"], 0u64, 0));

        while let Some((_est, dist, flow, time_left, from_a, from_b, opened)) = heap.pop() {
            crate::util::progressd(&(_est, dist));
            if _est == dist {
                // We use an optimistic heuristic, so if we are done and this
                // is the top of the heap, this has to be the optimal path.
                return Ok(dist.to_string());
            }

            let estimate = |time_left: u32, opened: u64| -> u32 {
                let mut remainings: Vec<_> = input
                    .iter()
                    .filter_map(|(valve, (flow, _))| {
                        if valve & opened == 0 {
                            Some(*flow)
                        } else {
                            None
                        }
                    })
                    .collect();
                remainings.sort();
                remainings
                    .iter()
                    .rev()
                    .enumerate()
                    // same heuristic but best case it takes 1turn/valve
                    // However, we don't account for distances so we're still overly
                    // optimistic.
                    .map(|(i, f)| f * time_left.saturating_sub(2 * i as u32))
                    .sum::<u32>()
            };

            let (valve_flow_a, connections_a) = &input[&from_a];
            let (valve_flow_b, connections_b) = &input[&from_b];
            let time_left = time_left - 1;
            let dist = dist + flow;
            if time_left == 0 {
                heap.push((dist, dist, flow, time_left, from_a, from_b, opened));
                continue;
            }
            for &next_a in connections_a {
                let est = dist + estimate(time_left, opened) + time_left * flow;
                for &next_b in connections_b {
                    if visited.insert((next_a, next_b, opened, est)) {
                        heap.push((est, dist, flow, time_left, next_a, next_b, opened));
                    }
                }
                if *valve_flow_b > 0 && opened & from_b == 0 {
                    let opened = opened | from_b;
                    let flow = flow + *valve_flow_b;
                    let est = dist + estimate(time_left, opened) + time_left * flow;
                    if visited.insert((next_a, from_b, opened, est)) {
                        heap.push((est, dist, flow, time_left, next_a, from_b, opened));
                    }
                }
            }
            if *valve_flow_a > 0 && opened & from_a == 0 {
                // this one could be opened
                let opened = opened | from_a;
                let flow = flow + *valve_flow_a;
                let est = dist + estimate(time_left, opened) + time_left * flow;
                for &next_b in connections_b {
                    if visited.insert((from_a, next_b, opened, est)) {
                        heap.push((est, dist, flow, time_left, from_a, next_b, opened));
                    }
                }
                if *valve_flow_b > 0 && opened & from_b == 0 {
                    let opened = opened | from_b;
                    let flow = flow + *valve_flow_b;
                    let est = dist + estimate(time_left, opened) + time_left * flow;
                    heap.push((est, dist, flow, time_left, from_a, from_b, opened));
                }
            }
        }

        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "1651";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "1707";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
