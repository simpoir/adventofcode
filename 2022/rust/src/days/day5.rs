use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = (Vec<Vec<char>>, Vec<(usize, usize, usize)>);

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        let (top, bottom) = data.split_once("\n\n").unwrap();
        let mut stacks = vec![
            vec![];
            top.lines()
                .last()
                .unwrap()
                .strip_suffix(' ')
                .unwrap()
                .rsplit_once(' ')
                .unwrap()
                .1
                .parse()
                .unwrap()
        ];
        for l in top.lines().take_while(|l| !l.starts_with(" 1")) {
            for (i, c) in l.chars().skip(1).step_by(4).enumerate() {
                if c != ' ' {
                    stacks[i].push(c);
                }
            }
        }
        let ops = bottom
            .lines()
            .map(|l| {
                let mut it = l.split_ascii_whitespace();
                (
                    it.nth(1).unwrap().parse().unwrap(),
                    it.nth(1).unwrap().parse::<usize>().unwrap() - 1,
                    it.nth(1).unwrap().parse::<usize>().unwrap() - 1,
                )
            })
            .collect();
        stacks.iter_mut().for_each(|s| s.reverse());
        Ok((stacks, ops))
    }

    fn part1(&mut self, (stacks, ops): &Self::Input) -> Result<String> {
        let mut stacks = stacks.clone();
        for &(amount, from, to) in ops {
            for _ in 0..amount {
                let v = stacks[from].pop().unwrap();
                stacks[to].push(v);
            }
        }
        Ok(stacks.iter().map(|s| s.last().unwrap()).collect())
    }

    fn part2(&mut self, (stacks, ops): &Self::Input) -> Result<String> {
        let mut stacks = stacks.clone();
        for &(amount, from, to) in ops {
            let top = stacks[from].len();
            let mut v = stacks[from].splice((top - amount)..top, []).collect();
            stacks[to].append(&mut v);
        }
        Ok(stacks.iter().map(|s| s.last().unwrap()).collect())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "CMZ";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "MCD";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
