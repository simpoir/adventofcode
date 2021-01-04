use std::collections::{BTreeMap, BTreeSet};

day! {
    day24;
    type INPUT = Vec<Vec<Dir>>;

    fn gen(file: &mut impl BufRead) -> Result<Self::INPUT> {
        let mut res = vec![];
        for l in file.lines() {
            let mut row = vec![];
            let l = l?;
            let mut it = l.bytes();
            while let Some(c) = it.next() {
                row.push(match c {
                    b'e' => Dir::E,
                    b'w' => Dir::W,
                    b'n' => {
                        if let Some(b'e') = it.next() {
                            Dir::NE
                        } else {
                            Dir::NW
                        }
                    }
                    b's' => {
                        if let Some(b'e') = it.next() {
                            Dir::SE
                        } else {
                            Dir::SW
                        }
                    }
                    _ => unreachable!(),
                });
            }
            res.push(row);
        }
        Ok(res)
    }

    fn part1(input: &Self::INPUT) -> Result<String> {
        let black = layout(input);
        Ok(black.len().to_string())
    }

    fn part2(input: &Self::INPUT) -> Result<String> {
        let mut black = layout(input);
        let mut counts: BTreeMap<(isize, isize), usize> = BTreeMap::new();
        for _ in 0..100 {
            for (x, y) in &black {
                counts.entry((x-1, *y)).and_modify(|c| *c+=1).or_insert(1);
                counts.entry((x+1, *y)).and_modify(|c| *c+=1).or_insert(1);
                counts.entry((*x, y-1)).and_modify(|c| *c+=1).or_insert(1);
                counts.entry((x+1, y-1)).and_modify(|c| *c+=1).or_insert(1);
                counts.entry((x-1, y+1)).and_modify(|c| *c+=1).or_insert(1);
                counts.entry((*x, y+1)).and_modify(|c| *c+=1).or_insert(1);
            }
            let mut new_black = BTreeSet::new();
            for (pos, c) in &counts {
                if *c == 2 || (*c == 1 && black.contains(pos)) {
                    new_black.insert(*pos);
                }
            }
            counts.clear();
            black = new_black;
        }
        Ok(black.len().to_string())
    }
}

pub enum Dir {
    E,
    W,
    NE,
    NW,
    SE,
    SW,
}

fn layout(input: &[Vec<Dir>]) -> BTreeSet<(isize, isize)> {
    let mut black = BTreeSet::new();
    for row in input {
        let pos = row
            .iter()
            .map(|x| match x {
                Dir::E => (1, 0),
                Dir::W => (-1, 0),
                Dir::NE => (1, -1),
                Dir::NW => (0, -1),
                Dir::SE => (0, 1),
                Dir::SW => (-1, 1),
            })
            .fold((0, 0), |a, b| (a.0 + b.0, a.1 + b.1));
        if !black.remove(&pos) {
            black.insert(pos);
        }
    }
    black
}
