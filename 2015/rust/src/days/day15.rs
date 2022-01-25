#[derive(Default)]
pub struct Day {}

impl crate::cli::Day for Day {
    type Input = Vec<[i64; 5]>;

    fn gen(&self, data: &str) -> Self::Input {
        data.lines()
            .map(|l| {
                let mut res = [0; 5];
                for (i, x) in l.split(", ").enumerate() {
                    res[i] = x.rsplit_once(' ').unwrap().1.parse().unwrap();
                }
                res
            })
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> String {
        let mut best_score = 0;
        combine(100, &mut vec![0; input.len()], 0, &mut |combi| {
            let points = score(combi, input);
            if points > best_score {
                best_score = points;
            }
        });
        best_score.to_string()
    }

    fn part2(&self, input: &Self::Input) -> String {
        let mut best_score = 0;
        combine(100, &mut vec![0; input.len()], 0, &mut |combi| {
            let points = score(combi, input);
            if cals(combi, input) == 500 && points > best_score {
                best_score = points;
            }
        });
        best_score.to_string()
    }
}

fn cals(combi: &[i64], costs: &[[i64; 5]]) -> i64 {
    combi
        .iter()
        .copied()
        .enumerate()
        .map(|(i, n)| n * costs[i][4])
        .sum::<i64>()
}

fn score(combi: &[i64], costs: &[[i64; 5]]) -> i64 {
    (0..4)
        .map(|spec| {
            combi
                .iter()
                .copied()
                .enumerate()
                .map(|(i, n)| n * costs[i][spec])
                .sum::<i64>()
                .max(0)
        })
        .product()
}

fn combine<F: FnMut(&[i64])>(remaining: i64, combi: &mut [i64], idx: usize, func: &mut F) {
    if idx == combi.len() {
        func(combi);
        return;
    }

    let next = idx + 1;
    for n in 0..=remaining {
        let remaining = remaining - n;
        combi[idx] = n;
        combine(remaining, combi, next, func);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_score() {
        assert_eq!(
            62842880,
            score(&[44, 56], &[[-1, -2, 6, 3, 8], [2, 3, -2, -1, 3]])
        );
    }
}
