use std::sync::RwLock;

use crate::cli::Result;

#[derive(Clone, Debug)]
pub struct Monkey<T> {
    items: Vec<T>,
    op: (Option<u64>, char),
    test: u64,
    t_true: usize,
    t_false: usize,
}

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<Monkey<u64>>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        Ok(data
            .split("\n\n")
            .map(|chunk| {
                let mut lines = chunk.lines();
                let items: Vec<u64> = lines
                    .nth(1)
                    .unwrap()
                    .split(": ")
                    .nth(1)
                    .unwrap()
                    .split(", ")
                    .map(|n| n.parse().unwrap())
                    .collect();
                let mut opline = lines.next().unwrap().split_ascii_whitespace().rev();
                let op = (
                    opline.next().unwrap().parse().ok(),
                    opline.next().unwrap().chars().next().unwrap(),
                );
                let test = lines
                    .next()
                    .unwrap()
                    .rsplit_once(' ')
                    .unwrap()
                    .1
                    .parse()
                    .unwrap();
                let t_true = lines
                    .next()
                    .unwrap()
                    .rsplit_once(' ')
                    .unwrap()
                    .1
                    .parse()
                    .unwrap();
                let t_false = lines
                    .next()
                    .unwrap()
                    .rsplit_once(' ')
                    .unwrap()
                    .1
                    .parse()
                    .unwrap();
                Monkey {
                    items,
                    op,
                    test,
                    t_true,
                    t_false,
                }
            })
            .collect())
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let monkeys: Vec<RwLock<Monkey<u64>>> =
            input.iter().map(|m| RwLock::new(m.clone())).collect();
        let mut inspects = vec![0; input.len()];
        for _round in 0..20 {
            for (m, monkey) in monkeys.iter().enumerate() {
                let mut monkey = monkey.write().unwrap();
                let mut stuff = vec![];
                std::mem::swap(&mut stuff, &mut monkey.items);
                inspects[m] += stuff.len();

                for mut item in stuff.drain(..) {
                    let rhs = monkey.op.0.unwrap_or(item);
                    if monkey.op.1 == '+' {
                        item += rhs;
                    } else {
                        item *= rhs;
                    }
                    item = item.div_euclid(3);
                    let test = item.rem_euclid(monkey.test);
                    monkeys[if test == 0 {
                        monkey.t_true
                    } else {
                        monkey.t_false
                    }]
                    .write()
                    .unwrap()
                    .items
                    .push(item);
                }
            }
        }
        inspects.sort();
        let res: usize = inspects.iter().rev().take(2).copied().product();
        Ok(res.to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        // same as part1, but we use use number theory to to pre-calculate the individual monkey
        // modulus, which we pass as a set, for an item to avoid dealing with the large numbers.
        let mut monkeys: Vec<Monkey<Vec<u64>>> = input
            .iter()
            .map(|m| {
                let smart_items = m
                    .items
                    .iter()
                    .map(|i| input.iter().map(|mm| i.rem_euclid(mm.test)).collect())
                    .collect();
                Monkey {
                    items: smart_items,
                    op: m.op,
                    test: m.test,
                    t_true: m.t_true,
                    t_false: m.t_false,
                }
            })
            .collect();
        let mut inspects = vec![0; input.len()];
        for _round in 0..10000 {
            for m in 0..monkeys.len() {
                let monkey = &mut monkeys[m];
                // the 'ol monkey-borrowcheck switcheroo
                let op = monkey.op;
                let t_true = monkey.t_true;
                let t_false = monkey.t_false;
                let mut stuff = vec![];
                std::mem::swap(&mut stuff, &mut monkey.items);
                inspects[m] += stuff.len();

                for mut item in stuff.drain(..) {
                    for (j, sub_item) in item.iter_mut().enumerate() {
                        let rhs = op.0.unwrap_or(*sub_item);
                        if op.1 == '+' {
                            *sub_item = (*sub_item + rhs).rem_euclid(monkeys[j].test);
                        } else {
                            *sub_item = (*sub_item * rhs).rem_euclid(monkeys[j].test);
                        }
                    }
                    monkeys[if item[m] == 0 { t_true } else { t_false }]
                        .items
                        .push(item);
                }
            }
        }
        inspects.sort();
        let res: usize = inspects.iter().rev().take(2).copied().product();
        Ok(res.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
If true: throw to monkey 2
If false: throw to monkey 3

Monkey 1:
Starting items: 54, 65, 75, 74
Operation: new = old + 6
Test: divisible by 19
If true: throw to monkey 2
If false: throw to monkey 0

Monkey 2:
Starting items: 79, 60, 97
Operation: new = old * old
Test: divisible by 13
If true: throw to monkey 1
If false: throw to monkey 3

Monkey 3:
Starting items: 74
Operation: new = old + 3
Test: divisible by 17
If true: throw to monkey 0
If false: throw to monkey 1
";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "10605";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "2713310158";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
