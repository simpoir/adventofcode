use std::collections::BTreeMap;

use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

#[derive(Clone)]
pub struct Vm<'i> {
    regs: BTreeMap<&'i str, isize>,
    code: Vec<(&'i str, bool, isize, &'i str, &'i str, isize)>,
    top: isize,
}

impl<'i> Vm<'i> {
    fn run(&mut self) {
        for (dst, inc, by, cmp, op, cmp_to) in &self.code {
            let cmp = *self.regs.entry(cmp).or_insert(0);
            if match *op {
                "==" => cmp == *cmp_to,
                "!=" => cmp != *cmp_to,
                ">" => cmp > *cmp_to,
                "<" => cmp < *cmp_to,
                ">=" => cmp >= *cmp_to,
                "<=" => cmp <= *cmp_to,
                _ => unimplemented!("{}", op),
            } {
                let by = if *inc { *by } else { -(*by) };
                *self.regs.entry(dst).or_insert(0) += by;
                self.top = self.top.max(*self.regs.get(dst).unwrap());
            }
        }
    }
}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vm<'i>;

    fn gen(&mut self, data: &'i str) -> Result<Self::Input> {
        Ok(Vm {
            regs: BTreeMap::new(),
            top: 0,
            code: data
                .lines()
                .map(|l| {
                    let mut words = l.split_ascii_whitespace();
                    (
                        words.next().unwrap(),
                        words.next() == Some("inc"),
                        words.next().unwrap().parse().unwrap(),
                        words.nth(1).unwrap(),
                        words.next().unwrap(),
                        words.next().unwrap().parse().unwrap(),
                    )
                })
                .collect(),
        })
    }

    fn part1(&mut self, vm: &Self::Input) -> Result<String> {
        let mut vm = vm.clone();
        vm.run();
        Ok(vm.regs.values().max().unwrap().to_string())
    }

    fn part2(&mut self, vm: &Self::Input) -> Result<String> {
        let mut vm = vm.clone();
        vm.run();
        Ok(vm.top.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "\
b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "1";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "10";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
