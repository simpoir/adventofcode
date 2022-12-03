use std::sync::mpsc::channel;

use crate::cli::Result;

use crate::days::day5::run;

#[derive(Default)]
pub struct Day {
    code: Vec<isize>,
}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<Vec<u8>>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        self.code = data.split(',').map(|c| c.parse().unwrap()).collect();
        let (robot_out, out) = channel();
        run(&self.code, [], robot_out);
        let sub: Vec<u8> = out.into_iter().map(|r| r.try_into().unwrap()).collect();
        Ok(sub
            .strip_suffix(b"\n")
            .unwrap()
            .split(|x| *x == b'\n')
            .map(|l| l.into())
            .collect())
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let res: u32 = (1..input.len() - 2)
            .flat_map(|y| {
                (1..input[0].len() - 2).map(move |x| {
                    if input[y][x] == b'#'
                        && input[y - 1][x] == b'#'
                        && input[y + 1][x] == b'#'
                        && input[y][x - 1] == b'#'
                        && input[y][x + 1] == b'#'
                    {
                        x as u32 * y as u32
                    } else {
                        0
                    }
                })
            })
            .sum();
        Ok(res.to_string())
    }

    fn part2(&mut self, _input: &Self::Input) -> Result<String> {
        let mut code = self.code.clone();
        assert_eq!(1, code[0]);
        code[0] = 2;

        let (robot_out, out) = channel();
        // Manually crafted input. Figure your own.
        run(
            &code,
            b"A,B,A,C,A,B,C,A,B,C
R,12,R,4,R,10,R,12
R,6,L,8,R,10
L,8,R,4,R,4,R,6
n
"
            .iter()
            .map(|c| *c as isize)
            .collect::<Vec<_>>(),
            robot_out,
        );
        let last = out.into_iter().last().unwrap();

        Ok(last.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let input = b"..#..........
..#..........
#######...###
#.#...#...#.#
#############
..#...#...#..
..#####...^..
"
        .split(|x| *x == b'\n')
        .map(|l| l.into())
        .collect();
        let expected = "76";
        assert_eq!(expected, d.part1(&input).unwrap());
    }
}
