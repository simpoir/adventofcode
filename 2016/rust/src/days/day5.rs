use openssl::hash::{hash, MessageDigest};

#[derive(Default)]
pub struct Day {}

impl crate::cli::Day for Day {
    type Input = Vec<u8>;

    fn gen(&self, data: &str) -> Self::Input {
        data.bytes().collect()
    }

    fn part1(&self, input: &Self::Input) -> String {
        let mut res = String::new();
        let mut itr = 0..;
        'dgt: for _ in 0..8 {
            loop {
                let mut msg = input.clone();
                itr.next()
                    .unwrap()
                    .to_string()
                    .bytes()
                    .for_each(|b| msg.push(b));
                let h = hash(MessageDigest::md5(), &msg).unwrap();
                if h.starts_with(&[0, 0]) && (h[2] & 0xF0 == 0) {
                    res.push_str(&format!("{:x}", h[2] & 0xF));
                    crate::util::progress(&res);
                    continue 'dgt;
                }
            }
        }
        res
    }

    fn part2(&self, input: &Self::Input) -> String {
        let mut res = [b'_'; 8];
        let mut itr = 0..;
        'dgt: while res.contains(&b'_') {
            loop {
                let mut msg = input.clone();
                itr.next()
                    .unwrap()
                    .to_string()
                    .bytes()
                    .for_each(|b| msg.push(b));
                let h = hash(MessageDigest::md5(), &msg).unwrap();
                if h.starts_with(&[0, 0]) && (h[2] & 0xF0 == 0) {
                    let val = (h[3] >> 4) as usize;
                    let pos = (h[2] & 0xF) as usize;
                    if pos > 7 || res[pos] != b'_' {
                        continue;
                    }
                    res[pos] = format!("{:x}", val).as_bytes()[0];
                    crate::util::progress(&String::from_utf8_lossy(&res));
                    continue 'dgt;
                }
            }
        }
        String::from_utf8_lossy(&res).to_string()
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
        let expected = "18f47a30";
        assert_eq!(expected, d.part1(&d.gen(input)));
    }

    #[test]
    fn test_part2() {
        let d: Day = Default::default();
        let input = "abc";
        let expected = "05ace8e3";
        assert_eq!(expected, d.part2(&d.gen(input)));
    }
}
