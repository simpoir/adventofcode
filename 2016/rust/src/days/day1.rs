use std::collections::HashSet;

#[derive(Default)]
pub struct Day {}

impl crate::cli::Day for Day {
    type Input = Vec<(bool, u8)>;

    fn gen(&self, data: &str) -> Self::Input {
        data.split(", ")
            .map(|b| (b.starts_with('L'), b.get(1..).unwrap().parse().unwrap()))
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> String {
        let mut pos = (0i32, 0i32);
        let mut ori = (0, 1);
        for &(dir, steps) in input {
            ori = if dir {
                (-ori.1, ori.0)
            } else {
                (ori.1, -ori.0)
            };
            pos.0 += ori.0 * steps as i32;
            pos.1 += ori.1 * steps as i32;
        }

        (pos.0 + pos.1).to_string()
    }

    fn part2(&self, input: &Self::Input) -> String {
        let mut visited = HashSet::new();
        let mut pos = (0i32, 0i32);
        let mut ori = (0, 1);
        for &(dir, steps) in input {
            ori = if dir {
                (-ori.1, ori.0)
            } else {
                (ori.1, -ori.0)
            };
            for _ in 0..steps {
                pos.0 += ori.0;
                pos.1 += ori.1;
                if !visited.insert(pos) {
                    return (pos.0 + pos.1).to_string();
                }
            }
        }

        unimplemented!();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let d: Day = Default::default();
        assert_eq!("12", d.part1(&d.gen("R5, L5, R5, R3")));
    }

    #[test]
    fn test_part2() {
        let d: Day = Default::default();
        assert_eq!("4", d.part2(&d.gen("R8, R4, R4, R8")));
    }
}
