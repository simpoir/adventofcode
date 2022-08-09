use std::collections::BTreeMap;

use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

pub enum Ops {
    Write(bool),
    Left,
    Right,
    Goto(char),
}

pub struct Machine {
    start: char,
    diag: usize,
    states: BTreeMap<char, ([Ops; 3], [Ops; 3])>,
}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Machine;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        let mut lines = data.lines();
        let start = lines.next().unwrap().chars().nth_back(1).unwrap();
        let diag = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .nth(5)
            .unwrap()
            .parse()?;
        let mut states = BTreeMap::new();
        // blank
        while lines.next().is_some() {
            let state = lines.next().unwrap().chars().nth_back(1).unwrap();
            let ops = (
                [
                    Ops::Write(lines.nth(1).unwrap().chars().nth_back(1) == Some('1')),
                    if lines.next().unwrap().ends_with("right.") {
                        Ops::Right
                    } else {
                        Ops::Left
                    },
                    Ops::Goto(lines.next().unwrap().chars().nth_back(1).unwrap()),
                ],
                [
                    Ops::Write(lines.nth(1).unwrap().chars().nth_back(1) == Some('1')),
                    if lines.next().unwrap().ends_with("right.") {
                        Ops::Right
                    } else {
                        Ops::Left
                    },
                    Ops::Goto(lines.next().unwrap().chars().nth_back(1).unwrap()),
                ],
            );
            states.insert(state, ops);
        }
        Ok(Machine {
            start,
            diag,
            states,
        })
    }

    fn part1(&mut self, machine: &Self::Input) -> Result<String> {
        let mut tape = vec![false; 102400];
        let mut pos = 51200;
        let mut state = machine.start;
        for _ in 0..machine.diag {
            let code = machine.states.get(&state).unwrap();
            let branch = if tape[pos] { &code.1 } else { &code.0 };
            for op in branch {
                match op {
                    Ops::Write(x) => tape[pos] = *x,
                    Ops::Left => pos -= 1,
                    Ops::Right => pos += 1,
                    Ops::Goto(s) => state = *s,
                }
            }
        }

        Ok(tape.iter().filter(|b| **b).count().to_string())
    }

    fn part2(&mut self, _input: &Self::Input) -> Result<String> {
        Ok("⭐".to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let input = "\
Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.";
        let expected = "3";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }
}
