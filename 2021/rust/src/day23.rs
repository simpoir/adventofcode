#[derive(Default)]
pub struct Day {}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Kind {
    A,
    B,
    C,
    D,
}

impl From<u8> for Kind {
    fn from(x: u8) -> Self {
        match x {
            b'A' => Kind::A,
            b'B' => Kind::B,
            b'C' => Kind::C,
            b'D' => Kind::D,
            _ => unimplemented!(),
        }
    }
}

impl Kind {
    const fn tgt_x(self) -> usize {
        match self {
            Kind::A => 2,
            Kind::B => 4,
            Kind::C => 6,
            Kind::D => 8,
        }
    }

    const fn score(self) -> usize {
        match self {
            Kind::A => 1,
            Kind::B => 10,
            Kind::C => 100,
            Kind::D => 1000,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Cell {
    Free,
    Blocked,
    Contains(Kind),
}

pub type Map<const N: usize> = [[Cell; 11]; N];

impl crate::Day for Day {
    type Input = Map<3>;

    fn gen(&self, data: &str) -> Self::Input {
        let lines: Vec<&str> = data.lines().map(|l| &l[1..]).collect();
        let mut res: Map<3> = [[Cell::Blocked; 11]; 3];
        for (y, l) in lines[1..=3].iter().enumerate() {
            for x in 0..11 {
                let b = l.as_bytes().get(x);
                res[y][x] = match b {
                    Some(b'#' | b' ') => Cell::Blocked,
                    Some(b'.') => Cell::Free,
                    Some(b) => Cell::Contains((*b).into()),
                    _ => continue,
                };
            }
        }
        // doorways are blocked
        for x in 0..11 {
            if res[1][x] != Cell::Blocked {
                res[0][x] = Cell::Blocked;
            }
        }

        res
    }

    fn part1(&self, input: &Self::Input) -> String {
        let unset_pods = get_unset_pods(input);
        let res = std::cell::Cell::new(999999);
        walk_pods(input, &unset_pods, 0, &res);
        format!("{}", res.get())
    }

    fn part2(&self, input: &Self::Input) -> String {
        let input: Map<5> = [
            input[0],
            input[1],
            [
                Cell::Blocked,
                Cell::Blocked,
                Cell::Contains(b'D'.into()),
                Cell::Blocked,
                Cell::Contains(b'C'.into()),
                Cell::Blocked,
                Cell::Contains(b'B'.into()),
                Cell::Blocked,
                Cell::Contains(b'A'.into()),
                Cell::Blocked,
                Cell::Blocked,
            ],
            [
                Cell::Blocked,
                Cell::Blocked,
                Cell::Contains(b'D'.into()),
                Cell::Blocked,
                Cell::Contains(b'B'.into()),
                Cell::Blocked,
                Cell::Contains(b'A'.into()),
                Cell::Blocked,
                Cell::Contains(b'C'.into()),
                Cell::Blocked,
                Cell::Blocked,
            ],
            input[2],
        ];
        let unset_pods = get_unset_pods(&input);
        let res = std::cell::Cell::new(999999);
        walk_pods(&input, &unset_pods, 0, &res);
        format!("{}", res.get())
    }
}

/// walk all pods paths recursively
fn walk_pods<const N: usize>(
    map: &Map<N>,
    pods: &[(Kind, usize, usize)],
    score: usize,
    best: &std::cell::Cell<usize>,
) {
    if best.get() <= score {
        return;
    }
    if pods.is_empty() {
        best.set(score);
        return;
    }

    for (pod_idx, pod) in pods.iter().enumerate() {
        let (kind, x, y) = *pod;
        if score == 0 {
            // slow loop feedback
            print!(
                "{}{}Walking pod {}",
                ansi_escapes::CursorTo::AbsoluteX(0),
                ansi_escapes::EraseLine,
                pods.len() - pod_idx
            );
            std::io::Write::flush(&mut std::io::stdout()).unwrap();
        }

        // in corridor
        if y == 0 {
            let tgt_x = kind.tgt_x();
            // door is free
            let tgt_y = match get_door_depth(kind, map) {
                None => continue,
                Some(n) => n,
            };
            let range = if (x..=tgt_x).is_empty() {
                tgt_x..=(x - 1)
            } else {
                (x + 1)..=tgt_x
            };
            if !map[0][range]
                .iter()
                .all(|c| matches!(c, Cell::Free | Cell::Blocked))
            {
                continue;
            }
            let score =
                score + kind.score() * (tgt_y + (tgt_x as isize - x as isize).abs() as usize);
            let mut map = *map;
            map[0][x] = Cell::Free;
            map[tgt_y][tgt_x] = Cell::Contains(kind);
            let pods: Vec<_> = pods.iter().filter(|p| *p != pod).copied().collect();
            walk_pods(&map, &pods, score, best);
        } else {
            // if can't get out
            if !map[1..y].iter().all(|a| a[x] == Cell::Free) {
                continue;
            }
            let mut map = *map;
            map[y][x] = Cell::Free;
            let mut pods = pods.to_vec();
            for (step, end) in [(-1, 0), (1isize, 10)] {
                let mut tgt_x = x;
                let mut score = score + kind.score() * y;
                loop {
                    tgt_x = ((tgt_x as isize) + step) as usize;
                    score += kind.score();
                    match map[0][tgt_x] {
                        Cell::Blocked => continue,
                        Cell::Contains(_) => break,
                        _ => (),
                    }

                    pods[pod_idx] = (kind, tgt_x, 0);
                    map[0][tgt_x] = Cell::Contains(kind);
                    walk_pods(&map, &pods, score, best);
                    map[0][tgt_x] = Cell::Free;

                    if tgt_x == end {
                        break;
                    }
                }
            }
        }
    }
}

/// get pods which still have to move.
fn get_unset_pods<const N: usize>(map: &Map<N>) -> std::vec::Vec<(Kind, usize, usize)> {
    let mut unset_pods = vec![];
    'x: for x in 0..map[0].len() {
        let mut bottom_unset = false;
        for y in (1..map.len()).rev() {
            if let Cell::Contains(a) = map[y][x] {
                if a.tgt_x() != x || bottom_unset {
                    unset_pods.push((a, x, y));
                    bottom_unset = true;
                }
            } else {
                continue 'x;
            }
        }
    }
    unset_pods.sort_by_key(|(k, _, _)| k.tgt_x());
    unset_pods
}

/// return highest depth the pod can go through a door
fn get_door_depth<const N: usize>(kind: Kind, map: &Map<N>) -> Option<usize> {
    let x = kind.tgt_x();
    for y in (1..map.len()).rev() {
        match map[y][x] {
            Cell::Contains(k) => {
                if k == kind {
                    continue;
                } else {
                    return None;
                }
            }
            _ => return Some(y),
        }
    }
    None
}

#[allow(dead_code)]
fn print_sol<const N: usize>(map: &Map<N>) {
    for l in map {
        for c in l {
            print!(
                "{}",
                match c {
                    Cell::Free => '.',
                    Cell::Blocked => '#',
                    Cell::Contains(Kind::A) => 'A',
                    Cell::Contains(Kind::B) => 'B',
                    Cell::Contains(Kind::C) => 'C',
                    Cell::Contains(Kind::D) => 'D',
                }
            );
        }
        println!();
    }
    println!();
}
