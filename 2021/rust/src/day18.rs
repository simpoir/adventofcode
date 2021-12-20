#[derive(Default)]
pub struct Day {}

#[derive(Debug, Clone, PartialEq)]
pub struct Elem {
    num: usize,
    lvl: usize,
}

impl crate::Day for Day {
    type Input = Vec<Vec<Elem>>;

    fn gen(&self, data: &str) -> Self::Input {
        data.trim_end()
            .lines()
            .map(|l| {
                let mut out = vec![];
                let mut lvl = 0;
                l.bytes().for_each(|b| match b {
                    b'[' => lvl += 1,
                    b']' => lvl -= 1,
                    b',' => (),
                    b' ' => (),
                    n => out.push(Elem {
                        lvl,
                        num: (n - b'0').into(),
                    }),
                });
                out
            })
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> String {
        let mut data = (*input).clone();
        let res = data
            .drain(..)
            .reduce(|a, b| {
                let mut snail: Vec<Elem> = [a, b]
                    .into_iter()
                    .flatten()
                    .map(|e| Elem {
                        lvl: e.lvl + 1,
                        num: e.num,
                    })
                    .collect();
                reduce(&mut snail);
                snail
            })
            .unwrap();
        format!("{}", magnitude(&res, 0).0)
    }

    fn part2(&self, input: &Self::Input) -> String {
        let mut max = 0;
        for a in input {
            for b in input {
                let mut snail: Vec<Elem> = [a, b]
                    .into_iter()
                    .flatten()
                    .map(|e| Elem {
                        lvl: e.lvl + 1,
                        num: e.num,
                    })
                    .collect();
                reduce(&mut snail);
                max = max.max(magnitude(&snail, 0).0);
            }
        }
        format!("{}", max)
    }
}

fn reduce(snail: &mut Vec<Elem>) {
    // reduce
    loop {
        if explode(snail) {
            continue;
        }
        if split(snail) {
            continue;
        }
        break;
    }
}

fn explode(snail: &mut Vec<Elem>) -> bool {
    for i in 0..snail.len() {
        let Elem { num, lvl } = snail[i];
        if lvl > 4 {
            if i > 0 {
                snail[i - 1].num += num;
            }
            let Elem { num, lvl } = snail[i + 1];
            assert_eq!(5, lvl, "deep explodable should't theoritically nest");
            if i + 2 < snail.len() {
                snail[i + 2].num += num;
            }
            snail.remove(i);
            snail[i] = Elem {
                num: 0,
                lvl: lvl - 1,
            };
            return true;
        }
    }
    false
}

fn split(snail: &mut Vec<Elem>) -> bool {
    for i in 0..snail.len() {
        let Elem { num, lvl } = snail[i];
        if num > 9 {
            let split = (num as f32) / 2.0;
            snail[i].num = split.floor() as usize;
            snail[i].lvl += 1;
            snail.insert(
                i + 1,
                Elem {
                    num: split.ceil() as usize,
                    lvl: lvl + 1,
                },
            );
            return true;
        }
    }
    false
}

/// returns magnitude and number of read elements
fn magnitude(snail: &[Elem], lvl: usize) -> (usize, usize) {
    if lvl == snail[0].lvl {
        return (snail[0].num, 1);
    }
    let (a, l_a) = magnitude(snail, lvl + 1);
    let (b, l_b) = magnitude(&snail[l_a..], lvl + 1);
    (3 * a + 2 * b, l_a + l_b)
}

#[allow(unused)]
fn dbg_snail(snail: &[Elem], lvl: usize) -> usize {
    if lvl == snail[0].lvl {
        print!("{}", snail[0].num);
        return 1;
    }
    print!("[");
    let l_a = dbg_snail(snail, lvl + 1);
    print!(",");
    let l_b = dbg_snail(&snail[l_a..], lvl + 1);
    print!("]");
    if lvl == 0 {
        println!();
    }
    l_a + l_b
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Day as _;

    #[test]
    fn tst_magnitude() {
        let result = Day {}.gen("[9,1]");
        assert_eq!(29, magnitude(&result[0], 0).0);

        let result = Day {}.gen("[[9,1],[1,9]]");
        assert_eq!(129, magnitude(&result[0], 0).0);

        let result = Day {}.gen("[[1,2],[[3,4],5]]");
        assert_eq!(143, magnitude(&result[0], 0).0);
    }

    #[test]
    fn tst_xplode() {
        let mut result = Day {}
            .gen("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")
            .drain(0..1)
            .next()
            .unwrap();
        let expected = Day {}
            .gen("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")
            .drain(0..1)
            .next()
            .unwrap();
        explode(&mut result);
        assert_eq!(expected, result);
    }

    #[test]
    fn test_split() {
        let mut result = vec![Elem { num: 10, lvl: 0 }];
        split(&mut result);
        assert_eq!(
            vec![Elem { num: 5, lvl: 1 }, Elem { num: 5, lvl: 1 },],
            result
        );

        let mut result = vec![Elem { num: 11, lvl: 0 }];
        split(&mut result);
        assert_eq!(
            vec![Elem { num: 5, lvl: 1 }, Elem { num: 6, lvl: 1 },],
            result
        );
    }
}
