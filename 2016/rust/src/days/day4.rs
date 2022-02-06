use std::{cell::RefCell, collections::HashMap, rc::Rc};

type Room = (Vec<u8>, usize, Vec<u8>);

#[derive(Default)]
pub struct Day {
    valid: RefCell<Vec<Rc<Room>>>,
}

impl crate::cli::Day for Day {
    type Input = Vec<Rc<Room>>;

    fn gen(&self, data: &str) -> Self::Input {
        data.lines()
            .map(|l| {
                let l = l.trim_end_matches(']');
                let (head, sector) = l.rsplit_once('-').unwrap();
                let (sector, _) = sector.split_once('[').unwrap();
                let l = l.as_bytes();
                Rc::new((
                    head.bytes().collect(),
                    sector.parse().unwrap(),
                    l[(l.len() - 5)..].iter().copied().collect(),
                ))
            })
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> String {
        input
            .iter()
            .filter_map(|room| {
                let mut h = HashMap::new();
                for c in &room.0 {
                    h.entry(c).and_modify(|e| *e += 1).or_insert(1);
                }
                h.remove(&b'-');
                for i in room.2.iter().take(5) {
                    if i != *h
                        .iter()
                        .max_by(|x, y| match x.1.cmp(y.1) {
                            std::cmp::Ordering::Equal => y.0.cmp(x.0),
                            x => x,
                        })
                        .unwrap()
                        .0
                    {
                        return None;
                    }
                    h.remove(i);
                }
                self.valid.borrow_mut().push(room.clone());
                Some(room.1)
            })
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self, input: &Self::Input) -> String {
        if self.valid.borrow().is_empty() {
            self.part1(input);
        }

        for v in self.valid.borrow().iter() {
            if shift(&v.0, v.1) == "northpole object storage" {
                return v.1.to_string();
            };
        }
        unimplemented!();
    }
}

fn shift(input: &[u8], sector: usize) -> String {
    let mut name = String::new();
    for c in input {
        if *c == b'-' {
            name.push(' ');
        } else {
            name.push(((b'a' as usize + ((c - b'a') as usize + sector) % 26) as u8).into())
        }
    }
    name
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let d: Day = Default::default();
        let input = "aaaaa-bbb-z-y-x-123[abxyz]
a-b-c-d-e-f-g-h-987[abcde]
not-a-real-room-404[oarel]
totally-real-room-200[decoy]";
        let expected = "1514";
        assert_eq!(expected, d.part1(&d.gen(input)));
    }

    #[test]
    fn test_part2() {
        let input = "qzmt-zixmtkozy-ivhz";
        let expected = "very encrypted name";
        assert_eq!(expected, shift(input.as_bytes(), 343));
    }
}
