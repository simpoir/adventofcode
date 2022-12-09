use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<(char, u8)>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        data.lines()
            .map(|l| {
                let (dir, count) = l.split_once(' ').unwrap();
                Ok((dir.chars().next().unwrap(), count.parse()?))
            })
            .collect()
    }

    fn part1(&mut self, moves: &Self::Input) -> Result<String> {
        Ok(sim::<1000, 2>(moves).to_string())
    }

    fn part2(&mut self, moves: &Self::Input) -> Result<String> {
        Ok(sim::<1000, 10>(moves).to_string())
    }
}

fn sim<const N: usize, const L: usize>(moves: &[(char, u8)]) -> usize {
    let mut visited = vec![vec![false; N]; N];

    let mut rope = [(N as isize / 2, N as isize / 2); L];

    for (dir, count) in moves {
        for _ in 0..*count {
            let mut head = &mut rope[0];
            match dir {
                'U' => head.1 += 1,
                'D' => head.1 -= 1,
                'L' => head.0 -= 1,
                'R' => head.0 += 1,
                _ => unimplemented!(),
            }

            let mut head = *head;
            for tail in rope.iter_mut().skip(1) {
                let mx = head.0.abs_diff(tail.0) == 2;
                let my = head.1.abs_diff(tail.1) == 2;
                if !(mx || my) {
                    // noop
                } else if tail.0 == head.0 && my {
                    tail.1 += (head.1 - tail.1).signum();
                } else if tail.1 == head.1 && mx {
                    tail.0 += (head.0 - tail.0).signum();
                } else {
                    tail.0 += (head.0 - tail.0).signum();
                    tail.1 += (head.1 - tail.1).signum();
                }
                head = *tail;
            }

            let tail = rope.last().unwrap();
            visited[tail.1 as usize][tail.0 as usize] = true;
        }
    }
    visited.iter().flatten().filter(|x| **x).count()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "13";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "36";
        let input = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
