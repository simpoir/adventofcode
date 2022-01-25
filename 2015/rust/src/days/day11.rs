#[derive(Default)]
pub struct Day {}

impl crate::cli::Day for Day {
    type Input = usize;

    fn gen(&self, data: &str) -> Self::Input {
        data.bytes().fold(0, |r, b| r * 26 + (b - b'a') as usize)
    }

    fn part1(&self, input: &Self::Input) -> String {
        let mut prog = 0;
        for pass in (*input).. {
            let pass = conv(pass);
            prog += 1;
            if prog > 1000000 {
                crate::util::progress(&pass);
                prog = 0;
            }
            if is_valid(&pass) {
                return pass;
            }
        }
        unreachable!()
    }

    fn part2(&self, input: &Self::Input) -> String {
        let mut nth = 0;
        let mut prog = 0;
        for pass in (*input).. {
            let pass = conv(pass);
            prog += 1;
            if prog > 1000000 {
                crate::util::progress(&pass);
                prog = 0;
            }
            if is_valid(&pass) {
                nth += 1;
                if nth == 2 {
                    return pass;
                }
            }
        }
        unreachable!()
    }
}

fn is_valid(pass: &str) -> bool {
    pass.as_bytes()
        .windows(3)
        .any(|a| a[0] == a[1] - 1 && a[0] == a[2] - 2)
        && {
            let mut prev = 0;
            pass.as_bytes()
                .windows(2)
                .enumerate()
                .filter(|(i, a)| {
                    if a[0] == a[1] && prev + 1 != *i {
                        prev = *i;
                        true
                    } else {
                        false
                    }
                })
                .count()
                > 1
        }
        && !pass.chars().any(|c| c == 'i' || c == 'o' || c == 'l')
}

fn conv(pass: usize) -> String {
    let mut pass = pass;
    let mut res = vec![];
    while pass != 0 {
        let digit = (pass % 26) as u8;
        pass /= 26;
        res.push((digit + b'a') as char)
    }
    res.iter().rev().collect()
}
