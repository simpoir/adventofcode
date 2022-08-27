use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = usize;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        Ok(data.parse()?)
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let result = find_best(*input, 3);
        Ok(format!("{},{}", result.0, result.1))
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let result = (1..=300)
            .map(|square| {
                crate::util::progress(&(300 - square));
                (find_best(*input, square), square)
            })
            .max_by_key(|n| n.0 .2)
            .unwrap();
        Ok(format!("{},{},{}", result.0 .0, result.0 .1, result.1))
    }
}

fn find_best(serial: usize, square: usize) -> (usize, usize, isize) {
    let mut grid = vec![[0; 300]; 300];
    grid.iter_mut().enumerate().for_each(|(y, row)| {
        row.iter_mut()
            .enumerate()
            .for_each(move |(x, cell)| *cell = pow(x + 1, y + 1, serial));
    });

    let grid = &grid;
    (0..(300 - square))
        .flat_map(|x| {
            (0..(300 - square)).map(move |y| {
                (
                    x + 1,
                    y + 1,
                    grid[y..(y + square)]
                        .iter()
                        .flat_map(|chunk| chunk[x..(x + square)].iter())
                        .sum::<isize>(),
                )
            })
        })
        .max_by_key(|n| n.2)
        .unwrap_or_default()
}

fn pow(x: usize, y: usize, serial: usize) -> isize {
    let rack = x + 10;
    let level = (rack * y + serial) * rack;
    ((level / 100) % 10) as isize - 5
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_pow() {
        assert_eq!(4, pow(3, 5, 8));
        assert_eq!(-5, pow(122, 79, 57));
        assert_eq!(0, pow(217, 196, 39));
        assert_eq!(4, pow(101, 153, 71));
    }

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let data = d.gen("18").unwrap();
        assert_eq!("33,45", d.part1(&data).unwrap());

        let data = d.gen("42").unwrap();
        assert_eq!("21,61", d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let input = "18";
        let expected = "90,269,16";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
