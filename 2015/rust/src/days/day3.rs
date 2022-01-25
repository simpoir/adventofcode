use std::collections::HashMap;

#[derive(Default)]
pub struct Day {}

impl crate::cli::Day for Day {
    type Input = String;

    fn gen(&self, data: &str) -> Self::Input {
        data.into()
    }

    fn part1(&self, input: &Self::Input) -> String {
        let mut h = HashMap::new();
        let mut pos = (0, 0);
        h.insert(pos, 1);
        for c in input.chars() {
            match c {
                '>' => pos = (pos.0 + 1, pos.1),
                '<' => pos = (pos.0 - 1, pos.1),
                '^' => pos = (pos.0, pos.1 - 1),
                _ => pos = (pos.0, pos.1 + 1),
            }
            *h.entry(pos).or_default() += 1;
        }
        h.len().to_string()
    }

    fn part2(&self, input: &Self::Input) -> String {
        let mut h = HashMap::new();
        let mut posa = (0, 0);
        let mut posb = (0, 0);
        let mut turn = false;
        h.insert(posa, 2);
        for c in input.chars() {
            let pos = if turn { &mut posa } else { &mut posb };
            turn = !turn;
            match c {
                '>' => *pos = (pos.0 + 1, pos.1),
                '<' => *pos = (pos.0 - 1, pos.1),
                '^' => *pos = (pos.0, pos.1 - 1),
                _ => *pos = (pos.0, pos.1 + 1),
            }
            *h.entry(*pos).or_default() += 1;
        }
        h.len().to_string()
    }
}
