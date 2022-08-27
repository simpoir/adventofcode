use std::collections::HashMap;

use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

const GRASS: u8 = b'.';
const TREE: u8 = b'|';
const YARD: u8 = b'#';

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<Vec<u8>>;

    fn gen(&mut self, data: &'i str) -> Result<Self::Input> {
        Ok(data.lines().map(|l| l.bytes().collect()).collect())
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let mut map = vec![vec![GRASS; input[0].len() + 2]];
        map.extend(input.iter().map(|l| {
            let mut v = vec![GRASS];
            v.extend_from_slice(l);
            v.push(GRASS);
            v
        }));
        map.push(vec![GRASS; input[0].len() + 2]);

        for _ in 0..10 {
            map = step(&map);
        }

        Ok((map.iter().flatten().filter(|c| **c == TREE).count()
            * map.iter().flatten().filter(|c| **c == YARD).count())
        .to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let mut map = vec![vec![GRASS; input[0].len() + 2]];
        map.extend(input.iter().map(|l| {
            let mut v = vec![GRASS];
            v.extend_from_slice(l);
            v.push(GRASS);
            v
        }));
        map.push(vec![GRASS; input[0].len() + 2]);

        let mut seen = HashMap::new();

        for i in 0..1_000_000_000 {
            if let Some(prev) = seen.insert(map.clone(), i) {
                let remainder = 1_000_000_000 - i;
                let cycles = i - prev;
                let missing = remainder % cycles;
                for _ in 0..missing {
                    map = step(&map);
                }
                break;
            }
            map = step(&map);
        }

        Ok((map.iter().flatten().filter(|c| **c == TREE).count()
            * map.iter().flatten().filter(|c| **c == YARD).count())
        .to_string())
    }
}

fn step(map: &[Vec<u8>]) -> Vec<Vec<u8>> {
    let mut new_map = map.to_owned();
    map.windows(3).enumerate().for_each(|(y, rows)| {
        let a = rows[0].windows(3).enumerate();
        let mut b = rows[1].windows(3);
        let mut c = rows[2].windows(3);
        for (x, a) in a {
            let mut trees = 0;
            let mut yards = 0;
            let mut grasses = 0;
            a.iter()
                .chain(b.next().unwrap().iter())
                .chain(c.next().unwrap().iter())
                .for_each(|c| match *c {
                    GRASS => grasses += 1,
                    TREE => trees += 1,
                    YARD => yards += 1,
                    _ => unreachable!(),
                });
            match &mut new_map[y + 1][x + 1] {
                c @ &mut GRASS => {
                    if trees >= 3 {
                        *c = TREE
                    }
                }
                c @ &mut TREE => {
                    if yards >= 3 {
                        *c = YARD
                    }
                }
                c @ &mut YARD => {
                    if yards < 2 || trees < 1 {
                        *c = GRASS
                    }
                }
                _ => unreachable!(),
            }
        }
    });
    new_map
}

#[allow(unused)]
fn dbg_map(map: &[Vec<u8>]) {
    println!();
    for l in map {
        println!("{}", String::from_utf8_lossy(l));
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let input = "\
.#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";
        let expected = "1147";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }
}
