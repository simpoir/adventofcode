use std::{cell::Cell, collections::VecDeque, rc::Rc, sync::RwLock};

use crate::cli::Result;

pub type Reg = Rc<Cell<isize>>;

#[derive(Debug, PartialEq, Eq)]
pub enum Val {
    Ref(Reg),
    Lit(isize),
}

impl Val {
    fn eval(&self) -> isize {
        match self {
            Self::Lit(x) => *x,
            Self::Ref(r) => r.get(),
        }
    }

    fn reg(&self) -> Reg {
        match self {
            Self::Lit(_) => panic!("Val is not Ref"),
            Self::Ref(r) => r.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Ops {
    Snd(Val),
    Set(Reg, Val),
    Add(Reg, Val),
    Mul(Reg, Val),
    Mod(Reg, Val),
    Rcv(Reg),
    Jgz(Val, Val),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Vm {
    rcv_on_null: bool,
    ip: usize,
    code: Vec<Ops>,
    a: Reg,
    b: Reg,
    f: Reg,
    i: Reg,
    p: Reg,
}

impl Clone for Vm {
    fn clone(&self) -> Self {
        // Unfortunately, the Cell alias makes cloning annoying.
        let a: Reg = Default::default();
        let b: Reg = Default::default();
        let f: Reg = Default::default();
        let i: Reg = Default::default();
        let p: Reg = Default::default();
        let dup_reg = |reg: &Reg| {
            if Reg::ptr_eq(reg, &self.a) {
                a.clone()
            } else if Reg::ptr_eq(reg, &self.b) {
                b.clone()
            } else if Reg::ptr_eq(reg, &self.f) {
                f.clone()
            } else if Reg::ptr_eq(reg, &self.i) {
                i.clone()
            } else if Reg::ptr_eq(reg, &self.p) {
                p.clone()
            } else {
                panic!()
            }
        };
        let dup_val = |v: &Val| match v {
            Val::Ref(r) => Val::Ref(dup_reg(r)),
            Val::Lit(l) => Val::Lit(*l),
        };
        let code = self
            .code
            .iter()
            .map(|op| match op {
                Ops::Snd(a) => Ops::Snd(dup_val(a)),
                Ops::Set(a, b) => Ops::Set(dup_reg(a), dup_val(b)),
                Ops::Add(a, b) => Ops::Add(dup_reg(a), dup_val(b)),
                Ops::Mul(a, b) => Ops::Mul(dup_reg(a), dup_val(b)),
                Ops::Mod(a, b) => Ops::Mod(dup_reg(a), dup_val(b)),
                Ops::Rcv(a) => Ops::Rcv(dup_reg(a)),
                Ops::Jgz(a, b) => Ops::Jgz(dup_val(a), dup_val(b)),
            })
            .collect();
        Self {
            ip: self.ip,
            a,
            b,
            f,
            i,
            p,
            code,
            rcv_on_null: self.rcv_on_null,
        }
    }
}

impl Vm {
    /// Steps through the VM. Returns false if would block.
    fn run<R: Fn() -> Option<isize>, S: Fn(isize)>(&mut self, snd: S, rcv: R) -> bool {
        if self.ip >= self.code.len() {
            return false;
        }
        match &self.code[self.ip] {
            Ops::Snd(x) => snd(x.eval()),
            Ops::Set(a, b) => a.set(b.eval()),
            Ops::Add(a, b) => a.set(a.get() + b.eval()),
            Ops::Mul(a, b) => a.set(a.get() * b.eval()),
            Ops::Mod(a, b) => a.set(a.get() % b.eval()),
            Ops::Rcv(x) => {
                if self.rcv_on_null || x.get() != 0 {
                    if let Some(val) = rcv() {
                        x.set(val);
                    } else {
                        return false;
                    }
                }
            }
            Ops::Jgz(a, b) => {
                if a.eval() > 0 {
                    self.ip = (self.ip as isize + b.eval()) as usize;
                    return true;
                }
            }
        };
        self.ip += 1;
        true
    }
}

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vm;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        let a: Reg = Default::default();
        let b: Reg = Default::default();
        let f: Reg = Default::default();
        let i: Reg = Default::default();
        let p: Reg = Default::default();
        let as_val = |x: &str| {
            if let Ok(v) = x.parse() {
                Val::Lit(v)
            } else {
                Val::Ref(match x {
                    "a" => a.clone(),
                    "b" => b.clone(),
                    "f" => f.clone(),
                    "i" => i.clone(),
                    "p" => p.clone(),
                    _ => unimplemented!("reg {x}"),
                })
            }
        };
        let code = data
            .lines()
            .map(|l| {
                let mut chunks = l.split_ascii_whitespace();
                let opcode = chunks.next().unwrap();
                match opcode {
                    "snd" => Ops::Snd(as_val(chunks.next().unwrap())),
                    "rcv" => Ops::Rcv(as_val(chunks.next().unwrap()).reg()),
                    "set" => Ops::Set(
                        as_val(chunks.next().unwrap()).reg(),
                        as_val(chunks.next().unwrap()),
                    ),
                    "add" => Ops::Add(
                        as_val(chunks.next().unwrap()).reg(),
                        as_val(chunks.next().unwrap()),
                    ),
                    "mul" => Ops::Mul(
                        as_val(chunks.next().unwrap()).reg(),
                        as_val(chunks.next().unwrap()),
                    ),
                    "mod" => Ops::Mod(
                        as_val(chunks.next().unwrap()).reg(),
                        as_val(chunks.next().unwrap()),
                    ),
                    "jgz" => Ops::Jgz(
                        as_val(chunks.next().unwrap()),
                        as_val(chunks.next().unwrap()),
                    ),
                    _ => unimplemented!("{}", opcode),
                }
            })
            .collect();
        Ok(Vm {
            rcv_on_null: false,
            ip: 0,
            a,
            b,
            f,
            i,
            p,
            code,
        })
    }

    fn part1(&mut self, vm: &Self::Input) -> Result<String> {
        let mut vm = vm.clone();
        let sent = Cell::new(0);
        let last_rcv = Cell::new(0);

        let rcv = || {
            let v = sent.get();
            last_rcv.set(v);
            Some(v)
        };
        while vm.run(|v| sent.set(v), rcv) {
            if last_rcv.get() != 0 {
                break;
            }
        }
        Ok(last_rcv.get().to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let qa = RwLock::new(VecDeque::<isize>::new());
        let qb = RwLock::new(VecDeque::<isize>::new());

        let mut a = input.clone();
        a.rcv_on_null = true;
        let mut b = input.clone();
        b.rcv_on_null = true;
        b.p.set(1);

        let res = Cell::new(0);
        let snd1 = |v| {
            res.set(res.get() + 1);
            qa.write().unwrap().push_back(v);
        };

        loop {
            if a.run(
                |v| qb.write().unwrap().push_back(v),
                || qa.write().unwrap().pop_front(),
            ) {
                continue;
            }
            if b.run(snd1, || qb.write().unwrap().pop_front()) {
                continue;
            }
            break;
        }
        Ok(res.get().to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_clone() {
        const INPUT: &str = "\
set a 1
add a b
mul a f
mod i 5
snd p
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";
        let mut d: Day = Default::default();
        let a = d.gen(INPUT).unwrap();
        let b = a.clone();
        a.a.set(1);
        a.b.set(2);
        a.f.set(3);
        a.i.set(4);
        a.p.set(5);
        b.a.set(1);
        b.b.set(2);
        b.f.set(3);
        b.i.set(4);
        b.p.set(5);
        assert_eq!(a, b);
    }

    #[test]
    fn test_part1() {
        const INPUT: &str = "\
set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2";
        let mut d: Day = Day {};
        let expected = "4";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        const INPUT: &str = "\
snd 1
snd 2
snd p
rcv a
rcv b
rcv i
rcv f";
        let mut d: Day = Day {};
        let expected = "3";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
