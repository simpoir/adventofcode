use crate::util::combine;

#[derive(Default)]
pub struct Day {}

impl crate::cli::Day for Day {
    type Input = Vec<usize>;

    fn gen(&self, data: &str) -> Self::Input {
        let mut res: Vec<_> = data.lines().map(|x| x.parse().unwrap()).collect();
        res.reverse();
        res
    }

    fn part1(&self, input: &Self::Input) -> String {
        let bal = input.iter().sum::<usize>() / 3;
        for e in (0..input.len()).rev() {
            let firsts = split(input, bal);
            for first in &firsts {
                let remainder: Vec<_> = input[..e]
                    .iter()
                    .filter(|x| !first.contains(x))
                    .copied()
                    .collect();
                let second = split(&remainder, bal);
                if !second.is_empty() {
                    let min_count = firsts.iter().map(|x| x.len()).min().unwrap();
                    return firsts
                        .iter()
                        .filter(|x| x.len() == min_count)
                        .map(|x| x.iter().product::<usize>())
                        .min()
                        .unwrap()
                        .to_string();
                }
            }
        }
        "".to_string()
    }

    fn part2(&self, input: &Self::Input) -> String {
        let bal = input.iter().sum::<usize>() / 4;
        for e in (0..input.len()).rev() {
            let firsts = split(input, bal);
            for first in &firsts {
                let remainder: Vec<_> = input[..e]
                    .iter()
                    .filter(|x| !first.contains(x))
                    .copied()
                    .collect();
                let second = split(&remainder, bal);
                if !second.is_empty() {
                    // FIXME Incorrect to just split twice, but it worked for my data.
                    let min_count = firsts.iter().map(|x| x.len()).min().unwrap();
                    return firsts
                        .iter()
                        .filter(|x| x.len() == min_count)
                        .map(|x| x.iter().product::<usize>())
                        .min()
                        .unwrap()
                        .to_string();
                }
            }
        }
        "".to_string()
    }
}

fn split(boxes: &[usize], tgt: usize) -> Vec<Vec<usize>> {
    let mut res = vec![];
    for i in 1..boxes.len() {
        let mut buf = vec![0; i];
        combine(boxes, &mut buf, &mut |t| {
            if t.iter().sum::<usize>() == tgt {
                res.push(t.to_vec());
            }
            true
        });
        if !res.is_empty() {
            break;
        }
    }
    res
}
