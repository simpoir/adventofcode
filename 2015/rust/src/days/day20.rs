use std::cmp::Reverse;

#[derive(Default)]
pub struct Day {}

impl crate::cli::Day for Day {
    type Input = u32;

    fn gen(&self, data: &str) -> Self::Input {
        data.parse().unwrap()
    }

    fn part1(&self, input: &Self::Input) -> String {
        let tgt: u32 = input / 10;
        let mut factors = std::collections::BinaryHeap::new();
        for i in 1u32.. {
            let mut sum: u32 = i;
            while let Some(Reverse((nn, ii))) = factors.pop() {
                if nn != i {
                    factors.push(Reverse((nn, ii)));
                    break;
                }

                sum += ii;
                factors.push(Reverse((nn + ii, ii)));
            }
            if sum >= tgt {
                return i.to_string();
            }
            factors.push(Reverse((i * 2, i)));
            crate::util::progress(&sum);
        }
        unreachable!();
    }

    fn part2(&self, input: &Self::Input) -> String {
        let tgt: u32 = input / 11 + if input % 11 > 0 { 1 } else { 0 };
        let mut factors = std::collections::BinaryHeap::new();
        for i in 1u32.. {
            let mut sum: u32 = i;
            while let Some(Reverse((nn, ii))) = factors.pop() {
                if nn != i {
                    factors.push(Reverse((nn, ii)));
                    break;
                }

                sum += ii;
                factors.push(Reverse((nn + ii, ii)));
            }
            if sum >= tgt {
                return i.to_string();
            }
            factors.push(Reverse((i * 2, i)));
            crate::util::progress(&sum);
        }
        unreachable!();
    }
}
