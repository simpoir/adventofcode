#[derive(Default)]
pub struct Day {}

#[derive(Debug)]
pub enum Op {
    Swap(usize, usize),
    Trans(char, char),
    Rotn(bool, usize),
    Rot(char),
    Rev(usize, usize),
    Mov(usize, usize),
}

impl crate::cli::Day for Day {
    type Input = Vec<Op>;

    fn gen(&self, data: &str) -> Self::Input {
        data.lines()
            .map(|l| {
                let mut chunks = l.split(' ');
                match (chunks.next().unwrap(), chunks.next().unwrap()) {
                    ("swap", "position") => Op::Swap(
                        chunks.next().unwrap().parse().unwrap(),
                        chunks.last().unwrap().parse().unwrap(),
                    ),
                    ("swap", "letter") => Op::Trans(
                        chunks.next().unwrap().chars().next().unwrap(),
                        chunks.last().unwrap().chars().next().unwrap(),
                    ),
                    ("rotate", "based") => Op::Rot(chunks.last().unwrap().chars().next().unwrap()),
                    ("rotate", x) => {
                        Op::Rotn(x == "right", chunks.next().unwrap().parse().unwrap())
                    }
                    ("reverse", _) => Op::Rev(
                        chunks.next().unwrap().parse().unwrap(),
                        chunks.last().unwrap().parse().unwrap(),
                    ),
                    ("move", _) => Op::Mov(
                        chunks.next().unwrap().parse().unwrap(),
                        chunks.last().unwrap().parse().unwrap(),
                    ),
                    _ => unimplemented!(),
                }
            })
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> String {
        scramble("abcdefgh", input)
    }

    fn part2(&self, input: &Self::Input) -> String {
        unscramble("fbgdceah", input)
    }
}

fn unscramble(tgt: &str, ops: &[Op]) -> String {
    let mut buf = vec!['.'; tgt.len()];
    // I don't feel like handling the duplicates caused by Op::Rot today so here's brut-force.
    fn walk(tgt: &str, buf: &mut Vec<char>, i: usize, ops: &[Op]) -> bool {
        if i >= buf.len() {
            let s = buf.iter().collect::<String>();
            let matches = scramble(&s, ops) == tgt;
            return matches;
        }
        for c in tgt.chars() {
            buf[i] = c;
            if walk(tgt, buf, i + 1, ops) {
                return true;
            }
        }
        false
    }
    walk(tgt, &mut buf, 0, ops);
    buf.iter().collect()
}

fn scramble(start: &str, ops: &[Op]) -> String {
    let mut res = start.chars().collect::<Vec<_>>();
    let ops: Box<dyn Iterator<Item = _>> = Box::new(ops.iter());
    for op in ops {
        match *op {
            Op::Swap(a, b) => res.swap(a, b),
            Op::Trans(a, b) => res.iter_mut().for_each(|x| match x {
                _ if *x == a => *x = b,
                _ if *x == b => *x = a,
                _ => (),
            }),
            Op::Rotn(right, n) => {
                if right {
                    res.rotate_right(n)
                } else {
                    res.rotate_left(n)
                }
            }
            Op::Rot(x) => {
                if let Some(p) = res.iter().position(|n| *n == x) {
                    if p >= 4 {
                        res.rotate_right(1);
                    }
                    res.rotate_right(p + 1);
                }
            }
            Op::Rev(a, b) => res[a..=b].reverse(),
            Op::Mov(a, b) => {
                let x = res.remove(a);
                res.insert(b, x)
            }
        }
    }
    res.iter().collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    const INPUT: &str = "swap position 4 with position 0
swap letter d with letter b
reverse positions 0 through 4
rotate left 1 step
move position 1 to position 4
move position 3 to position 0
rotate based on position of letter b
rotate based on position of letter d";

    #[test]
    fn test_part1() {
        let d: Day = Default::default();
        let expected = "decab";
        assert_eq!(expected, scramble("abcde", &d.gen(INPUT)));
    }

    #[test]
    fn test_part2() {
        let d: Day = Default::default();
        let ops = &d.gen("reverse positions 0 through 4");
        assert_eq!("abcde", unscramble("edcba", ops));
    }
}
