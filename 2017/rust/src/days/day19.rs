use crate::cli::Result;

#[derive(Default)]
pub struct Day {
    steps: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Cell {
    Blank,
    Vert,
    Horiz,
    Corner,
    Letter(char),
}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = ((isize, isize), Vec<Vec<Cell>>);

    fn need_part1() -> bool {
        true
    }

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        Ok((
            (
                data.lines()
                    .next()
                    .unwrap()
                    .chars()
                    .position(|c| c == '|')
                    .unwrap() as isize,
                0,
            ),
            data.lines()
                .map(|l| {
                    l.chars()
                        .map(|c| match c {
                            ' ' => Cell::Blank,
                            '|' => Cell::Vert,
                            '-' => Cell::Horiz,
                            '+' => Cell::Corner,
                            x => Cell::Letter(x),
                        })
                        .collect()
                })
                .collect(),
        ))
    }

    fn part1(&mut self, (start, grid): &Self::Input) -> Result<String> {
        let mut crossed = String::new();
        let (mut x, mut y) = start;
        let (mut dx, mut dy) = (0isize, 1isize);
        loop {
            match grid[y as usize][x as usize] {
                Cell::Blank => break,
                Cell::Vert | Cell::Horiz => (),
                Cell::Corner => {
                    if grid[(y + dx) as usize][(x + dy) as usize] != Cell::Blank {
                        (dx, dy) = (dy, dx)
                    } else {
                        (dx, dy) = (-dy, -dx)
                    }
                }
                Cell::Letter(c) => crossed.push(c),
            }
            self.steps += 1;
            (x, y) = (x + dx, y + dy);
        }

        Ok(crossed)
    }

    fn part2(&mut self, _input: &Self::Input) -> Result<String> {
        Ok(self.steps.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "     |          
     |  +--+    
     A  |  C    
 F---|--|-E---+ 
     |  |  |  D 
     +B-+  +--+ 
                ";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "ABCDEF";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "38";
        let data = d.gen(INPUT).unwrap();
        d.part1(&data).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
