use std::collections::HashSet;
use std::io::Write;
use std::{cell::Cell, hash::Hash};

#[derive(Default)]
pub struct Day {}

#[derive(Debug, Clone)]
pub enum Operand {
    W,
    X,
    Y,
    Z,
    Num(isize),
}

#[derive(Debug, Clone)]
pub enum Op {
    Inp(Operand),
    Add(Operand, Operand),
    Mul(Operand, Operand),
    Div(Operand, Operand),
    Mod(Operand, Operand),
    Eql(Operand, Operand),
}

#[derive(Debug, Clone)]
pub struct VmState {
    w: Cell<isize>,
    x: Cell<isize>,
    y: Cell<isize>,
    z: Cell<isize>,
    ip: usize,
}

#[derive(Debug, Clone)]
pub struct Vm {
    code: Vec<Op>,
    state: VmState,
}

impl crate::Day for Day {
    type Input = Vm;

    fn gen(&self, data: &str) -> Self::Input {
        fn parse_operand(stuff: &str) -> Operand {
            match stuff {
                "w" => Operand::W,
                "x" => Operand::X,
                "y" => Operand::Y,
                "z" => Operand::Z,
                n => Operand::Num(n.parse().unwrap()),
            }
        }

        let code = data
            .trim_end()
            .lines()
            .map(|l| {
                let mut l = l.split(' ');
                match l.next().unwrap() {
                    "inp" => Op::Inp(parse_operand(l.next().unwrap())),
                    "add" => Op::Add(
                        parse_operand(l.next().unwrap()),
                        parse_operand(l.next().unwrap()),
                    ),
                    "mul" => Op::Mul(
                        parse_operand(l.next().unwrap()),
                        parse_operand(l.next().unwrap()),
                    ),
                    "div" => Op::Div(
                        parse_operand(l.next().unwrap()),
                        parse_operand(l.next().unwrap()),
                    ),
                    "mod" => Op::Mod(
                        parse_operand(l.next().unwrap()),
                        parse_operand(l.next().unwrap()),
                    ),
                    "eql" => Op::Eql(
                        parse_operand(l.next().unwrap()),
                        parse_operand(l.next().unwrap()),
                    ),
                    _ => unimplemented!(),
                }
            })
            .collect();

        Vm {
            code,
            state: VmState::new(),
        }
    }

    fn part1(&self, input: &Self::Input) -> String {
        let mut res = [0; 14];
        run(
            &mut input.clone(),
            &mut res,
            0,
            &mut HashSet::new(),
            &(1..10).rev(),
        )
        .unwrap()
    }

    fn part2(&self, input: &Self::Input) -> String {
        let mut res = [0; 14];
        run(
            &mut input.clone(),
            &mut res,
            0,
            &mut HashSet::new(),
            &(1..10),
        )
        .unwrap()
    }
}

#[allow(clippy::mutable_key_type)]
fn run<R>(
    vm: &mut Vm,
    inputs: &mut [isize; 14],
    idx: usize,
    cache: &mut HashSet<VmState>,
    range: &R,
) -> Option<String>
where
    R: IntoIterator<Item = isize> + Clone,
{
    if cache.contains(&vm.state) {
        return None;
    } else if idx == 4 {
        // slow loop feedback
        print!(
            "{}Scan {}{}{}",
            ansi_escapes::CursorTo::AbsoluteX(0),
            inputs[0],
            inputs[1],
            inputs[2]
        );
        std::io::stdout().flush().unwrap();
    }

    let initial = vm.state.clone();
    for i in range.clone() {
        inputs[idx] = i;
        if let E::Eop(res) = vm.step(i) {
            if res {
                return Some(inputs.iter().map(|i| format!("{}", i)).collect());
            }
        } else {
            let res = run(vm, inputs, idx + 1, cache, range);
            if res.is_some() {
                return res;
            }
        }
        vm.state = initial.clone();
    }
    cache.insert(initial);
    None
}

impl VmState {
    fn new() -> VmState {
        VmState {
            w: 0.into(),
            x: 0.into(),
            y: 0.into(),
            z: 0.into(),
            ip: 0,
        }
    }
}

#[derive(Debug, PartialEq)]
enum E {
    Eop(bool),
    Cont,
}

impl Vm {
    fn step(&mut self, input: isize) -> E {
        loop {
            self.sub_step(input);
            match self.code.get(self.state.ip) {
                None => return E::Eop(self.state.z.get() == 0),
                Some(Op::Inp(_)) => return E::Cont,
                _ => (),
            }
        }
    }

    fn sub_step(&mut self, input: isize) {
        if let Some(i) = self.code.get(self.state.ip) {
            match i {
                Op::Inp(a) => self.get_mut(a).set(input),
                Op::Add(a, b) => self.get_mut(a).set(self.get_reg(a) + self.get_reg(b)),
                Op::Mul(a, b) => self.get_mut(a).set(self.get_reg(a) * self.get_reg(b)),
                Op::Div(a, b) => {
                    if self.get_reg(b) == 0 {
                        // nan
                        return;
                    }
                    self.get_mut(a).set(self.get_reg(a) / self.get_reg(b))
                }
                Op::Mod(a, b) => {
                    if self.get_reg(b) == 0 {
                        // nan
                        return;
                    }
                    self.get_mut(a).set(self.get_reg(a) % self.get_reg(b))
                }
                Op::Eql(a, b) => self
                    .get_mut(a)
                    .set((self.get_reg(a) == self.get_reg(b)).into()),
            }
            self.state.ip += 1;
        };
    }

    fn get_mut(&self, o: &Operand) -> &Cell<isize> {
        match o {
            Operand::W => &self.state.w,
            Operand::X => &self.state.x,
            Operand::Y => &self.state.y,
            Operand::Z => &self.state.z,
            Operand::Num(_) => unreachable!(),
        }
    }

    fn get_reg(&self, o: &Operand) -> isize {
        match o {
            Operand::W => self.state.w.get(),
            Operand::X => self.state.x.get(),
            Operand::Y => self.state.y.get(),
            Operand::Z => self.state.z.get(),
            Operand::Num(n) => *n,
        }
    }
}

impl Hash for VmState {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_isize(self.x.get());
        state.write_isize(self.y.get());
        state.write_isize(self.z.get());
        state.write_usize(self.ip);
    }
}

impl PartialEq for VmState {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && self.ip == other.ip
    }
}

impl Eq for VmState {}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Day as _;

    #[test]
    fn test_bits() {
        let data = "inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2";
        let alu = Day {}.gen(data);
        let mut alu0 = alu.clone();
        alu0.step(0);
        assert_eq!(E::Eop(true), alu0.step(0));
        assert_eq!(0, alu0.state.w.get());
        assert_eq!(0, alu0.state.x.get());
        assert_eq!(0, alu0.state.y.get());
        assert_eq!(0, alu0.state.z.get());

        let mut alu0 = alu.clone();
        assert_eq!(E::Eop(false), alu0.step(1));
        assert_eq!(0, alu0.state.w.get());
        assert_eq!(0, alu0.state.x.get());
        assert_eq!(0, alu0.state.y.get());
        assert_eq!(1, alu0.state.z.get());

        let mut alu0 = alu.clone();
        assert_eq!(E::Eop(true), alu0.step(10));
        assert_eq!(1, alu0.state.w.get());
        assert_eq!(0, alu0.state.x.get());
        assert_eq!(1, alu0.state.y.get());
        assert_eq!(0, alu0.state.z.get());

        let mut alu0 = alu;
        assert_eq!(E::Eop(false), alu0.step(15));
        assert_eq!(1, alu0.state.w.get());
        assert_eq!(1, alu0.state.x.get());
        assert_eq!(1, alu0.state.y.get());
        assert_eq!(1, alu0.state.z.get());
    }
}
