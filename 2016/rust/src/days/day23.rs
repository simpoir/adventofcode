use std::{cell::Cell, collections::HashSet, rc::Rc};

#[derive(Default)]
pub struct Day {}

#[derive(Clone, Debug)]
pub enum Operand {
    Lit(isize),
    Ref(Rc<Cell<isize>>),
}

impl Operand {
    fn get(&self) -> isize {
        match &self {
            Self::Lit(i) => *i,
            Self::Ref(r) => r.get(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Op {
    Cpy(Operand, Operand),
    Inc(Operand),
    Dec(Operand),
    Jnz(Operand, Operand),
    Tgl(Operand),
    Out(Operand),
}

#[derive(Clone)]
pub struct Vm {
    ip: usize,
    code: Vec<Op>,
    pub a: Rc<Cell<isize>>,
    b: Rc<Cell<isize>>,
    c: Rc<Cell<isize>>,
    d: Rc<Cell<isize>>,
    visited: HashSet<(isize, isize, isize, isize)>,
    pub out: isize,
}

impl Vm {
    pub fn run(&mut self) {
        while self.step() {}
    }

    fn step(&mut self) -> bool {
        if self.ip >= self.code.len() {
            return false;
        }
        let op = &self.code[self.ip];
        match op {
            Op::Cpy(a, r) => {
                if let Operand::Ref(r) = r {
                    r.set(a.get())
                }
            }
            Op::Inc(r) => {
                if let Operand::Ref(r) = r {
                    r.set(r.get() + 1)
                }
            }
            Op::Dec(r) => {
                if let Operand::Ref(r) = r {
                    r.set(r.get() - 1)
                }
            }
            Op::Jnz(a, j) => {
                if a.get() != 0 {
                    self.ip = (self.ip as isize + j.get()) as usize;
                    return true;
                }
            }
            Op::Tgl(l) => {
                let l = l.get();
                if let Some(tgt) = self.code.get_mut((self.ip as isize + l) as usize) {
                    *tgt = match tgt {
                        Op::Cpy(a, b) => Op::Jnz(a.clone(), b.clone()),
                        Op::Inc(a) => Op::Dec(a.clone()),
                        Op::Dec(a) => Op::Inc(a.clone()),
                        Op::Jnz(a, b) => Op::Cpy(a.clone(), b.clone()),
                        Op::Tgl(a) => Op::Inc(a.clone()),
                        Op::Out(_) => todo!(),
                    };
                }
            }
            Op::Out(o) => {
                if self.out != o.get() {
                    self.a.set(0);
                    return false;
                }
                self.out ^= 1;
                if !self
                    .visited
                    .insert((self.a.get(), self.b.get(), self.c.get(), self.d.get()))
                {
                    // success
                    self.a.set(1);
                    return false;
                }
            }
        }
        self.ip += 1;
        true
    }
}

impl crate::cli::Day for Day {
    type Input = Vm;

    fn gen(&self, data: &str) -> Self::Input {
        let a = Rc::new(Cell::new(0));
        let b = Rc::new(Cell::new(0));
        let c = Rc::new(Cell::new(0));
        let d = Rc::new(Cell::new(0));
        let code = data
            .lines()
            .map(|l| {
                let args: Vec<&str> = l.split(' ').collect();
                let a1: Operand = {
                    if let Ok(lit) = args[1].parse() {
                        Operand::Lit(lit)
                    } else {
                        Operand::Ref(match args[1] {
                            "a" => a.clone(),
                            "b" => b.clone(),
                            "c" => c.clone(),
                            "d" => d.clone(),
                            _ => unimplemented!(),
                        })
                    }
                };
                let a2: Option<Operand> = if args.len() > 2 {
                    Some(if let Ok(lit) = args[2].parse() {
                        Operand::Lit(lit)
                    } else {
                        Operand::Ref(match args[2] {
                            "a" => a.clone(),
                            "b" => b.clone(),
                            "c" => c.clone(),
                            "d" => d.clone(),
                            _ => unimplemented!(),
                        })
                    })
                } else {
                    None
                };
                match args[0] {
                    "cpy" => Op::Cpy(a1, a2.unwrap()),
                    "jnz" => Op::Jnz(a1, a2.unwrap()),
                    "inc" => Op::Inc(a1),
                    "dec" => Op::Dec(a1),
                    "tgl" => Op::Tgl(a1),
                    "out" => Op::Out(a1),
                    _ => unimplemented!(),
                }
            })
            .collect();
        Vm {
            ip: 0,
            code,
            a,
            b,
            c,
            d,
            out: 0,
            visited: HashSet::new(),
        }
    }

    fn part1(&self, input: &Self::Input) -> String {
        let mut vm = (*input).clone();
        vm.a.set(7);
        vm.run();
        vm.a.get().to_string()
    }

    fn part2(&self, input: &Self::Input) -> String {
        let mut vm = (*input).clone();
        vm.a.set(12);
        vm.run();
        vm.a.get().to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let d: Day = Default::default();
        let input = "cpy 2 a
tgl a
tgl a
tgl a
cpy 1 a
dec a
dec a";
        let expected = "3";
        assert_eq!(expected, d.part1(&d.gen(input)));
    }
}
