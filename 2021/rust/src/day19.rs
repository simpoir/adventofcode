use std::{cell::RefCell, collections::VecDeque, io::Write};

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct Point(isize, isize, isize);
pub type Scan = Vec<Point>;

enum Rot {
    X,
    Y,
    Z,
}

pub struct Day {
    offsets: RefCell<Scan>,
}

const ALL_POS: [Rot; 24] = [
    Rot::Z,
    Rot::Z,
    Rot::Z,
    Rot::X,
    Rot::Y,
    Rot::Y,
    Rot::Y,
    Rot::Z,
    Rot::X,
    Rot::X,
    Rot::X,
    Rot::Z,
    Rot::Y,
    Rot::Y,
    Rot::Y,
    Rot::Z,
    Rot::X,
    Rot::X,
    Rot::X,
    Rot::Z,
    Rot::X,
    Rot::Z,
    Rot::Z,
    Rot::Z,
];

impl Default for Day {
    fn default() -> Self {
        Self {
            offsets: RefCell::new(vec![]),
        }
    }
}

impl crate::Day for Day {
    type Input = Vec<Scan>;

    fn gen(&self, data: &str) -> Self::Input {
        let mut scanners = vec![];
        let mut scanner = vec![];
        for l in data.trim_end().lines().skip(1) {
            if l.starts_with("---") {
                continue;
            }
            if l.is_empty() {
                scanners.push(scanner);
                scanner = vec![];
                continue;
            }
            let mut it = l.splitn(3, ',');
            scanner.push(Point(
                it.next().unwrap().parse().unwrap(),
                it.next().unwrap().parse().unwrap(),
                it.next().unwrap().parse().unwrap(),
            ));
        }
        scanners.push(scanner);
        scanners
    }

    fn part1(&self, input: &Self::Input) -> String {
        // all points relative to scan 0
        let mut abs = vec![];
        let mut remaining: VecDeque<Scan> = VecDeque::from_iter(input[1..].iter().cloned());
        abs.extend(input[0].iter().cloned());
        self.offsets.borrow_mut().push(Point(0, 0, 0));

        'matching: while let Some(mut scan) = remaining.pop_front() {
            print!(
                "{}unmatched scanners: {} ",
                ansi_escapes::CursorTo::AbsoluteX(0),
                remaining.len() + 1
            );
            std::io::stdout().flush().unwrap();

            let (matching, offset) = best_match(abs.iter().rev(), &scan);
            if matching >= 12 {
                scan.iter().map(|n| n + &offset).for_each(|n| {
                    if !abs.contains(&n) {
                        abs.push(n)
                    }
                });
                self.offsets.borrow_mut().push(offset);
                continue 'matching;
            }
            for dir in ALL_POS {
                (0..scan.len()).for_each(|i| {
                    scan[i] = rot3d(&scan[i], &dir);
                });
                let (matching, offset) = best_match(abs.iter().rev(), &scan);
                if matching >= 12 {
                    scan.iter().map(|n| n + &offset).for_each(|n| {
                        if !abs.contains(&n) {
                            abs.push(n)
                        }
                    });
                    self.offsets.borrow_mut().push(offset);
                    continue 'matching;
                }
            }
            remaining.push_back(scan);
        }

        print!(
            "{}{}",
            ansi_escapes::CursorTo::AbsoluteX(0),
            ansi_escapes::EraseLine
        );
        let res = abs.len();
        format!("{}", res)
    }

    fn part2(&self, input: &Self::Input) -> String {
        let mut res: usize = 0;
        if self.offsets.borrow().is_empty() {
            self.part1(input);
        }

        let o = self.offsets.borrow();
        for a in o.iter() {
            for b in o.iter() {
                res = res.max(manhattan(a, b));
            }
        }
        format!("{}", res)
    }
}

fn best_match<'a, T>(a: T, b: &[Point]) -> (usize, Point)
where
    T: Iterator<Item = &'a Point> + Clone,
{
    let mut best = 0;
    let mut best_offset = Point(0, 0, 0);
    for ref_a in a.clone() {
        for ref_b in b.iter() {
            let offset = ref_a - ref_b;
            let mut matching = 0;
            'point: for bbb in b.iter() {
                for aaa in a.clone() {
                    if bbb + &offset == *aaa {
                        matching += 1;
                        continue 'point;
                    }
                }
            }
            if matching > best {
                if matching >= 12 {
                    return (matching, offset);
                }
                best = matching;
                best_offset = offset;
            }
        }
    }
    (best, best_offset)
}

fn rot3d(p: &Point, dir: &Rot) -> Point {
    match dir {
        Rot::Z => Point(p.1, -p.0, p.2),
        Rot::X => Point(p.0, p.2, -p.1),
        Rot::Y => Point(p.2, p.1, -p.0),
    }
}

fn manhattan(a: &Point, b: &Point) -> usize {
    ((a.0 - b.0).abs() + (a.1 - b.1).abs() + (a.2 - b.2).abs()) as usize
}

impl std::ops::Add for &Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl std::ops::Sub for &Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_matching() {
        let a = [Point(0, 2, 0), Point(4, 1, 0), Point(3, 3, 0)];
        let b = [Point(-1, -1, 0), Point(-5, 0, 0), Point(-2, 1, 0)];
        let (m, offset) = best_match(a.iter(), &b);
        assert_eq!(3, m);
        assert_eq!(Point(5, 2, 0), offset);

        let a = [Point(0, 2, 0), Point(4, 1, 0), Point(3, 3, 0)];
        let b = [Point(0, 0, 1), Point(-5, 0, 0), Point(-2, 1, 0)];
        let (m, offset) = best_match(a.iter(), &b);
        assert_eq!(2, m);
        assert_eq!(Point(5, 2, 0), offset);
    }
}
