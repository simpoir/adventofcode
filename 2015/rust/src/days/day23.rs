#[derive(Default)]
pub struct Day {}

#[derive(Debug)]
pub enum Op {
    Hlf(bool),
    Tpl(bool),
    Inc(bool),
    Jmp(isize),
    Jie(bool, isize),
    Jio(bool, isize),
}

impl crate::cli::Day for Day {
    type Input = Vec<Op>;

    fn gen(&self, data: &str) -> Self::Input {
        data.lines()
            .map(|l| {
                let (head, tail) = l.split_once(' ').unwrap();
                match head {
                    "hlf" => Op::Hlf(tail == "b"),
                    "tpl" => Op::Tpl(tail == "b"),
                    "inc" => Op::Inc(tail == "b"),
                    "jmp" => Op::Jmp(tail.parse().unwrap()),
                    "jie" => Op::Jie(
                        tail.get(0..1).unwrap() == "b",
                        tail.get(3..).unwrap().parse().unwrap(),
                    ),
                    "jio" => Op::Jio(
                        tail.get(0..1).unwrap() == "b",
                        tail.get(3..).unwrap().parse().unwrap(),
                    ),
                    _ => unimplemented!(),
                }
            })
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> String {
        run(input, (0, 0)).1.to_string()
    }

    fn part2(&self, input: &Self::Input) -> String {
        run(input, (1, 0)).1.to_string()
    }
}

fn run(code: &[Op], init: (usize, usize)) -> (usize, usize) {
    let mut ip = 0;
    let mut regs = init;
    while let Some(op) = code.get(ip) {
        match op {
            Op::Hlf(b) => *if *b { &mut regs.1 } else { &mut regs.0 } /= 2,
            Op::Tpl(b) => *if *b { &mut regs.1 } else { &mut regs.0 } *= 3,
            Op::Inc(b) => *if *b { &mut regs.1 } else { &mut regs.0 } += 1,
            Op::Jmp(n) => {
                ip = (ip as isize + n) as usize;
                continue;
            }
            Op::Jie(b, n) => {
                if *if *b { &mut regs.1 } else { &mut regs.0 } & 1 == 0 {
                    ip = (ip as isize + n) as usize;
                    continue;
                }
            }
            Op::Jio(b, n) => {
                if *if *b { &mut regs.1 } else { &mut regs.0 } == 1 {
                    ip = (ip as isize + n) as usize;
                    continue;
                }
            }
        }
        ip += 1;
    }
    regs
}
