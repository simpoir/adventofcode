use openssl::hash::{hash, MessageDigest};

#[derive(Default)]
pub struct Day {}

impl crate::cli::Day for Day {
    type Input = String;

    fn gen(&self, data: &str) -> Self::Input {
        data.into()
    }

    fn part1(&self, input: &Self::Input) -> String {
        let mut feedback = 0;
        for i in 1.. {
            feedback += 1;
            if feedback > 100000 {
                crate::util::progress(&i.to_string());
                feedback = 0;
            }
            let data = format!("{}{}", input, i);
            let res = hash(MessageDigest::md5(), data.as_bytes()).unwrap();
            if res.starts_with(&[0; 2]) && res[2] < 16 {
                return i.to_string();
            }
        }
        unimplemented!();
    }

    fn part2(&self, input: &Self::Input) -> String {
        let mut feedback = 0;
        for i in 1.. {
            feedback += 1;
            if feedback > 100000 {
                crate::util::progress(&i.to_string());
                feedback = 0;
            }
            let data = format!("{}{}", input, i);
            let res = hash(MessageDigest::md5(), data.as_bytes()).unwrap();
            if res.starts_with(&[0; 3]) {
                return i.to_string();
            }
        }
        unimplemented!();
    }
}
