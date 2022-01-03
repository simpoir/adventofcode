use std::ops::RangeInclusive;

#[derive(Default)]
pub struct Day {}

#[derive(Clone, Debug, PartialEq)]
pub struct Cub {
    x: RangeInclusive<isize>,
    y: RangeInclusive<isize>,
    z: RangeInclusive<isize>,
}

#[derive(Debug)]
pub struct Op {
    on: bool,
    cuboid: Cub,
}

impl crate::Day for Day {
    type Input = Vec<Op>;

    fn gen(&self, data: &str) -> Self::Input {
        data.trim_end()
            .lines()
            .map(|l| {
                let (on, ranges) = l.split_once(' ').unwrap();
                let on = on == "on";
                let ranges = ranges
                    .chars()
                    .filter(|x| !matches!(x, 'x' | 'y' | 'z' | '='))
                    .collect::<String>();
                let mut ranges = ranges.split(',');
                let xx = ranges.next().unwrap().split_once("..").unwrap();
                let x = xx.0.parse().unwrap()..=xx.1.parse().unwrap();
                let y = ranges.next().unwrap().split_once("..").unwrap();
                let y = y.0.parse().unwrap()..=y.1.parse().unwrap();
                let z = ranges.next().unwrap().split_once("..").unwrap();
                let z = z.0.parse().unwrap()..=z.1.parse().unwrap();
                Op {
                    on,
                    cuboid: Cub { x, y, z },
                }
            })
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> String {
        let mut chunks: Vec<Cub> = vec![];

        for Op { on, cuboid } in input {
            let mut fragmented = vec![];
            chunks = chunks
                .drain(..)
                .filter(|c| {
                    if c.intersects(cuboid) {
                        fragmented.append(&mut c.clone().fragment(cuboid));
                        false
                    } else {
                        true
                    }
                })
                .collect();
            chunks.append(&mut fragmented);
            if *on {
                chunks.push(cuboid.clone());
            }
        }
        let full: usize = chunks.iter().map(|c| c.area()).sum();

        let mut fragmented = vec![];
        let area = Cub {
            x: -50..=50,
            y: -50..=50,
            z: -50..=50,
        };
        chunks = chunks
            .drain(..)
            .filter(|c| {
                if c.intersects(&area) {
                    fragmented.append(&mut c.clone().fragment(&area));
                    false
                } else {
                    true
                }
            })
            .collect();
        chunks.append(&mut fragmented);
        let sub: usize = chunks.iter().map(|c| c.area()).sum();

        format!("{}", full - sub)
    }

    fn part2(&self, input: &Self::Input) -> String {
        let mut chunks: Vec<Cub> = vec![];
        for Op { on, cuboid } in input {
            let mut fragmented = vec![];
            chunks = chunks
                .drain(..)
                .filter(|c| {
                    if c.intersects(cuboid) {
                        fragmented.append(&mut c.clone().fragment(cuboid));
                        false
                    } else {
                        true
                    }
                })
                .collect();
            chunks.append(&mut fragmented);
            if *on {
                chunks.push(cuboid.clone());
            }
        }

        let res: usize = chunks.iter().map(|c| c.area()).sum();
        format!("{}", res)
    }
}

impl Cub {
    fn intersects(&self, other: &Cub) -> bool {
        (self.x.contains(other.x.start())
            || self.x.contains(other.x.end())
            || other.x.contains(self.x.start()))
            && (self.y.contains(other.y.start())
                || self.y.contains(other.y.end())
                || other.y.contains(self.y.start()))
            && (self.z.contains(other.z.start())
                || self.z.contains(other.z.end())
                || other.z.contains(self.z.start()))
    }

    fn fragment(self, other: &Cub) -> Vec<Cub> {
        let mut res = vec![self];
        let mut res: Vec<Cub> = res
            .drain(..)
            .map(|c| {
                if c.intersects(other) {
                    c.fragment_x(other)
                } else {
                    vec![c]
                }
            })
            .flatten()
            .collect();
        let mut res: Vec<Cub> = res
            .drain(..)
            .map(|c| {
                if c.intersects(other) {
                    c.fragment_y(other)
                } else {
                    vec![c]
                }
            })
            .flatten()
            .collect();
        let mut res: Vec<Cub> = res
            .drain(..)
            .map(|c| {
                if c.intersects(other) {
                    c.fragment_z(other)
                } else {
                    vec![c]
                }
            })
            .flatten()
            .collect();
        res.drain(..).filter(|c| !c.intersects(other)).collect()
    }

    fn fragment_x(self, other: &Cub) -> Vec<Cub> {
        let mut res = vec![];
        let Cub { mut x, y, z } = self;
        if x.contains(other.x.start()) && x.start() != other.x.start() {
            res.push(Cub {
                x: *x.start()..=(other.x.start() - 1),
                y: y.clone(),
                z: z.clone(),
            });
            x = *other.x.start()..=*x.end();
        }
        if x.contains(other.x.end()) && x.end() != other.x.end() {
            res.push(Cub {
                x: *x.start()..=*other.x.end(),
                y: y.clone(),
                z: z.clone(),
            });
            x = (*other.x.end() + 1)..=*x.end();
        }
        res.push(Cub { x, y, z });
        res
    }

    fn fragment_y(self, other: &Cub) -> Vec<Cub> {
        let mut res = vec![];
        let Cub { mut y, x, z } = self;
        if y.contains(other.y.start()) && y.start() != other.y.start() {
            res.push(Cub {
                x: x.clone(),
                y: *y.start()..=(other.y.start() - 1),
                z: z.clone(),
            });
            y = *other.y.start()..=*y.end();
        }
        if y.contains(other.y.end()) && y.end() != other.y.end() {
            res.push(Cub {
                x: x.clone(),
                y: *y.start()..=*other.y.end(),
                z: z.clone(),
            });
            y = (*other.y.end() + 1)..=*y.end();
        }
        res.push(Cub { y, x, z });
        res
    }

    fn fragment_z(self, other: &Cub) -> Vec<Cub> {
        let mut res = vec![];
        let Cub { mut z, x, y } = self;
        if z.contains(other.z.start()) && z.start() != other.z.start() {
            res.push(Cub {
                x: x.clone(),
                y: y.clone(),
                z: *z.start()..=(other.z.start() - 1),
            });
            z = *other.z.start()..=*z.end();
        }
        if z.contains(other.z.end()) && z.end() != other.z.end() {
            res.push(Cub {
                x: x.clone(),
                y: y.clone(),
                z: *z.start()..=*other.z.end(),
            });
            z = (*other.z.end() + 1)..=*z.end();
        }
        res.push(Cub { z, x, y });
        res
    }

    fn area(&self) -> usize {
        ((self.x.end() - self.x.start() + 1)
            * (self.y.end() - self.y.start() + 1)
            * (self.z.end() - self.z.start() + 1)) as usize
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn line(range: RangeInclusive<isize>) -> Cub {
        Cub {
            x: range,
            y: 0..=0,
            z: 0..=0,
        }
    }

    fn cub(range: RangeInclusive<isize>) -> Cub {
        Cub {
            x: range.clone(),
            y: range.clone(),
            z: range,
        }
    }

    #[test]
    fn test_lines() {
        // no overlap
        let a = line(0..=5);
        let b = line(10..=15);
        let expected = vec![a.clone()];
        assert_eq!(expected, a.fragment(&b));

        let a = line(0..=10);
        let b = line(5..=15);
        let expected = vec![line(0..=4)];
        assert_eq!(expected, a.fragment(&b));

        let a = line(5..=15);
        let b = line(0..=10);
        let expected = vec![line(11..=15)];
        assert_eq!(expected, a.fragment(&b));

        // a contains b
        let a = line(0..=15);
        let b = line(5..=10);
        let expected = vec![line(0..=4), line(11..=15)];
        assert_eq!(expected, a.fragment(&b));

        // b contains a
        let a = line(5..=10);
        let b = line(0..=15);
        assert!(a.fragment(&b).is_empty());
    }

    #[test]
    fn test_area() {
        assert_eq!(1, cub(1..=1).area());
        assert_eq!(8, cub(0..=1).area());
    }

    #[test]
    fn test_overlap() {
        // full overlap
        let a = cub(5..=10);
        let b = cub(5..=10);
        assert!(a.fragment(&b).is_empty());

        // inside overlap
        let a = cub(5..=10);
        let b = cub(0..=12);
        assert!(a.fragment(&b).is_empty());

        // outsize overlap
        let a = cub(0..=2);
        let b = cub(1..=1);
        let sum: usize = a.fragment(&b).iter().map(|c| c.area()).sum();
        assert_eq!(26, sum);
    }
}
