use std::collections::HashMap;

use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<Vec<u8>>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        Ok(data.lines().map(|l| l.bytes().collect()).collect())
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let mut res = 0;
        let mut n = 0;
        let mut valid = false;
        for (y, l) in input.iter().enumerate() {
            for (x, c) in l.iter().enumerate() {
                if *c >= b'0' && *c <= b'9' {
                    n = n * 10 + (c - b'0') as u32;

                    if valid {
                        continue;
                    }
                    for [dx, dy] in crate::util::AROUND {
                        let x = ((x as isize) + dx) as usize;
                        let y = ((y as isize) + dy) as usize;
                        if input
                            .get(y)
                            .and_then(|row| row.get(x))
                            .map(|c| *c != b'.' && (*c < b'0' || *c > b'9'))
                            .unwrap_or(false)
                        {
                            valid = true;
                            break;
                        }
                    }
                } else if valid {
                    res += n;
                    n = 0;
                    valid = false;
                } else {
                    n = 0;
                }
            }
        }
        Ok(res.to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let mut gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

        let mut n = 0;
        let mut valid = None;
        for (y, l) in input.iter().enumerate() {
            for (x, c) in l.iter().enumerate() {
                if *c >= b'0' && *c <= b'9' {
                    n = n * 10 + (c - b'0') as u32;

                    if valid.is_some() {
                        continue;
                    }
                    for [dx, dy] in crate::util::AROUND {
                        let x = ((x as isize) + dx) as usize;
                        let y = ((y as isize) + dy) as usize;
                        if input
                            .get(y)
                            .and_then(|row| row.get(x))
                            .map(|c| *c == b'*')
                            .unwrap_or(false)
                        {
                            valid = Some((x, y));
                            break;
                        }
                    }
                } else if let Some(gear) = valid {
                    gears.entry(gear).or_default().push(n);
                    n = 0;
                    valid = None;
                } else {
                    n = 0;
                }
            }
        }

        let res: u32 = gears
            .values()
            .filter_map(|v| if v.len() > 1 { Some(v[0] * v[1]) } else { None })
            .sum();
        Ok(res.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "4361";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "467835";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
