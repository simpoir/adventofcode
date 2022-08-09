use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet, VecDeque},
    rc::Rc,
};

use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = &'i str;

    fn gen(&mut self, data: &'i str) -> Result<Self::Input> {
        Ok(data)
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        Ok((0..128)
            .map(|n| {
                knot_hash(&format!("{input}-{n}"))
                    .iter()
                    .copied()
                    .map(u8::count_ones)
                    .sum::<u32>()
            })
            .sum::<u32>()
            .to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let mut merged_colors: BTreeMap<u32, Rc<RefCell<BTreeSet<u32>>>> = BTreeMap::new();
        let mut next_color = 0;

        #[derive(Copy, Clone, Eq, PartialEq)]
        enum Cell {
            Empty,
            Blank,
            Colored(u32),
        }

        let mut grid: Vec<Vec<Cell>> = (0..128)
            .map(|n| {
                knot_hash(&format!("{input}-{n}"))
                    .iter()
                    .flat_map(|&b| {
                        (0u8..8).map(move |p| {
                            if (b << p) & 0x80 == 0x80 {
                                Cell::Blank
                            } else {
                                Cell::Empty
                            }
                        })
                    })
                    .collect()
            })
            .collect();

        // We iter on the grid, flooding groups with colors (u32).
        // When 2 colors collide, we merge colors.
        for j in 0..128 {
            for i in 0..128 {
                let cell = grid[j][i];
                let this_color: u32;
                grid[j][i] = match cell {
                    Cell::Blank => {
                        this_color = next_color;
                        next_color += 1;
                        merged_colors.insert(
                            this_color,
                            Rc::new(RefCell::new(BTreeSet::from_iter([this_color]))),
                        );
                        Cell::Colored(this_color)
                    }
                    Cell::Empty => continue,
                    Cell::Colored(color) => {
                        this_color = color;
                        Cell::Colored(this_color)
                    }
                };
                if i < 127 {
                    grid[j][i + 1] = match grid[j][i + 1] {
                        Cell::Empty => Cell::Empty,
                        Cell::Blank => Cell::Colored(this_color),
                        Cell::Colored(c) => {
                            if c != this_color {
                                let a = merged_colors.get(&c).unwrap();
                                let b = merged_colors.get(&this_color).unwrap();
                                if a != b {
                                    let a = a.clone();
                                    a.borrow_mut().append(&mut b.borrow_mut());
                                    for x in a.borrow().iter() {
                                        merged_colors.insert(*x, a.clone());
                                    }
                                }
                            }
                            Cell::Colored(c)
                        }
                    }
                }
                if j < 127 {
                    grid[j + 1][i] = match grid[j + 1][i] {
                        Cell::Empty => Cell::Empty,
                        Cell::Blank => Cell::Colored(this_color),
                        Cell::Colored(c) => {
                            if c != this_color {
                                let a = merged_colors.get(&c).unwrap();
                                let b = merged_colors.get(&this_color).unwrap();
                                if a != b {
                                    let a = a.clone();
                                    a.borrow_mut().append(&mut b.borrow_mut());
                                    for x in a.borrow().iter() {
                                        merged_colors.insert(*x, a.clone());
                                    }
                                }
                            }
                            Cell::Colored(c)
                        }
                    }
                }
            }
        }

        Ok(merged_colors
            .iter_mut()
            .map(|(&_k, v)| *v.borrow().iter().min().unwrap())
            .collect::<BTreeSet<u32>>()
            .len()
            .to_string())
    }
}

fn knot_hash(key: &str) -> Vec<u8> {
    let mut input: Vec<usize> = key.bytes().map(|b| b as usize).collect();
    input.append(&mut vec![17, 31, 73, 47, 23]);
    let mut rope = (0..256).collect::<VecDeque<usize>>();
    let mut skip = 0;
    let mut offset = 0;
    for _ in 0..64 {
        input.iter().for_each(|length| {
            let mut buf = vec![];
            for _ in 0..*length {
                buf.push(rope.pop_front().unwrap());
            }
            for x in buf.drain(..).rev() {
                rope.push_back(x);
            }
            rope.rotate_left(skip);
            offset += length + skip;
            skip = (skip + 1) % 256;
        });
    }
    rope.rotate_right(offset % 256);
    rope.iter()
        .copied()
        .flat_map(u8::try_from)
        .collect::<Vec<_>>()
        .chunks(16)
        .map(|w| w.iter().copied().reduce(|a, b| a ^ b).unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "flqrgnkx";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "8108";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "1242";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
