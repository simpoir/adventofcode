use std::collections::HashMap;

#[derive(Default)]
pub struct Day {}

#[derive(Copy, Clone)]
pub enum Out {
    Out(usize),
    Bot(usize),
}

#[derive(Copy, Clone)]
pub enum Op {
    Set(usize, usize),
    Logic(usize, Out, Out),
}

impl crate::cli::Day for Day {
    type Input = Vec<Op>;

    fn gen(&self, data: &str) -> Self::Input {
        data.lines()
            .map(|l| {
                let mut words = l.split(' ');
                if words.next().unwrap() == "value" {
                    let val = words.next().unwrap().parse().unwrap();
                    let bot = words.rev().next().unwrap().parse().unwrap();
                    Op::Set(val, bot)
                } else {
                    let bot = words.next().unwrap().parse().unwrap();
                    let out_lo = words.nth(3).unwrap() == "output";
                    let lo = words.next().unwrap().parse().unwrap();
                    let mut words = words.rev();
                    let hi = words.next().unwrap().parse().unwrap();
                    let out_hi = words.next().unwrap() == "output";
                    Op::Logic(
                        bot,
                        if out_lo { Out::Out(lo) } else { Out::Bot(lo) },
                        if out_hi { Out::Out(hi) } else { Out::Bot(hi) },
                    )
                }
            })
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> String {
        let mut res = String::new();
        runtil(input, &mut |bots, _| {
            for (k, v) in bots {
                if v.contains(&61) && v.contains(&17) {
                    res = k.to_string();
                    return true;
                }
            }
            false
        });
        res
    }

    fn part2(&self, input: &Self::Input) -> String {
        let mut res = String::new();
        runtil(input, &mut |_, outs| {
            let ress = outs[0] * outs[1] * outs[2];
            if ress > 0 {
                res = ress.to_string();
                true
            } else {
                false
            }
        });
        res
    }
}

type Bots = HashMap<usize, Vec<usize>>;
type State = (Bots, HashMap<usize, (Out, Out)>);

fn runtil<F>(ops: &[Op], f: &mut F)
where
    F: FnMut(&Bots, &[usize]) -> bool,
{
    let mut outs = vec![0; 20];
    let (mut bots, sorts) = setup(ops);
    let mut q = vec![];
    loop {
        if f(&bots, &outs) {
            return;
        }
        for (k, v) in &mut bots {
            if v.len() >= 2 {
                let a = v.pop().unwrap();
                let b = v.pop().unwrap();
                let lo = a.min(b);
                let hi = a.max(b);
                let (lo_to, hi_to) = sorts.get(k).unwrap();
                match hi_to {
                    Out::Out(o) => outs[*o] = hi,
                    Out::Bot(bot) => q.push((bot, hi)),
                }
                match lo_to {
                    Out::Out(o) => outs[*o] = lo,
                    Out::Bot(bot) => q.push((bot, lo)),
                }
            }
        }
        if q.is_empty() {
            break;
        }
        for (to, val) in q.drain(..) {
            bots.entry(*to).or_default().push(val);
        }
    }
}

fn setup(ops: &[Op]) -> State {
    let mut buckets = HashMap::new();
    let mut sort = HashMap::new();
    for op in ops {
        if let Op::Set(val, bot) = op {
            buckets.entry(*bot).or_insert_with(Vec::new).push(*val);
        } else if let Op::Logic(bot, lo, hi) = *op {
            sort.insert(bot, (lo, hi));
        }
    }
    (buckets, sort)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let d: Day = Default::default();
        let input = "value 61 goes to bot 2
bot 2 gives low to bot 1 and high to bot 0
value 17 goes to bot 1
bot 1 gives low to output 1 and high to bot 0
bot 0 gives low to output 2 and high to output 0
value 2 goes to bot 2";
        let expected = "0";
        assert_eq!(expected, d.part1(&d.gen(input)));
    }

    #[test]
    fn test_part2() {
        let d: Day = Default::default();
        let input = "";
        let expected = "";
        assert_eq!(expected, d.part2(&d.gen(input)));
    }
}
