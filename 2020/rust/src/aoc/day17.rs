use itertools::iproduct;
use std::collections::BTreeMap;

day! {
    day17;
    type INPUT = BTreeMap<(isize, isize), u8>;

    fn gen(file: &mut impl BufRead) -> Result<Self::INPUT> {
        let mut res = BTreeMap::new();
        for (y, l) in file.lines().enumerate() {
            for (x, c) in l?.chars().enumerate() {
                if c == '#' {
                    res.insert((x as isize, y as isize), 0);
                }
            }
        }
        Ok(res)
    }

    fn part1(cubes: &Self::INPUT) -> Result<String> {
        type CubeMap = BTreeMap<(isize, isize, isize), u8>;

        let adj: Vec<(isize, isize, isize)> = iproduct!(-1..=1, -1..=1, -1..=1).filter(|x| *x != (0, 0, 0)).collect();
        let mut cubes: CubeMap = cubes.iter().map(|(k, _v)| ((k.0, k.1, 0), 0)).collect();

        for _ in 0..6 {
            let mut cubes1 = CubeMap::new();
            for p in cubes.keys() {
                for inc in &adj {
                    let pp = (p.0 + inc.0, p.1 + inc.1, p.2 + inc.2);
                    cubes1.entry(pp).and_modify(|e| *e += 1).or_insert(1);
                }
            }
            cubes = cubes1.iter().filter_map(|(p, count)| {
                if *count == 3 || (*count == 2 && cubes.contains_key(p)) {
                    Some((*p, *count))
                } else {
                    None
                }
            }).collect();
        }
        Ok(format!("{}", cubes.len()))
    }

    fn part2(cubes: &Self::INPUT) -> Result<String> {
        type CubeMap = BTreeMap<(isize, isize, isize, isize), u8>;

        let adj: Vec<(isize, isize, isize, isize)> = iproduct!(-1..=1, -1..=1, -1..=1, -1..=1).filter(|x| *x != (0, 0, 0, 0)).collect();
        let mut cubes: CubeMap = cubes.iter().map(|(k, _v)| ((k.0, k.1, 0, 0), 0)).collect();

        for _ in 0..6 {
            let mut cubes1 = CubeMap::new();
            for p in cubes.keys() {
                for inc in &adj {
                    let pp = (p.0 + inc.0, p.1 + inc.1, p.2 + inc.2, p.3 + inc.3);
                    cubes1.entry(pp).and_modify(|e| *e += 1).or_insert(1);
                }
            }
            cubes = cubes1.iter().filter_map(|(p, count)| {
                if *count == 3 || (*count == 2 && cubes.contains_key(p)) {
                    Some((*p, *count))
                } else {
                    None
                }
            }).collect();
        }
        Ok(format!("{}", cubes.len()))
    }

}
