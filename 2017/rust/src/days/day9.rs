use std::{
    iter::{Enumerate, Peekable},
    str::Bytes,
};

use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

#[derive(Debug)]
pub struct Node(usize, usize, bool, Vec<Node>, usize);

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Node;

    fn gen(&mut self, data: &'i str) -> Result<Self::Input> {
        let mut it = data.bytes().enumerate().peekable();
        fn parse(it: &mut Peekable<Enumerate<Bytes>>) -> Node {
            let mut subnodes = vec![];
            let (start, mut ch) = it.next().unwrap();
            let mut end;
            let garbage;
            let mut gbg = 0;
            match ch {
                b'{' => {
                    garbage = false;
                    if it.peek().unwrap().1 != b'}' {
                        subnodes.push(parse(it));
                    }
                    (end, ch) = it.next().unwrap();
                    loop {
                        if ch == b'}' {
                            break;
                        }
                        subnodes.push(parse(it));
                        (end, ch) = it.next().unwrap();
                    }
                }
                b'<' => {
                    garbage = true;
                    (end, ch) = it.next().unwrap();
                    loop {
                        match ch {
                            b'!' => {
                                it.next();
                            }
                            b'>' => break,
                            _ => gbg += 1,
                        }
                        (end, ch) = it.next().unwrap();
                    }
                }
                _ => unimplemented!(),
            }
            Node(start, end, garbage, subnodes, gbg)
        }
        Ok(parse(&mut it))
    }

    fn part1(&mut self, tree: &Self::Input) -> Result<String> {
        fn score(tree: &Node, pts: usize) -> usize {
            if tree.2 {
                return 0;
            }
            return pts + tree.3.iter().map(|n| score(n, pts + 1)).sum::<usize>();
        }
        Ok(score(tree, 1).to_string())
    }

    fn part2(&mut self, tree: &Self::Input) -> Result<String> {
        fn score(tree: &Node) -> usize {
            if tree.2 {
                tree.4
            } else {
                tree.3.iter().map(score).sum()
            }
        }
        Ok(score(tree).to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let input = "{{<a!>},{<a!>},{<a!>},{<ab>}}";
        let expected = "3";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let input = r#"<{o"i!a,<{i<a>"#;
        let expected = "10";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
