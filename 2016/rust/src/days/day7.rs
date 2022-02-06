#[derive(Default)]
pub struct Day {}

impl crate::cli::Day for Day {
    type Input = Vec<(Vec<String>, Vec<String>)>;

    fn gen(&self, data: &str) -> Self::Input {
        data.lines()
            .map(|l| {
                let mut a = vec![];
                let mut b = vec![];
                for chunk in l.split(']') {
                    if let Some((head, tail)) = chunk.split_once('[') {
                        a.push(head.to_string());
                        b.push(tail.to_string());
                    } else {
                        a.push(chunk.to_string())
                    }
                }
                (a, b)
            })
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> String {
        input
            .iter()
            .filter(|(a, b)| a.iter().any(abba) && !b.iter().any(abba))
            .count()
            .to_string()
    }

    fn part2(&self, input: &Self::Input) -> String {
        input
            .iter()
            .filter(|(a, b)| {
                a.iter()
                    .map(|x| x.as_bytes().windows(3))
                    .flatten()
                    .filter_map(aba)
                    .any(|bab| b.iter().any(|x| x.contains(&bab)))
            })
            .count()
            .to_string()
    }
}

#[allow(clippy::ptr_arg)]
fn abba(input: &String) -> bool {
    input
        .as_bytes()
        .windows(4)
        .any(|x| x[0] == x[3] && x[1] == x[2] && x[0] != x[1])
}

#[allow(clippy::ptr_arg)]
fn aba(x: &[u8]) -> Option<String> {
    if x[0] == x[2] && x[0] != x[1] {
        Some(String::from_utf8(vec![x[1], x[0], x[1]]).unwrap())
    } else {
        None
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let d: Day = Default::default();
        let input = "abba[mnop]qrst
abcd[bddb]xyyx
aaaa[qwer]tyui
ioxxoj[asdfgh]zxcvbn";
        let expected = "2";
        assert_eq!(expected, d.part1(&d.gen(input)));
    }

    #[test]
    fn test_part2() {
        let d: Day = Default::default();
        let input = "aba[bab]xyz
xyx[xyx]xyx
aaa[kek]eke
zazbz[bzb]cdb";
        let expected = "3";
        assert_eq!(expected, d.part2(&d.gen(input)));
    }
}
