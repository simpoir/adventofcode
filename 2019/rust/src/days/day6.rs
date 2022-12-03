use std::collections::HashMap;

use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<(&'i str, &'i str)>;

    fn gen(&mut self, data: &'i str) -> Result<Self::Input> {
        Ok(data.lines().map(|l| l.split_once(')').unwrap()).collect())
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let mut index = HashMap::<_, Vec<_>>::new();
        for &(parent, child) in input {
            index.entry(parent).or_default().push(child);
        }

        fn walk(node: &str, index: &HashMap<&str, Vec<&str>>, depth: usize) -> usize {
            depth
                + index
                    .get(node)
                    .unwrap_or(&vec![])
                    .iter()
                    .map(|n| walk(n, index, depth + 1))
                    .sum::<usize>()
        }

        Ok(walk("COM", &index, 0).to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let mut index = HashMap::new();
        for &(parent, child) in input {
            index.insert(child, parent);
        }

        let mut you_steps = vec![];
        let mut steps = 0;
        let mut current = index.get("YOU").unwrap();
        while *current != "COM" {
            you_steps.push((*current, steps));
            steps += 1;
            current = index.get(current).unwrap();
        }

        let mut san_steps = vec![];
        let mut steps = 0;
        let mut current = index.get("SAN").unwrap();
        while *current != "COM" {
            san_steps.push((*current, steps));
            steps += 1;
            current = index.get(current).unwrap();
        }

        let result: i32 = you_steps
            .iter()
            .flat_map(|(a, a_steps)| {
                san_steps.iter().filter_map(move |(b, b_steps)| {
                    if a == b {
                        Some(a_steps + b_steps)
                    } else {
                        None
                    }
                })
            })
            .min()
            .unwrap();

        Ok(result.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";
        let expected = "42";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let input = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";
        let expected = "4";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
