use super::day23::Vm;

#[derive(Default)]
pub struct Day {}

impl crate::cli::Day for Day {
    type Input = Vm;

    fn gen(&self, data: &str) -> Self::Input {
        super::day23::Day {}.gen(data)
    }

    fn part1(&self, vm: &Self::Input) -> String {
        for a in 0.. {
            let mut vm = vm.clone();
            vm.a.set(a);
            vm.run();
            if vm.a.get() == 1 {
                return a.to_string();
            }
        }
        unimplemented!();
    }

    fn part2(&self, _input: &Self::Input) -> String {
        "⭐".to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let d: Day = Default::default();
        let input = "";
        let expected = "";
        assert_eq!(expected, d.part1(&d.gen(input)));
    }
}
