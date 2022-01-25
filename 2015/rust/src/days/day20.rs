#[derive(Default)]
pub struct Day {}

impl crate::cli::Day for Day {
    type Input = u32;

    fn gen(&self, data: &str) -> Self::Input {
        data.parse().unwrap()
    }

    fn part1(&self, input: &Self::Input) -> String {
        let tgt: u32 = input / 10;
        let mut factors = vec![0];
        factors.reserve(tgt as usize);
        for i in 1u32.. {
            factors.push(1);
            let mut sum: u32 = 0;
            for (i, n) in factors.iter_mut().enumerate().skip(1) {
                *n -= 1;
                if *n == 0 {
                    *n += i as u32;
                    sum += i as u32;
                }
            }
            if sum >= tgt {
                return i.to_string();
            }
            crate::util::progress(&sum);
        }
        unreachable!();
    }

    fn part2(&self, input: &Self::Input) -> String {
        let tgt: u32 = input / 11 + if input % 11 > 0 { 1 } else { 0 };
        let mut factors = std::collections::VecDeque::new();
        for i in 1u32.. {
            factors.push_back((i, 1, 50));
            let mut sum: u32 = 0;
            for (i, n, remain) in factors.iter_mut() {
                *n -= 1;
                if *n == 0 {
                    *remain -= 1;
                    *n += *i;
                    sum += *i;
                }
            }
            if sum >= tgt {
                return i.to_string();
            }
            if factors.front().unwrap().2 == 0 {
                factors.pop_front();
            }
            crate::util::progress(&sum);
        }
        unreachable!();
    }
}
