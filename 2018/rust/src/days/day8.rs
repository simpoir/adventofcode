use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

pub struct Node {
    child: Vec<Node>,
    meta: Vec<u64>,
}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Node;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        let mut iter = data.split_ascii_whitespace().map(|x| Ok(x.parse()?));
        tree_from_data(&mut iter)
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        fn sum_meta(node: &Node) -> u64 {
            node.meta.iter().sum::<u64>() + node.child.iter().map(sum_meta).sum::<u64>()
        }
        Ok(sum_meta(input).to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        fn sum_node(node: &Node) -> u64 {
            if node.child.is_empty() {
                node.meta.iter().sum::<u64>()
            } else {
                node.meta
                    .iter()
                    .filter_map(|i| {
                        i.checked_sub(1)
                            .and_then(|i| node.child.get(i as usize).map(sum_node))
                    })
                    .sum()
            }
        }
        Ok(sum_node(input).to_string())
    }
}

fn tree_from_data(data: &mut dyn DoubleEndedIterator<Item = Result<u64>>) -> Result<Node> {
    let nchild = data.next().unwrap()?;
    let nmeta = data.next().unwrap()?;
    Ok(Node {
        child: (0..nchild)
            .map(|_| tree_from_data(data))
            .collect::<Result<Vec<Node>>>()?,
        meta: data.take(nmeta.try_into()?).flatten().collect(),
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "138";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "66";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
