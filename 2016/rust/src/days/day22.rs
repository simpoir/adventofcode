use std::collections::BinaryHeap;

#[derive(Default)]
pub struct Day {}

#[derive(PartialEq, Eq, Copy, Clone)]
pub struct Df(u16, u16, u16);
pub type Grid = [[Df; 33]; 33];

impl crate::cli::Day for Day {
    type Input = Grid;

    fn gen(&self, data: &str) -> Self::Input {
        let mut res: Grid = [[Df(0, 0, 0); 33]; 33];
        data.lines().skip(2).for_each(|l| {
            let mut chunks = l.split(' ').filter(|c| !c.is_empty());
            let mut pos = chunks.next().unwrap().split('-').skip(1);
            let x: usize = pos.next().unwrap()[1..].parse().unwrap();
            let y: usize = pos.next().unwrap()[1..].parse().unwrap();
            res[y][x] = Df(
                chunks
                    .next()
                    .unwrap()
                    .trim_end_matches('T')
                    .parse()
                    .unwrap(),
                chunks
                    .next()
                    .unwrap()
                    .trim_end_matches('T')
                    .parse()
                    .unwrap(),
                chunks
                    .next()
                    .unwrap()
                    .trim_end_matches('T')
                    .parse()
                    .unwrap(),
            );
        });
        res
    }

    fn part1(&self, input: &Self::Input) -> String {
        let mut count = 0;
        for a in input.iter().flatten() {
            for b in input.iter().flatten() {
                if can_move_to(a, b) {
                    count += 1;
                }
            }
        }
        count.to_string()
    }

    fn part2(&self, _input: &Self::Input) -> String {
        "".to_string()
    }
}

fn h(blank: (usize, usize), goal: (usize, usize)) -> usize {
    goal.0
        + goal.1
        + ((blank.0 as isize) - (goal.0 as isize).abs()) as usize
        + ((blank.1 as isize) - (goal.1 as isize).abs()) as usize
}

fn can_move_to(a: &Df, b: &Df) -> bool {
    a != b && a.1 != 0 && a.1 <= b.2
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part2() {
        let d: Day = Default::default();
        let input = "Filesystem            Size  Used  Avail  Use%
/dev/grid/node-x0-y0   10T    8T     2T   80%
/dev/grid/node-x0-y1   11T    6T     5T   54%
/dev/grid/node-x0-y2   32T   28T     4T   87%
/dev/grid/node-x1-y0    9T    7T     2T   77%
/dev/grid/node-x1-y1    8T    0T     8T    0%
/dev/grid/node-x1-y2   11T    7T     4T   63%
/dev/grid/node-x2-y0   10T    6T     4T   60%
/dev/grid/node-x2-y1    9T    8T     1T   88%
/dev/grid/node-x2-y2    9T    6T     3T   66%";
        let expected = "7";
        assert_eq!(expected, d.part2(&d.gen(input)));
    }
}
