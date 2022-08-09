use std::{cell::Cell, rc::Rc};

use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

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
    Set(Reg, Val),
    Add(Reg, Val),
    Sub(Reg, Val),
    Mul(Reg, Val),
    Jnz(Val, Val),
}

#[derive(Debug, PartialEq, Eq)]
pub struct Vm {
    rcv_on_null: bool,
    ip: usize,
    code: Vec<Ops>,
    a: Reg,
    b: Reg,
    c: Reg,
    d: Reg,
    e: Reg,
    f: Reg,
    g: Reg,
    h: Reg,
    i: Reg,
    p: Reg,
}

impl Clone for Vm {
    fn clone(&self) -> Self {
        // Unfortunately, the Cell alias makes cloning annoying.
        let a: Reg = Default::default();
        let b: Reg = Default::default();
        let c: Reg = Default::default();
        let d: Reg = Default::default();
        let e: Reg = Default::default();
        let f: Reg = Default::default();
        let g: Reg = Default::default();
        let h: Reg = Default::default();
        let i: Reg = Default::default();
        let p: Reg = Default::default();
        let dup_reg = |reg: &Reg| {
            if Reg::ptr_eq(reg, &self.a) {
                a.clone()
            } else if Reg::ptr_eq(reg, &self.b) {
                b.clone()
            } else if Reg::ptr_eq(reg, &self.c) {
                c.clone()
            } else if Reg::ptr_eq(reg, &self.d) {
                d.clone()
            } else if Reg::ptr_eq(reg, &self.e) {
                e.clone()
            } else if Reg::ptr_eq(reg, &self.f) {
                f.clone()
            } else if Reg::ptr_eq(reg, &self.g) {
                g.clone()
            } else if Reg::ptr_eq(reg, &self.h) {
                h.clone()
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
                Ops::Set(a, b) => Ops::Set(dup_reg(a), dup_val(b)),
                Ops::Add(a, b) => Ops::Add(dup_reg(a), dup_val(b)),
                Ops::Sub(a, b) => Ops::Sub(dup_reg(a), dup_val(b)),
                Ops::Mul(a, b) => Ops::Mul(dup_reg(a), dup_val(b)),
                Ops::Jnz(a, b) => Ops::Jnz(dup_val(a), dup_val(b)),
            })
            .collect();
        Self {
            ip: self.ip,
            a,
            b,
            c,
            d,
            e,
            f,
            g,
            h,
            i,
            p,
            code,
            rcv_on_null: self.rcv_on_null,
        }
    }
}

impl Vm {
    /// Steps through the VM. Returns false if would block.
    fn run(&mut self, mul: &mut usize) -> bool {
        if self.ip >= self.code.len() {
            return false;
        }
        match &self.code[self.ip] {
            Ops::Set(a, b) => a.set(b.eval()),
            Ops::Add(a, b) => a.set(a.get() + b.eval()),
            Ops::Sub(a, b) => a.set(a.get() - b.eval()),
            Ops::Mul(a, b) => {
                *mul += 1;
                a.set(a.get() * b.eval())
            }
            Ops::Jnz(a, b) => {
                if a.eval() != 0 {
                    self.ip = (self.ip as isize + b.eval()) as usize;
                    return true;
                }
            }
        };
        self.ip += 1;
        true
    }
}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vm;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        let a: Reg = Default::default();
        let b: Reg = Default::default();
        let c: Reg = Default::default();
        let d: Reg = Default::default();
        let e: Reg = Default::default();
        let f: Reg = Default::default();
        let g: Reg = Default::default();
        let h: Reg = Default::default();
        let i: Reg = Default::default();
        let p: Reg = Default::default();
        let as_val = |x: &str| {
            if let Ok(v) = x.parse() {
                Val::Lit(v)
            } else {
                Val::Ref(match x {
                    "a" => a.clone(),
                    "b" => b.clone(),
                    "c" => c.clone(),
                    "d" => d.clone(),
                    "e" => e.clone(),
                    "f" => f.clone(),
                    "g" => g.clone(),
                    "h" => h.clone(),
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
                    "set" => Ops::Set(
                        as_val(chunks.next().unwrap()).reg(),
                        as_val(chunks.next().unwrap()),
                    ),
                    "add" => Ops::Add(
                        as_val(chunks.next().unwrap()).reg(),
                        as_val(chunks.next().unwrap()),
                    ),
                    "sub" => Ops::Sub(
                        as_val(chunks.next().unwrap()).reg(),
                        as_val(chunks.next().unwrap()),
                    ),
                    "mul" => Ops::Mul(
                        as_val(chunks.next().unwrap()).reg(),
                        as_val(chunks.next().unwrap()),
                    ),
                    "jnz" => Ops::Jnz(
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
            c,
            d,
            e,
            f,
            g,
            h,
            i,
            p,
            code,
        })
    }

    fn part1(&mut self, vm: &Self::Input) -> Result<String> {
        let mut vm = vm.clone();
        let mut mul = 0;
        while vm.run(&mut mul) {}
        Ok(mul.to_string())
    }

    fn part2(&mut self, vm: &Self::Input) -> Result<String> {
        let _code = "\
jnz 1 5           //
mul b 100         // b = 107900
sub b -100000     // 
set c b           // c = 124900
sub c -17000      //
set f 1           //   loop { f = false
set d 2           //
set e 2           //          for d in 2..b { 
set g d           //               for e in 2..b {
mul g e           //
sub g b           //
jnz g 2           //                    if d * e == b {
set f 0           //                     f = true; break; d * e are divisors
sub e -1          //                    }
jnz g -8          //               }
sub d -1          //
set g d           //
sub g b           //
jnz g -13         //          } while d != b
jnz f 2           //          if f { // so h is not prime
sub h -1          //            h += 1
set g b           //          }
sub g c           //
jnz g 2           //          if b == c {
jnz 1 3           //            exit
sub b -17         //          } b += 17
jnz 1 -23"; // }

        // So, basically a count of non-prime numbers between [b, c] (inclusively)
        let mut vm = vm.clone();
        vm.a.set(1);
        // init puzzle numbers
        for _ in 0..8 {
            vm.run(&mut 0);
        }

        fn not_prime(n: &usize) -> bool {
            // numbers are small, erathosthenes rules
            for i in 2..=(f64::sqrt(*n as f64) as usize) {
                if n % i == 0 {
                    return true;
                }
            }
            false
        }
        let b = vm.b.get();
        let c = vm.c.get();
        Ok((b as usize..=c as usize)
            .step_by(17)
            .filter(not_prime)
            .count()
            .to_string())
    }
}
