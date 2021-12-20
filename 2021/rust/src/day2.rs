#[derive(Default)]
pub struct Day {}

pub enum Cmd {
    Forward(isize),
    Up(isize),
    Down(isize),
}

impl crate::Day for Day {
    type Input = Vec<Cmd>;

    fn gen(&self, data: &str) -> Self::Input {
        data.trim()
            .split('\n')
            .map(|l| match l.split_once(" ") {
                Some(("up", v)) => Cmd::Up(v.parse().unwrap()),
                Some(("down", v)) => Cmd::Down(v.parse().unwrap()),
                Some(("forward", v)) => Cmd::Forward(v.parse().unwrap()),
                _ => unimplemented!(),
            })
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> String {
        let res = input.iter().fold((0, 0), |(h, v), c| match c {
            Cmd::Forward(x) => (h + x, v),
            Cmd::Up(x) => (h, v - x),
            Cmd::Down(x) => (h, v + x),
        });
        format!("{}", res.0 * res.1)
    }

    fn part2(&self, input: &Self::Input) -> String {
        let res = input.iter().fold((0, 0, 0), |(aim, h, v), c| match c {
            Cmd::Forward(x) => (aim, h + x, v + x * aim),
            Cmd::Up(x) => (aim - x, h, v),
            Cmd::Down(x) => (aim + x, h, v),
        });
        format!("{}", res.1 * res.2)
    }
}
