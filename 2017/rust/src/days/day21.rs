use crate::cli::Result;

pub struct Day {
    iters: usize,
}

impl Default for Day {
    fn default() -> Self {
        Self { iters: 5 }
    }
}

pub type R2 = (Vec<Vec<bool>>, Vec<Vec<bool>>);
pub type R3 = (Vec<Vec<bool>>, Vec<Vec<bool>>);

impl<'i> crate::cli::Day<'i> for Day {
    type Input = (Vec<R2>, Vec<R3>);

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        let mut r2 = vec![];
        let mut r3 = vec![];
        data.lines().for_each(|l| {
            let (head, tail) = l.split_once(" => ").unwrap();
            let k = head
                .split('/')
                .map(|r| r.chars().map(|c| c == '#').collect())
                .collect();
            let v = tail
                .split('/')
                .map(|r| r.chars().map(|c| c == '#').collect())
                .collect();
            if head.len() == 5 {
                r2.push((k, v));
            } else {
                r3.push((k, v));
            }
        });
        Ok((r2, r3))
    }

    fn part1(&mut self, rules: &Self::Input) -> Result<String> {
        let mut grid = vec![
            vec![false, true, false],
            vec![false, false, true],
            vec![true, true, true],
        ];
        for _ in 0..self.iters {
            grid = enhance(&grid, rules);
        }
        Ok(grid.iter().flatten().filter(|x| **x).count().to_string())
    }

    fn part2(&mut self, rules: &Self::Input) -> Result<String> {
        let mut grid = vec![
            vec![false, true, false],
            vec![false, false, true],
            vec![true, true, true],
        ];
        for i in (0..18).rev() {
            crate::util::progress(&i);
            grid = enhance(&grid, rules);
        }
        Ok(grid.iter().flatten().filter(|x| **x).count().to_string())
    }
}

const TRANSFORMS: [[[isize; 2]; 3]; 8] = [
    [[1, 0], [0, 1], [0, 0]],   // ident
    [[-1, 0], [0, 1], [1, 0]],  // horiz
    [[1, 0], [0, -1], [0, 1]],  // vert
    [[0, -1], [1, 0], [1, 0]],  // R90
    [[-1, 0], [0, -1], [1, 1]], // R180
    [[0, 1], [-1, 0], [0, 1]],  // R270
    [[0, 1], [1, 0], [0, 0]],   // transpose
    [[0, -1], [-1, 0], [1, 1]], // rtranspose
];

fn enhance(grid: &[Vec<bool>], rules: &(Vec<R2>, Vec<R3>)) -> Vec<Vec<bool>> {
    let len = grid[0].len();
    let (slice, mut res, rules) = if len % 2 == 0 {
        let siz = len / 2 * 3;
        (2, vec![vec![false; siz as usize]; siz as usize], &rules.0)
    } else {
        let siz = len / 3 * 4;
        (3, vec![vec![false; siz as usize]; siz as usize], &rules.1)
    };
    for (j, cj) in (0..len).step_by(slice).enumerate() {
        'chunks: for (i, ci) in (0..len).step_by(slice).enumerate() {
            for (rule_in, rule_out) in rules {
                'trans: for transform in TRANSFORMS {
                    for di in 0..slice {
                        for (dj, rule_iny) in rule_in.iter().enumerate().take(slice) {
                            let y = (di as isize * transform[1][0]
                                + dj as isize * transform[1][1]
                                + transform[2][1] * (slice - 1) as isize)
                                as usize;
                            let x = (di as isize * transform[0][0]
                                + dj as isize * transform[0][1]
                                + transform[2][0] * (slice - 1) as isize)
                                as usize;
                            if grid[cj + y][ci + x] != rule_iny[di] {
                                continue 'trans;
                            }
                        }
                    }
                    expand_rule(&mut res, i, j, rule_out);
                    continue 'chunks;
                }
            }
            panic!("no rule mached chunk");
        }
    }
    res
}

fn expand_rule(res: &mut [Vec<bool>], i: usize, j: usize, rule_out: &[Vec<bool>]) {
    let n = rule_out[0].len();
    for (inj, outj) in ((j * n)..(j * n + n)).enumerate() {
        for (ini, outi) in ((i * n)..(i * n + n)).enumerate() {
            res[outj][outi] = rule_out[inj][ini];
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let mut d: Day = Day { iters: 2 };
        let input = "\
../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#";
        let expected = "12";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }
}
