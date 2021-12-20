use std::collections::BTreeSet;

#[derive(Default)]
pub struct Day {}

type Pattern = BTreeSet<char>;

impl crate::Day for Day {
    type Input = Vec<(Vec<Pattern>, Vec<Pattern>)>;

    fn gen(&self, data: &str) -> Self::Input {
        data.lines()
            .map(|l| {
                let (input, output) = l.split_once(" | ").unwrap();
                (
                    input.split(' ').map(|s| s.chars().collect()).collect(),
                    output.split(' ').map(|s| s.chars().collect()).collect(),
                )
            })
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> String {
        let res: usize = input
            .iter()
            .map(|l| {
                l.1.iter()
                    .filter(|s| matches!(s.len(), 2 | 4 | 3 | 7))
                    .count()
            })
            .sum();
        format!("{}", res)
    }

    fn part2(&self, input: &Self::Input) -> String {
        let mut res = 0;
        for line in input {
            let known1 = line.0.iter().find(|s| s.len() == 2).unwrap();
            let known7 = line.0.iter().find(|s| s.len() == 3).unwrap();
            let known4 = line.0.iter().find(|s| s.len() == 4).unwrap();
            let known8 = line.0.iter().find(|s| s.len() == 7).unwrap();
            let un069: Vec<_> = line.0.iter().filter(|s| s.len() == 6).collect();
            let known9 = un069.iter().find(|x| x.is_superset(known4)).unwrap();
            let known0 = un069
                .iter()
                .find(|x| x.is_superset(known7) && *x != known9)
                .unwrap();
            let known6 = un069
                .iter()
                .find(|x| (*x != known9) && (*x != known0))
                .unwrap();
            let un235: Vec<_> = line.0.iter().filter(|s| s.len() == 5).collect();
            let known2 = un235
                .iter()
                .find(|x| !known9.is_superset(x) && !known6.is_superset(x))
                .unwrap();
            let known3 = un235
                .iter()
                .find(|x| known9.is_superset(x) && !known6.is_superset(x))
                .unwrap();
            let known5 = un235
                .iter()
                .find(|x| known9.is_superset(x) && known6.is_superset(x))
                .unwrap();
            let known = [
                known0, known1, known2, known3, known4, known5, known6, known7, known8, known9,
            ];
            let out = line.1.iter().fold(0, |res, x| {
                res * 10 + known.iter().position(|n| *n == x).unwrap()
            });
            res += out;
        }
        format!("{}", res)
    }
}
