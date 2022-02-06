#[derive(Default)]
pub struct Day {}

pub enum Op {
    Rect(usize, usize),
    Rx(usize, usize),
    Ry(usize, usize),
}

impl crate::cli::Day for Day {
    type Input = Vec<Op>;

    fn gen(&self, data: &str) -> Self::Input {
        data.lines()
            .map(|l| {
                if l.starts_with("rect") {
                    let (_, tail) = l.rsplit_once(' ').unwrap();
                    let (x, y) = tail.split_once('x').unwrap();
                    Op::Rect(x.parse().unwrap(), y.parse().unwrap())
                } else if l.starts_with("rotate column") {
                    let mut chunks = l.rsplit(' ');
                    let n = chunks.next().unwrap().parse().unwrap();
                    let x = chunks.nth(1).unwrap().get(2..).unwrap().parse().unwrap();
                    Op::Ry(x, n)
                } else {
                    let mut chunks = l.rsplit(' ');
                    let n = chunks.next().unwrap().parse().unwrap();
                    let y = chunks.nth(1).unwrap().get(2..).unwrap().parse().unwrap();
                    Op::Rx(y, n)
                }
            })
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> String {
        let res = exe::<50, 6>(input);
        res.iter()
            .flatten()
            .filter(|b| **b == b'#')
            .count()
            .to_string()
    }

    fn part2(&self, input: &Self::Input) -> String {
        exe::<50, 6>(input)
            .map(|l| String::from_utf8_lossy(&l).to_string())
            .join("\n")
    }
}

fn exe<const W: usize, const H: usize>(ops: &[Op]) -> [[u8; W]; H] {
    let mut grid = [[b' '; W]; H];
    for op in ops {
        match op {
            Op::Rect(x, y) => (0..*x).for_each(|x| (0..*y).for_each(|y| grid[y][x] = b'#')),
            Op::Rx(y, n) => {
                let mut buf = [0u8; W];
                buf.iter_mut()
                    .enumerate()
                    .for_each(|(i, x)| *x = grid[*y][(i + W - n) % W]);
                grid[*y] = buf;
            }
            Op::Ry(x, n) => {
                let mut buf = [0u8; H];
                buf.iter_mut()
                    .enumerate()
                    .for_each(|(i, y)| *y = grid[(i + H - n) % H][*x]);
                buf.iter()
                    .enumerate()
                    .for_each(|(y, val)| grid[y][*x] = *val);
            }
        }
    }
    grid
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let d: Day = Default::default();
        let input = "rect 3x2
rotate column x=1 by 1
rotate row y=0 by 4
rotate column x=1 by 1";
        let expected = [*b" #  # #", *b"# #    ", *b" #     "];
        assert_eq!(expected, exe::<7, 3>(&d.gen(input)));
    }
}
