pub mod day10;
pub mod day11;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

day_mod! {
    day01;
    day! {
        type INPUT = Vec<u64>;

        fn gen(file: &mut impl BufRead) -> Result<Self::INPUT> {
            let mut res = String::new();
            file.read_to_string(&mut res)?;
            Ok(res.lines().map(|line| line.parse().unwrap()).collect())
        }

        fn part1(input: &Self::INPUT) -> Result<String> {
            for i in 0..input.len() {
                for j in i..input.len() {
                    if input[i] + input[j] == 2020 {
                        return Ok(format!("{}", input[i] * input[j]));
                    }
                }
            }
            unreachable!();
        }

        fn part2(input: &Self::INPUT) -> Result<String> {
            for i in 0..input.len() {
                for j in i..input.len() {
                    for k in i..input.len() {
                        if input[i] + input[j] + input[k] == 2020 {
                            return Ok(format!("{}", input[i] * input[j] * input[k]));
                        }
                    }
                }
            }
            unreachable!();
        }
    }
}

day_mod! {
    day02;
    day! {
        type INPUT = Vec<(usize, usize, char, String)>;

        fn gen(file: &mut impl BufRead) -> Result<Self::INPUT> {
            let mut res = String::new();
            file.read_to_string(&mut res)?;
            Ok(res
                .lines()
                .map(|line| {
                    let dash = line.find('-').unwrap();
                    let sp = line.find(' ').unwrap();
                    (
                        line[..dash].parse().unwrap(),
                        line[dash + 1..sp].parse().unwrap(),
                        line.chars().nth(sp + 1).unwrap(),
                        line[sp + 3..].into(),
                    )
                })
                .collect())
        }

        fn part1(input: &Self::INPUT) -> Result<String> {
            Ok(format!(
                "{}",
                input
                    .iter()
                    .filter(|(min, max, c, entry)| {
                        let count = entry.chars().filter(|x| x == c).count();
                        *min <= count && count <= *max
                    })
                    .count()
            ))
        }

        fn part2(input: &Self::INPUT) -> Result<String> {
            Ok(format!(
                "{}",
                input
                    .iter()
                    .filter(|(min, max, c, entry)| {
                        (entry.chars().nth(*min).unwrap() == *c)
                            ^ (entry.chars().nth(*max).unwrap() == *c)
                    })
                    .count()
            ))
        }
    }
}

day_mod! {
    day03;
    fn check(input: &Vec<String>, x: usize, y: usize) -> usize {
        let mut pos: (usize, usize) = (0, 0);
        let mut count = 0;
        let len = input[0].len();
        while pos.1 < input.len() {
            if input[pos.1].as_bytes()[pos.0 % len] == b'#' {
                count += 1;
            }
            pos = (pos.0 + x, pos.1 + y);
        }
        return count;
    }

    day! {
        type INPUT = Vec<String>;

        fn gen(file: &mut impl BufRead) -> Result<Self::INPUT> {
            let mut res = String::new();
            file.read_to_string(&mut res)?;
            Ok(res.lines().map(String::from).collect())
        }

        fn part1(input: &Self::INPUT) -> Result<String> {
            Ok(format!("{}", check(&input, 3, 1)))
        }

        fn part2(input: &Self::INPUT) -> Result<String> {
            Ok(format!(
                "{}",
                [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
                    .iter()
                    .map(|(x, y)| check(input, *x, *y))
                    .fold(1, |a, b| a * b)
            ))
        }
    }
}

day_mod! {
    day12;
    day! {
        type INPUT = Vec<String>;

        fn gen(file: &mut impl BufRead) -> Result<Self::INPUT> {
            let mut res = String::new();
            file.read_to_string(&mut res)?;
            Ok(res.lines().map(String::from).collect())
        }

        fn part1(input: &Self::INPUT) -> Result<String> {
            let mut heading = (1isize, 0isize);
            let mut pos = (0isize, 0isize);
            for line in input {
                let (dir, count) = line.split_at(1);
                let count: isize = count.parse().unwrap();
                pos = match (dir, count) {
                    ("N", y) => (pos.0, pos.1 + y),
                    ("S", y) => (pos.0, pos.1 - y),
                    ("E", x) => (pos.0 + x, pos.1),
                    ("W", x) => (pos.0 - x, pos.1),
                    ("F", n) => (pos.0 + heading.0 * n, pos.1 + heading.1 * n),
                    ("R", 90) | ("L", 270) => {
                        heading = (heading.1, -heading.0);
                        pos
                    }
                    ("L", 90) | ("R", 270) => {
                        heading = (-heading.1, heading.0);
                        pos
                    }
                    (_, 180) => {
                        heading = (-heading.0, -heading.1);
                        pos
                    }
                    _ => unreachable!(),
                };
            }
            Ok(format!("{}", pos.0.abs() + pos.1.abs()))
        }

        fn part2(input: &Self::INPUT) -> Result<String> {
            let mut heading = (10isize, 1isize);
            let mut pos = (0isize, 0isize);
            for line in input {
                let (dir, count) = line.split_at(1);
                let count: isize = count.parse().unwrap();
                heading = match (dir, count) {
                    ("N", y) => (heading.0, heading.1 + y),
                    ("S", y) => (heading.0, heading.1 - y),
                    ("E", x) => (heading.0 + x, heading.1),
                    ("W", x) => (heading.0 - x, heading.1),
                    ("R", 90) | ("L", 270) => (heading.1, -heading.0),
                    ("L", 90) | ("R", 270) => (-heading.1, heading.0),
                    ("F", n) => {
                        pos = (pos.0 + heading.0 * n, pos.1 + heading.1 * n);
                        heading
                    }
                    (_, 180) => (-heading.0, -heading.1),
                    _ => unreachable!(),
                };
            }
            Ok(format!("{}", pos.0.abs() + pos.1.abs()))
        }
    }
}

day_mod! {
    day13;
    #[test]
    fn test_solve() {
        assert_eq!(9, ppcm(3, 5, 0, 1));
        let vals = [(0, 3), (1, 5), (2, 7)];
        assert_eq!(9, solve(&vals[0..2]));
        assert_eq!(54, solve(&vals));
    }

    fn solve(it_off: &[(usize, usize)]) -> usize {
        let first = it_off[0];
        it_off[1..]
            .iter()
            .fold(first, |a, b| (ppcm(a.1, b.1, a.0, b.0), a.1 * b.1))
            .0
    }

    fn ppcm(a: usize, b: usize, a_off: usize, b_off: usize) -> usize {
        let mut x = a_off + b_off;
        while x % b != 0 {
            x += a;
        }
        x - b_off
    }

    day!{
        type INPUT = (usize, Vec<Option<usize>>);

        fn gen(file: &mut impl BufRead) -> Result<Self::INPUT> {
            let mut res = String::new();
            file.read_to_string(&mut res)?;
            let (l1, tail) = res.split_at(res.find("\n").unwrap());
            Ok((
                l1.parse().unwrap(),
                tail.trim()
                    .split(",")
                    .map(|x| match x {
                        "x" => None,
                        i => Some(i.parse().unwrap()),
                    })
                    .collect(),
            ))
        }

        fn part1(input: &Self::INPUT) -> Result<String> {
            let (ts, buses) = input;
            let nexts = buses.iter().filter_map(|x| *x).map(|b| {
                let n = b - (ts % b);
                (n, b)
            });
            let result = nexts.min_by_key(|b| b.0).unwrap();
            Ok(format!("{}", result.0 * result.1))
        }

        fn part2(input: &Self::INPUT) -> Result<String> {
            let off_fact: Vec<(usize, usize)> = input
                .1
                .iter()
                .enumerate()
                .filter_map(|x| match x.1 {
                    None => None,
                    Some(n) => Some((x.0, *n)),
                })
                .collect();
            // This works because we know bus number are primes between each
            // other (otherwise we'd get a much lower result).
            // Because of that fact, for a and b, we know their lowest common
            // multiple will be a*b. We also know they will have the required
            // relative offset only wonce within that slice.
            // Thus, we find that offset relative to the ab slice.
            // Then we chain with that slice size, the offset we found and
            // apply the next number/offset.
            //
            // The beauty of this method is the more numbers, the faster
            // we increment, as repeating cycles become of a+b+c+d+...
            let ts = solve(&off_fact);
            Ok(format!("{}", ts))
        }
    }
}

day_mod! {
    day14;
    use std::collections::BTreeMap;

    pub enum Cmd {
        SetMask(u64, u64),
        SetVal(u64, u64),
    }

    day! {
        type INPUT = Vec<Cmd>; // on, off (addr, val)

        fn gen(file: &mut impl BufRead) -> Result<Self::INPUT> {
            let mut data = String::new();
            file.read_to_string(&mut data)?;
            Ok(data
                .lines()
                .map(|l| {
                    if l.starts_with("mask") {
                        let mask = l[7..].bytes().fold((0, 0), |a, b| match b {
                            b'0' => ((a.0 << 1) + 1, a.1 << 1),
                            b'1' => (a.0 << 1, (a.1 << 1) + 1),
                            _ => (a.0 << 1, a.1 << 1),
                        });
                        Cmd::SetMask(mask.0, mask.1)
                    } else {
                        let s1 = l.find(']').unwrap();
                        let val = l[s1 + 4..].parse().unwrap();
                        Cmd::SetVal(l[4..s1].parse().unwrap(), val)
                    }
                })
                .collect())
        }

        fn part1(input: &Self::INPUT) -> Result<String> {
            let mut mem = BTreeMap::new();
            let mut masks = (0, 0);
            for cmd in input {
                match cmd {
                    Cmd::SetMask(on, off) => {
                        masks = (*on, *off);
                    }
                    Cmd::SetVal(addr, val) => {
                        mem.insert(addr, (val & !masks.0) | masks.1);
                    }
                }
            }
            Ok(format!("{:?}", mem.iter().map(|v| *v.1).sum::<u64>()))
        }

        fn part2(input: &Self::INPUT) -> Result<String> {
            let mut mem = BTreeMap::new();
            let mut masks = (0, 0);
            for cmd in input {
                match cmd {
                    Cmd::SetMask(on, off) => {
                        masks = (*on, *off);
                    }
                    Cmd::SetVal(addr, val) => {
                        let float_mask =
                            !(masks.0 | masks.1) & 0b111111111111111111111111111111111111;
                        let floats: Vec<u64> = (0..36)
                            .filter_map(|i| {
                                if (float_mask >> i) & 1 == 1 {
                                    Some(i)
                                } else {
                                    None
                                }
                            })
                            .collect();
                        let float_count: u64 = 1 << floats.len();
                        let addr = addr & masks.0; // filter zeros
                        let addr = addr | masks.1; // set ones
                        for i in 0..float_count {
                            let floated = floats
                                .iter()
                                .enumerate()
                                .fold(0, |a, (bit, off)| a | (((i >> bit) & 1) << off))
                                as u64;
                            mem.insert(addr | floated, val);
                        }
                    }
                }
            }
            Ok(format!("{:?}", mem.iter().map(|v| *v.1).sum::<u64>()))
        }
    }
}

day_mod! {
    day15;

    fn play(input: &[u64], nth: usize) -> u64 {
        let mut known = vec![0u64; nth];
        let mut round = 1;
        let mut prev = input[0];
        for i in 1..input.len() {
            round += 1;
            known[prev as usize] = i as u64;
            prev = input[i];
        }

        while round != nth {
            let say = match known[prev as usize] {0 => 0, x => (round as u64) - x};
            known[prev as usize] = round as u64;
            round += 1;
            prev = say;
        }
        return prev
    }

    day! {
        type INPUT = Vec<u64>;

        fn gen(file: &mut impl BufRead) -> Result<Self::INPUT> {
            let mut input = String::new();
            file.read_to_string(&mut input).unwrap();
            Ok(input.trim().split(',').map(|x| x.parse().unwrap()).collect())
        }

        fn part1(input: &Self::INPUT) -> Result<String> {
            Ok(format!("{}", play(&input[..], 2020)))
        }

        fn part2(input: &Self::INPUT) -> Result<String> {
            Ok(format!("{}", play(&input[..], 30000000)))
        }
    }
}
