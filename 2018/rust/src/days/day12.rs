use std::collections::HashMap;

use crate::cli::Result;

pub struct Day {
    part2: i64,
}

impl Default for Day {
    fn default() -> Self {
        Self {
            part2: 50_000_000_000,
        }
    }
}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = (Vec<bool>, [bool; 32]);

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        let mut lines = data.lines();
        let init = lines
            .next()
            .unwrap()
            .strip_prefix("initial state: ")
            .unwrap()
            .chars()
            .map(|c| c == '#')
            .collect();
        let mut map = [false; 32];
        lines.skip(1).for_each(|l| {
            let idx = l
                .chars()
                .take(5)
                .fold(0, |a, x| a * 2 + (x == '#') as usize);
            map[idx] = l.ends_with('#');
        });
        Ok((init, map))
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let mut grid: Vec<i64> = input
            .0
            .iter()
            .enumerate()
            .filter_map(|(i, &x)| if x { Some(i as i64) } else { None })
            .collect();

        for _ in 0..20 {
            grid = generation2(&grid, &input.1);
        }
        Ok(grid.iter().sum::<i64>().to_string())
    }

    fn part2(&mut self, (init, mapping): &Self::Input) -> Result<String> {
        let mut grid: Vec<i64> = init
            .iter()
            .enumerate()
            .filter_map(|(i, &x)| if x { Some(i as i64) } else { None })
            .collect();
        // not guaranteed (e.g. the sample data doesn't have that).
        // But we assume the input repeats a pattern with some travel. Mine does.
        let mut seen = HashMap::new();
        seen.insert(
            grid.iter().map(|&x| x - grid[0]).collect::<Vec<i64>>(),
            (0, grid[0]),
        );

        for i in 0..self.part2 {
            grid = generation2(&grid, mapping);

            // This is the key; when this is met, every plant has settled and the pattern is
            // just travelling. This would be wrong with input which cycles though
            // multiple steps (mine travels every step)
            if seen
                .insert(
                    grid.iter().map(|&x| x - grid[0]).collect::<Vec<i64>>(),
                    (i, grid[0]),
                )
                .is_some()
            {
                let travel = self.part2 - (i + 1);
                grid = grid.iter().map(|&x| x + travel).collect();
                break;
            }
        }

        Ok(grid.iter().sum::<i64>().to_string())
    }
}

fn generation2(plants: &[i64], mapping: &[bool; 32]) -> Vec<i64> {
    // instead of scanning an actual array, we use
    // a "lightweight" array using 5bit indexes, and pad the gaps.
    let mut result = vec![];
    let mut it = plants.iter().peekable();
    let mut next_expected = **it.peek().unwrap();
    let mut arr = 0;

    while let Some(&&pos) = it.peek() {
        if pos == next_expected {
            arr = arr * 2 + 1;
            arr &= 0b11111;
            it.next();
            next_expected += 1;
        } else {
            arr = (arr * 2) & 0b11111;
            next_expected += 1;

            if arr == 0 {
                arr = 1;
                it.next();
                next_expected = pos + 1;
            }
        }
        if mapping[arr] {
            result.push(next_expected - 3);
        }
    }

    while arr != 0 {
        arr = (arr * 2) & 0b11111;
        next_expected += 1;
        if mapping[arr] {
            result.push(next_expected - 3);
        }
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "\
initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "325";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Day { part2: 50 };
        let expected = "551";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
