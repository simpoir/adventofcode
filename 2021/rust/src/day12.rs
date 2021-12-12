pub struct Day {}

pub struct Node {
    is_small: bool,
    name: String,
    paths: Vec<usize>,
}

impl Node {
    fn new(name: &str) -> Self {
        Node {
            is_small: name.chars().next().unwrap().is_ascii_lowercase(),
            name: name.to_string(),
            paths: vec![],
        }
    }
}

impl crate::Day for Day {
    type Input = Vec<Node>;

    fn gen(&self, data: &str) -> Self::Input {
        let mut res: Self::Input = vec![Node::new("start")];
        data.lines().for_each(|l| {
            let (a, b) = l.split_once('-').unwrap();
            let i_a = res.iter().position(|x| x.name == a).unwrap_or_else(|| {
                res.push(Node::new(a));
                res.len() - 1
            });
            let i_b = res.iter().position(|x| x.name == b).unwrap_or_else(|| {
                res.push(Node::new(b));
                res.len() - 1
            });
            res[i_a].paths.push(i_b);
            res[i_b].paths.push(i_a);
        });
        res
    }

    fn part1(&self, input: &Self::Input) -> String {
        let mut visited = vec![];
        let count = walk(input, 0, &mut visited, true);

        format!("{}", count)
    }

    fn part2(&self, input: &Self::Input) -> String {
        let mut visited = vec![];
        let count = walk(input, 0, &mut visited, false);

        format!("{}", count)
    }
}

fn walk(nodes: &[Node], node_idx: usize, visited: &mut Vec<usize>, revisited: bool) -> usize {
    let node = &nodes[node_idx];
    if node.name == "end" {
        return 1;
    }
    let revisited = if node.is_small && visited.contains(&node_idx) {
        if revisited {
            return 0;
        } else {
            if node.name == "start" {
                return 0;
            }
            true
        }
    } else {
        revisited
    };
    visited.push(node_idx);
    let mut counts = 0;
    for i in &node.paths {
        counts += walk(nodes, *i, visited, revisited);
    }
    visited.pop();
    counts
}
