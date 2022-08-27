use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

#[derive(Clone, Copy, Debug)]
pub struct Entity {
    hp: u8,
    id: u8,
}

#[derive(Clone, Copy, Debug)]
pub enum Cell {
    Wall,
    Space,
    Goblin(Entity),
    Elf(Entity),
}

impl Cell {
    fn is_elf(&self) -> bool {
        matches!(self, Cell::Elf(_))
    }

    fn is_goblin(&self) -> bool {
        matches!(self, Cell::Goblin(_))
    }

    fn id(&self) -> u8 {
        match self {
            Cell::Goblin(e) => e.id,
            Cell::Elf(e) => e.id,
            _ => 0,
        }
    }

    /// take damage, return true if dead
    fn dmg(&mut self, elven_power: u8) -> bool {
        match self {
            Cell::Goblin(e) => {
                e.hp = e.hp.saturating_sub(elven_power);
                e.hp == 0
            }
            Cell::Elf(e) => {
                e.hp = e.hp.saturating_sub(3);
                e.hp == 0
            }
            _ => false,
        }
    }
}

pub type Map = Vec<Vec<Cell>>;

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Map;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        let mut i = 0;
        Ok(data
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '#' => Cell::Wall,
                        '.' => Cell::Space,
                        'E' => {
                            i += 1;
                            Cell::Elf(Entity { hp: 200, id: i })
                        }
                        'G' => {
                            i += 1;
                            Cell::Goblin(Entity { hp: 200, id: i })
                        }
                        _ => unimplemented!(),
                    })
                    .collect()
            })
            .collect())
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let mut map = input.clone();
        let rounds = run(&mut map, 3);
        Ok((rounds
            * map
                .iter()
                .flatten()
                .map(|c| match c {
                    Cell::Goblin(g) => g.hp as u32,
                    _ => 0u32,
                })
                .sum::<u32>())
        .to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let n_elf = input.iter().flatten().filter(|c| c.is_elf()).count();
        for i in 4.. {
            let mut map = input.clone();
            let rounds = run(&mut map, i);
            if n_elf == map.iter().flatten().filter(|c| c.is_elf()).count() {
                return Ok((rounds
                    * map
                        .iter()
                        .flatten()
                        .map(|c| match c {
                            Cell::Elf(e) => e.hp as u32,
                            _ => 0u32,
                        })
                        .sum::<u32>())
                .to_string());
            }
        }
        unreachable!()
    }
}

fn run(map: &mut Map, elven_power: u8) -> u32 {
    for i in 0.. {
        let mut dead = vec![];
        let order: Vec<(usize, usize, u8)> = map
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(x, cell)| (x, y, cell))
                    .filter(|(_, _, cell)| matches!(cell, Cell::Goblin(_) | Cell::Elf(_)))
            })
            .map(|(x, y, cell)| (x, y, cell.id()))
            .collect();

        for (mut x, mut y, id) in order {
            // move
            if dead.contains(&id) {
                continue;
            }
            if let Some(step) = find_nearest(!map[y][x].is_elf(), map, (x, y)) {
                map[step.1][step.0] = map[y][x];
                map[y][x] = Cell::Space;
                (x, y) = step;
            }
            if map
                .iter()
                .flatten()
                .filter(|e| {
                    map[y][x].is_elf() && e.is_goblin() || map[y][x].is_goblin() && e.is_elf()
                })
                .count()
                == 0
            {
                return i;
            }

            // attack
            if let Some((x, y)) = find_target(!map[y][x].is_elf(), map, (x, y)) {
                if map[y][x].dmg(elven_power) {
                    dead.push(map[y][x].id());
                    map[y][x] = Cell::Space;
                }
            }
        }
    }
    unreachable!()
}

fn find_nearest(elf: bool, map: &Map, pos: (usize, usize)) -> Option<(usize, usize)> {
    let mut q = vec![(None, pos)];
    let mut targets = vec![];
    let mut walked = vec![vec![false; map[0].len()]; map.len()];
    walked[pos.1][pos.0] = true;

    while targets.is_empty() && !q.is_empty() {
        let mut new_q = vec![];
        for (pos0, (x, y)) in q {
            for (dx, dy) in [(0isize, -1isize), (-1, 0), (1, 0), (0, 1)] {
                let x = ((x as isize + dx).max(0) as usize).min(map[0].len());
                let y = ((y as isize + dy).max(0) as usize).min(map.len());
                if walked[y][x] {
                    continue;
                }
                walked[y][x] = true;
                match (elf, &map[y][x]) {
                    (true, Cell::Elf(_)) | (false, Cell::Goblin(_)) => {
                        targets.push((pos0, (x, y)));
                    }
                    (_, Cell::Space) => {
                        let pos0 = pos0.or(Some((x, y)));
                        new_q.push((pos0, (x, y)));
                    }
                    _ => continue,
                }
            }
        }
        q = new_q;
    }
    targets
        .iter()
        .min_by(|a, b| a.1 .1.cmp(&b.1 .1).then_with(|| a.1 .0.cmp(&b.1 .0)))
        .and_then(|(step0, _tgt)| *step0)
}

fn find_target(elf: bool, map: &Map, (x, y): (usize, usize)) -> Option<(usize, usize)> {
    let mut targets = vec![];
    for (dx, dy) in [(0isize, -1isize), (-1, 0), (1, 0), (0, 1)] {
        let x = ((x as isize + dx).max(0) as usize).min(map[0].len());
        let y = ((y as isize + dy).max(0) as usize).min(map.len());
        match (elf, &map[y][x]) {
            (true, Cell::Elf(e)) | (false, Cell::Goblin(e)) => {
                targets.push((x, y, e));
            }
            _ => continue,
        }
    }
    targets
        .iter()
        .min_by(|a, b| {
            a.2.hp
                .cmp(&b.2.hp)
                .then_with(|| a.1.cmp(&b.1))
                .then_with(|| a.0.cmp(&b.0))
        })
        .map(|(x, y, _)| (*x, *y))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const A: &str = "\
#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "27730";
        let data = d.gen(A).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    const B: &str = "\
#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";

    #[test]
    fn test_part1_large() {
        let mut d: Day = Default::default();
        let expected = "18740";
        let data = d.gen(B).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    const C: &str = "\
#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######";

    #[test]
    fn test_part1_maze() {
        let mut d: Day = Default::default();
        let expected = "28944";
        let data = d.gen(C).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    const D: &str = "\
#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######";

    #[test]
    fn test_part1_crowded() {
        let mut d: Day = Default::default();
        let expected = "27755";
        let data = d.gen(D).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2a() {
        let mut d: Day = Default::default();
        let expected = "4988";
        let data = d.gen(A).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }

    #[test]
    fn test_part2b() {
        let mut d: Day = Default::default();
        let expected = "1140";
        let data = d.gen(B).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }

    #[test]
    fn test_part2c() {
        let mut d: Day = Default::default();
        let expected = "6474";
        let data = d.gen(C).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }

    #[test]
    fn test_part2d() {
        let mut d: Day = Default::default();
        let expected = "3478";
        let data = d.gen(D).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
