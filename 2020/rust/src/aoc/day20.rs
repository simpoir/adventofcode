use std::collections::BTreeMap;

const IMG_DIM: usize = 8;
pub type IMG = [[u8; IMG_DIM]; IMG_DIM];

#[derive(PartialEq)]
pub struct Tile {
    sides: [u16; 4],
    name: u16,
    img: IMG,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[Tile {}: ", self.name)?;
        for s in &self.sides {
            write!(f, "{}({}) ", s, rbit(*s))?;
        }
        writeln!(f)?;
        for l in &self.img {
            writeln!(f, "{}", String::from_utf8_lossy(l))?;
        }
        writeln!(f, "]]")
    }
}

day! {
    day20;
    type INPUT = Vec<Vec<Tile>>;

    fn gen(file: &mut impl BufRead) -> Result<Self::INPUT> {
        let mut lines = file.lines().filter_map(|l| l.ok());
        let mut res = BTreeMap::new();
        loop {
            let name;
            let mut sides = [0u16; 4];
            if let Some(l) = lines.next() {
                name = l[5..9].parse()?;
            } else {
                break
            }

            let top = lines.next().unwrap().into_bytes();
            for c in &top {
                sides[0] <<= 1;
                if *c == b'#' {
                    sides[0] |= 1
                }
            }
            sides[1] = if top[9] == b'#' { 1 } else { 0 };
            sides[3] = if top[0] == b'#' { 1 } else { 0 };

            let mut img: IMG = Default::default();
            for dst_row in img.iter_mut() {
                let l = lines.next().unwrap().into_bytes();
                sides[1] <<= 1;
                sides[3] <<= 1;
                sides[1] |= if l[9] == b'#' { 1 } else { 0 };
                sides[3] |= if l[0] == b'#' { 1 } else { 0 };
                dst_row[..].clone_from_slice(&l[1..IMG_DIM + 1])
            }

            let bottom = lines.next().unwrap().into_bytes();
            for c in &bottom {
                sides[2] <<= 1;
                if *c == b'#' {
                    sides[2] |= 1
                }
            }
            sides[1] <<= 1;
            sides[3] <<= 1;
            sides[1] |= if bottom[9] == b'#' { 1 } else { 0 };
            sides[3] |= if bottom[0] == b'#' { 1 } else { 0 };
            sides[2] = rbit(sides[2]);
            sides[3] = rbit(sides[3]);

            res.insert(name, Tile {name, sides, img});
            lines.next();
        }

        // solve during gen, because part 2 builds on the solve
        let tiles = res;
        let width  = (tiles.len() as f32).sqrt() as usize;

        // Find corners the cheaty way. I say cheat because there is a key
        // assumption here that the sides match exactly 1 to 1, so we don't
        // actually have to try any combination. We can just index sides.
        let mut sides_index: BTreeMap<u16, Vec<Orientation>> = BTreeMap::new();
        for tile in tiles.values() {
            for (i, side) in tile.sides.iter().enumerate() {
                sides_index.entry(*side).or_insert(vec![]).push(Orientation::Straight(i, tile.name));
                let rside = rbit(*side);
                sides_index.entry(rside).or_insert(vec![]).push(Orientation::Flipped(i, tile.name));
            }
        }

        let top_left = tiles.values().find(|tile| {
            let matching = tile.sides.iter().filter(|x| sides_index[x].len() > 1).count();
            matching == 2
        }).expect("obvious corner");

        // orient first corner
        let top_left = rotate(top_left, match (sides_index[&top_left.sides[0]].len() == 1,
            sides_index[&top_left.sides[1]].len() == 1) {
            (true, false) => Rot::R0,
            (true, true) => Rot::R270,
            (false, true) => Rot::R180,
            (false, false) => Rot::R90,
        });
        let mut row = vec![];
        row.push(top_left);
        while row.len() < width {
            let prev_tile: &Tile = row.last().unwrap();
            let prev_tile_id = prev_tile.name;
            let to_match = rbit(prev_tile.sides[1]); // match is mirror.
            let matching_tile = sides_index[&to_match].iter().find(|x| {
                x.unwrap() != prev_tile_id
            }).unwrap();
            let tile = match matching_tile {
                Orientation::Straight(rot, id) => lrot(&tiles[id], *rot),
                Orientation::Flipped(rot, id) => flrot(&tiles[id], *rot),
            };
            row.push(tile);
        }
        let mut solved = vec![row];

        // match top to bottom. Don't care about sides because it's all 1-1.
        for y in 0..width-1 {
            let mut row = vec![];
            for x in  0..width {
                let prev_tile: &Tile = &solved[y][x];
                let prev_tile_id = prev_tile.name;
                let to_match = rbit(prev_tile.sides[2]); // match is mirror
                let matching_tile = sides_index[&to_match].iter().find(|x| {
                    x.unwrap() != prev_tile_id
                }).expect("matching side");
                let tile = match matching_tile {
                    Orientation::Straight(rot, id) => brot(&tiles[id], *rot),
                    Orientation::Flipped(rot, id) => fbrot(&tiles[id], *rot),
                };
                row.push(tile);
            }
            solved.push(row);
        }

        Ok(solved)
    }

    fn part1(solved: &Self::INPUT) -> Result<String> {
        let end = solved.len()-1;
        let res = solved[0][0].name as u64 * solved[0][end].name as u64 * solved[end][0].name as u64 * solved[end][end].name as u64;
        Ok(format!("{}", res))
    }

    fn part2(solved: &Self::INPUT) -> Result<String> {
        let width = solved.len();
        let mut grid: Vec<Vec<u8>> = (0..width).map(|row| {
            (0..IMG_DIM).map(move |line| {
                (0..width).map(|block| &solved[row][block].img[line]).flatten().copied().collect()
            })
        }).flatten().collect();
        let grid_width = grid.len() - 1;


        let monstr: Vec<(usize, usize)> =
            ["                  #",
             "#    ##    ##    ###",
             " #  #  #  #  #  #"]
            .iter()
            .enumerate()
            .map(|(y, l)| {
                l.bytes()
                    .enumerate()
                    .filter_map(move |(x, c)| if c == b'#' { Some((x, y)) } else { None })
            })
            .flatten()
            .collect();
        'monstrmatch: for _ in 0..2 {
            for _ in 0..4 {
                let matches = monsearch(&mut grid, &monstr);
                if matches > 0 {
                    break 'monstrmatch;
                }
                grid = (0..=grid_width).map(|x| (0..=grid_width).map(|y| {
                    grid[grid_width - y][x]
                }).collect()).collect();
            }
            grid = grid.drain(..).rev().collect();
        }

        #[allow(clippy::naive_bytecount)]
        Ok(format!("{}", grid.iter().map(|row| row.iter().filter(|c| **c == b'#').count()).sum::<usize>()))
    }
}

/// search and tag monsters, returning matched count.
fn monsearch(grid: &mut Vec<Vec<u8>>, coords: &[(usize, usize)]) -> usize {
    let mut res = 0;
    for i in 0..(grid.len() - 20) {
        'pos: for j in 0..(grid.len() - 3) {
            for (x, y) in coords {
                if grid[y + j][x + i] == b'.' {
                    continue 'pos;
                }
            }
            // tag found
            for (x, y) in coords {
                grid[y + j][x + i] = b'O';
            }
            res += 1;
        }
    }
    res
}

// wrapped tile to avoid needlessly flipping them
#[derive(Debug)]
enum Orientation {
    Straight(/** rot */ usize, /** tile */ u16),
    Flipped(usize, u16),
}

impl Orientation {
    fn unwrap(&self) -> u16 {
        match self {
            Orientation::Straight(_, id) => *id,
            Orientation::Flipped(_, id) => *id,
        }
    }
}

enum Rot {
    R0,
    R90,
    R180,
    R270,
}

/// 10-bit reverser
fn rbit(mut input: u16) -> u16 {
    let mut res = 0;
    for _ in 0..10 {
        res <<= 1;
        res |= input & 1;
        input >>= 1;
    }
    res
}

/// rotate top to left
fn lrot(input: &Tile, top: usize) -> Tile {
    rotate(
        input,
        match top {
            0 => Rot::R270,
            1 => Rot::R180,
            2 => Rot::R90,
            _ => Rot::R0,
        },
    )
}
fn flrot(input: &Tile, top: usize) -> Tile {
    rotate(
        &flip_v(input),
        match top {
            0 => Rot::R90,
            1 => Rot::R180,
            2 => Rot::R270,
            _ => Rot::R0,
        },
    )
}

/// rotate top
fn brot(input: &Tile, top: usize) -> Tile {
    rotate(
        input,
        match top {
            0 => Rot::R0,
            1 => Rot::R270,
            2 => Rot::R180,
            _ => Rot::R90,
        },
    )
}

/// rotate flipped top
fn fbrot(input: &Tile, top: usize) -> Tile {
    rotate(
        &flip_v(input),
        match top {
            0 => Rot::R180,
            1 => Rot::R270,
            2 => Rot::R0,
            _ => Rot::R90,
        },
    )
}

fn rotate(input: &Tile, rot: Rot) -> Tile {
    let mut output: IMG = Default::default();
    let mut sides = [0u16; 4];
    match rot {
        Rot::R0 => {
            for (j, out_row) in output.iter_mut().enumerate() {
                out_row[..].clone_from_slice(&input.img[j][..]);
            }
            sides[..].clone_from_slice(&input.sides[..]);
        }
        Rot::R90 => {
            for (j, out_row) in output.iter_mut().enumerate() {
                for (i, in_col) in input.img.iter().enumerate() {
                    out_row[IMG_DIM - 1 - i] = in_col[j];
                }
            }
            sides[0] = input.sides[3];
            sides[1] = input.sides[0];
            sides[2] = input.sides[1];
            sides[3] = input.sides[2];
        }
        Rot::R180 => {
            for (out_row, in_row) in output.iter_mut().zip(input.img.iter().rev()) {
                for (i, item) in in_row.iter().rev().enumerate() {
                    out_row[i] = *item;
                }
            }
            sides[0] = input.sides[2];
            sides[1] = input.sides[3];
            sides[2] = input.sides[0];
            sides[3] = input.sides[1];
        }
        Rot::R270 => {
            for (j, out_row) in output.iter_mut().enumerate() {
                for (i, in_col) in input.img.iter().enumerate() {
                    out_row[i] = in_col[IMG_DIM - 1 - j];
                }
            }
            for i in 0..4 {
                sides[(i + 3) % 4] = input.sides[i];
            }
            sides[0] = input.sides[1];
            sides[1] = input.sides[2];
            sides[2] = input.sides[3];
            sides[3] = input.sides[0];
        }
    }
    Tile {
        sides,
        name: input.name,
        img: output,
    }
}

fn flip_v(input: &Tile) -> Tile {
    let mut output = [[0u8; IMG_DIM]; IMG_DIM];
    for (j, item) in output.iter_mut().enumerate() {
        *item = input.img[IMG_DIM - 1 - j];
    }
    let mut sides = [0u16; 4];
    sides[2] = rbit(input.sides[0]);
    sides[1] = rbit(input.sides[1]);
    sides[0] = rbit(input.sides[2]);
    sides[3] = rbit(input.sides[3]);
    Tile {
        name: input.name,
        img: output,
        sides,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rot() {
        let mut img: IMG = Default::default();
        img[0][0] = b'x';
        img[IMG_DIM - 1][0] = b'y';
        let mut img2: IMG = Default::default();
        img2[0][IMG_DIM - 1] = b'x';
        img2[0][0] = b'y';
        let name = 42;
        assert_eq!(
            Tile {
                name,
                img: img2,
                sides: [4, 1, 2, 3]
            },
            rotate(
                &Tile {
                    name,
                    img,
                    sides: [1, 2, 3, 4]
                },
                Rot::R90
            )
        );
    }
}
