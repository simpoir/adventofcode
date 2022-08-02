use crate::cli::Result;
use std::collections::BTreeMap;

#[derive(Default)]
pub struct Day {
    root: Option<usize>,
}

type Node<'i> = (u32, &'i str, Vec<usize>);

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<Node<'i>>;

    fn need_part1() -> bool {
        true
    }

    fn gen(&mut self, data: &'i str) -> Result<Self::Input> {
        let mut seen = BTreeMap::new();
        let mut count = 0;
        let mut tree = vec![(0, "", vec![]); data.lines().count()];
        data.lines().for_each(|l| {
            let mut words = l.split_ascii_whitespace();
            let name = words.next().unwrap();
            let entry = *seen.entry(name).or_insert_with(|| {
                count += 1;
                count - 1
            });
            let weight: u32 = words
                .next()
                .unwrap()
                .trim_matches(&['(', ')'][..])
                .parse()
                .unwrap();
            tree[entry] = (
                weight,
                name,
                words
                    .skip(1)
                    .map(|child| {
                        let entry = seen.entry(child.trim_end_matches(',')).or_insert_with(|| {
                            count += 1;
                            count - 1
                        });
                        *entry
                    })
                    .collect(),
            );
        });
        Ok(tree)
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let res = (0..input.len())
            .find(|i| {
                input
                    .iter()
                    .map(|j| &j.2)
                    .flatten()
                    .find(|k| *k == i)
                    .is_none()
            })
            .expect("one root");
        self.root = Some(res);
        Ok(input[res].1.to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let rootidx = self.root.expect("part1 ran");
        let root = &input[rootidx];
        let mut totals = vec![0; input.len()];

        fn totalize(input: &[Node<'_>], rootidx: usize, totals: &mut [u32]) -> u32 {
            input[rootidx].0
                + input[rootidx]
                    .2
                    .iter()
                    .map(|i| {
                        totals[*i] = totalize(input, *i, totals);
                        totals[*i]
                    })
                    .sum::<u32>()
        }
        totals[rootidx] = totalize(input, rootidx, &mut totals);

        fn aggr(node: &Node<'_>, totals: &[u32]) -> BTreeMap<u32, u32> {
            let mut aggr = BTreeMap::new();
            node.2
                .iter()
                .for_each(|i| *aggr.entry(totals[*i]).or_insert(0) += 1);
            aggr
        }

        let subweights_aggr = aggr(&root, &totals);
        // only works because root contains a triplet.
        let good_subweight = subweights_aggr.iter().max_by_key(|(_, v)| *v).unwrap().0;
        let bad_subnode = root
            .2
            .iter()
            .find(|i| totals[**i] != *good_subweight)
            .unwrap();

        fn balance(input: &[Node<'_>], rootidx: usize, expected: u32, totals: &[u32]) -> u32 {
            let node = &input[rootidx];
            if node.2.is_empty() {
                expected
            } else {
                let sub_aggr = aggr(&node, totals);
                if sub_aggr.len() == 1 {
                    expected - node.2.len() as u32 * totals[node.2[0]]
                } else {
                    let sub_expected = (expected - node.0) / node.2.len() as u32;
                    let badidx = node.2.iter().find(|i| totals[**i] != sub_expected).unwrap();
                    balance(input, *badidx, sub_expected, totals)
                }
            }
        }

        Ok(balance(input, *bad_subnode, *good_subweight, &totals).to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    const INPUT: &str = "\
pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "tknk";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "60";
        let data = d.gen(INPUT).unwrap();
        d.part1(&data).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
