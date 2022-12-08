use crate::cli::Result;

#[derive(Debug)]
pub enum Node {
    Dir(Vec<Node>),
    File(usize),
}

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Node;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        let mut tree = Node::Dir(vec![]);
        let mut stack = vec![];
        for l in data.lines() {
            if l.starts_with('$') {
                if l == "$ cd /" || l == "$ ls" {
                    continue;
                } else if l == "$ cd .." {
                    let mut other = stack.pop().unwrap();
                    std::mem::swap(&mut tree, &mut other);
                    if let Node::Dir(dir) = &mut tree {
                        dir.push(other);
                    }
                } else {
                    // cd
                    let mut new_tree = Node::Dir(vec![]);
                    std::mem::swap(&mut tree, &mut new_tree);
                    stack.push(new_tree);
                }
            } else if l.starts_with("dir") {
                // noop
            } else {
                let (space, _name) = l.split_once(' ').unwrap();
                if let Node::Dir(dir) = &mut tree {
                    dir.push(Node::File(space.parse()?));
                }
            }
        }
        while let Some(mut other) = stack.pop() {
            std::mem::swap(&mut tree, &mut other);
            if let Node::Dir(dir) = &mut tree {
                dir.push(other);
            }
        }
        Ok(tree)
    }

    fn part1(&mut self, tree: &Self::Input) -> Result<String> {
        let mut total = 0;
        walk(tree, &mut |t| {
            if t <= 100000 {
                total += t
            }
        });
        Ok(total.to_string())
    }

    fn part2(&mut self, tree: &Self::Input) -> Result<String> {
        let mut sizes = vec![];
        let tot = walk(tree, &mut |t| {
            sizes.push(t);
        });
        let missing = 30000000 - (70000000 - tot);
        Ok(sizes
            .iter()
            .filter(|s| **s >= missing)
            .min()
            .unwrap()
            .to_string())
    }
}

fn walk<T>(tree: &Node, cb: &mut T) -> usize
where
    T: FnMut(usize),
{
    match tree {
        Node::Dir(dir) => {
            let subtot = dir.iter().map(|sub| walk(sub, cb)).sum();
            cb(subtot);
            subtot
        }
        Node::File(siz) => *siz,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "95437";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "24933642";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
