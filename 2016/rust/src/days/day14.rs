use std::collections::HashSet;
use std::collections::VecDeque;

use openssl::hash::Hasher;
use openssl::hash::MessageDigest;

#[derive(Default)]
pub struct Day {}

type Q = VecDeque<(usize, HashSet<u8>)>;

impl crate::cli::Day for Day {
    type Input = String;

    fn gen(&self, data: &str) -> Self::Input {
        data.to_string()
    }

    fn part1(&self, salt: &Self::Input) -> String {
        gen::<0>(salt).to_string()
    }

    fn part2(&self, salt: &Self::Input) -> String {
        gen::<2016>(salt).to_string()
    }
}

fn gen<const STRETCH: usize>(salt: &str) -> usize {
    let mut threes: Q = Default::default();
    let mut fives: Q = Default::default();
    for n in 0..1000usize {
        scan::<STRETCH>(salt, n, &mut threes, &mut fives);
    }

    let mut top = 1000usize..;
    'key: for key_num in (0..64).rev() {
        crate::util::progress(&key_num);
        loop {
            let top = top.next().unwrap();
            scan::<STRETCH>(salt, top, &mut threes, &mut fives);
            let bottom = top - 1000;
            if let Some(five) = fives.pop_front() {
                if five.0 > bottom {
                    fives.push_front(five);
                }
            }

            if let Some((n, opts)) = threes.pop_front() {
                if n == bottom {
                    if opts
                        .iter()
                        .any(|opt| fives.iter().any(|l| l.1.contains(opt)))
                    {
                        continue 'key;
                    }
                } else {
                    threes.push_front((n, opts));
                }
            }
        }
    }
    top.next().unwrap() - 1001
}

fn scan<const STRETCH: usize>(salt: &str, n: usize, threes: &mut Q, fives: &mut Q) {
    let mut h = Hasher::new(MessageDigest::md5()).unwrap();
    h.update(salt.as_bytes()).unwrap();
    h.update(n.to_string().as_bytes()).unwrap();
    let mut digest = h.finish().unwrap();
    for _ in 0..STRETCH {
        let mut h = Hasher::new(MessageDigest::md5()).unwrap();
        for b in digest.iter() {
            h.update(format!("{b:0>2x}").as_bytes()).unwrap();
        }
        digest = h.finish().unwrap();
    }
    let mut row3 = HashSet::new();
    let mut row5 = HashSet::new();
    let digest = digest
        .iter()
        .flat_map(|b| [*b >> 4, *b & 0xF])
        .collect::<Vec<_>>();
    for w in digest.windows(3) {
        if w[0] == w[1] && w[0] == w[2] {
            row3.insert(w[0]);
            break;
        }
    }
    for w in digest.windows(5) {
        if w[0] == w[1] && w[0] == w[2] && w[0] == w[3] && w[0] == w[4] {
            row5.insert(w[0]);
        }
    }
    if !row3.is_empty() {
        threes.push_back((n, row3));
        if !row5.is_empty() {
            fives.push_back((n, row5));
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let d: Day = Default::default();
        let input = "abc";
        let expected = "22728";
        assert_eq!(expected, d.part1(&d.gen(input)));
    }

    #[test]
    fn test_part2() {
        let d: Day = Default::default();
        let input = "abc";
        let expected = "22551";
        assert_eq!(expected, d.part2(&d.gen(input)));
    }
}
