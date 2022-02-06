#[derive(Default)]
pub struct Day {}

impl crate::cli::Day for Day {
    type Input = Vec<(usize, usize)>;

    fn gen(&self, data: &str) -> Self::Input {
        data.lines()
            .map(|l| {
                let mut words = l.split(' ');
                let count = words.nth(3).unwrap().parse().unwrap();
                let init = words.last().unwrap().trim_end_matches('.').parse().unwrap();
                (count, init)
            })
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> String {
        solve(input).to_string()
    }

    fn part2(&self, input: &Self::Input) -> String {
        let mut new = input.clone();
        new.push((11, 0));
        solve(&new).to_string()
    }
}

fn solve(input: &[(usize, usize)]) -> usize {
    let mut pos = vec![];
    for (adv, (c, init)) in input.iter().enumerate() {
        pos.push((0..*c).cycle().skip(*init + 1 + adv));
    }
    for n in 0usize.. {
        let i = pos[0].next().unwrap();
        let mut done = true;
        for it in &mut pos[1..] {
            let nth = it.next().unwrap();
            done = done && nth == i;
        }
        if done {
            return n;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    const INPUT: &str = "Disc #1 has 5 positions; at time=0, it is at position 4.
Disc #2 has 2 positions; at time=0, it is at position 1.";

    #[test]
    fn test_part1() {
        let d: Day = Default::default();
        let expected = "5";
        assert_eq!(expected, d.part1(&d.gen(INPUT)));
    }
}
