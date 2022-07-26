use std::collections::VecDeque;

use openssl::hash::{hash, MessageDigest};

#[derive(Default)]
pub struct Day {}

impl crate::cli::Day for Day {
    type Input = String;

    fn gen(&self, data: &str) -> Self::Input {
        data.to_string()
    }

    fn part1(&self, input: &Self::Input) -> String {
        bfs::<false>(input)
    }

    fn part2(&self, input: &Self::Input) -> String {
        bfs::<true>(input).len().to_string()
    }
}

fn bfs<const LONG: bool>(pass: &str) -> String {
    let mut q = VecDeque::new();
    let mut res = String::new();
    q.push_back((0, 0, pass.to_string()));
    while let Some((x, y, path)) = q.pop_front() {
        if x == 3 && y == 3 {
            res = path[pass.len()..].to_string();
            if LONG {
                continue;
            }
            return res;
        }
        let h = hash(MessageDigest::md5(), path.as_bytes()).unwrap();
        if h[0] >> 4 >= 0xb && y > 0 {
            let mut path = path.clone();
            path.push('U');
            q.push_back((x, y - 1, path));
        }
        if h[0] & 0xf >= 0xb && y < 3 {
            let mut path = path.clone();
            path.push('D');
            q.push_back((x, y + 1, path));
        }
        if h[1] >> 4 >= 0xb && x > 0 {
            let mut path = path.clone();
            path.push('L');
            q.push_back((x - 1, y, path));
        }
        if h[1] & 0xf >= 0xb && x < 3 {
            let mut path = path.clone();
            path.push('R');
            q.push_back((x + 1, y, path));
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let d: Day = Default::default();
        let input = "kglvqrro";
        let expected = "DDUDRLRRUDRD";
        assert_eq!(expected, d.part1(&d.gen(input)));
    }

    #[test]
    fn test_part2() {
        let d: Day = Default::default();
        let input = "kglvqrro";
        let expected = "492";
        assert_eq!(expected, d.part2(&d.gen(input)));
    }
}
