#[derive(Default)]
pub struct Day {}

impl crate::Day for Day {
    type Input = Vec<Vec<bool>>;

    fn gen(&self, data: &str) -> Self::Input {
        data.lines()
            .map(|l| l.chars().map(|c| c == '1').collect())
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> String {
        let cut = input.len() / 2;
        let res = self
            .bitsum(input)
            .iter()
            .map(|x| x > &cut)
            .fold((0, 0), |(g, e), b| {
                if b {
                    (g * 2 + 1, e * 2)
                } else {
                    (g * 2, e * 2 + 1)
                }
            });
        format!("{}", res.0 * res.1)
    }

    fn part2(&self, input: &Self::Input) -> String {
        let width = input[0].len();
        let mut o2: Vec<&Vec<bool>> = input.iter().collect();
        for i in 0..width {
            let count = o2.iter().filter(|n| n[i]).count();
            if count * 2 >= o2.len() {
                o2 = o2.iter().filter(|n| n[i]).copied().collect();
            } else {
                o2 = o2.iter().filter(|n| !n[i]).copied().collect();
            }
        }
        let o2 = o2[0].iter().fold(0, |a, b| a * 2 + if *b { 1 } else { 0 });
        let mut co2: Vec<&Vec<bool>> = input.iter().collect();
        for i in 0..width {
            let count = co2.iter().filter(|n| n[i]).count();
            if count * 2 >= co2.len() {
                co2 = co2.iter().filter(|n| !n[i]).copied().collect();
            } else {
                co2 = co2.iter().filter(|n| n[i]).copied().collect();
            }
            if co2.len() == 1 {
                break;
            }
        }
        let co2 = co2[0].iter().fold(0, |a, b| a * 2 + if *b { 1 } else { 0 });
        format!("{}", o2 * co2)
    }
}

impl Day {
    fn bitsum(&self, input: &[Vec<bool>]) -> Vec<usize> {
        let width = input[0].len();
        input.iter().fold(vec![0; width], |t, x| {
            x.iter()
                .zip(t)
                .map(|(a, b)| if *a { b + 1 } else { b })
                .collect()
        })
    }
}
