use crate::cli::Result;

#[derive(Default)]
pub struct Day {
    len: usize,
}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<(isize, isize, isize, isize)>;

    fn need_part1() -> bool {
        true
    }

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        data.lines()
            .map(|l| {
                let mut chunks = l.split(['<', '>', ',']).map(|s| s.trim_start());
                Ok((
                    chunks.nth(1).unwrap().parse()?,
                    chunks.next().unwrap().parse()?,
                    chunks.nth(1).unwrap().parse()?,
                    chunks.next().unwrap().parse()?,
                ))
            })
            .collect()
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let mut best_h = isize::MAX;
        let mut grid: Vec<(isize, isize)> = input.iter().map(|x| (x.0, x.1)).collect();
        for i in 0.. {
            let new_grid: Vec<(isize, isize)> = grid
                .iter()
                .zip(input.iter())
                .map(|(&(x, y), &(_, _, dx, dy))| (x + dx, y + dy))
                .collect();
            let top = new_grid.iter().map(|x| x.1).max().unwrap();
            let bottom = new_grid.iter().map(|x| x.1).min().unwrap();
            let new_best = top - bottom;

            if new_best >= best_h {
                self.len = i;
                break;
            }
            best_h = new_best;
            grid = new_grid;
        }

        let bottom = grid.iter().map(|x| x.1).min().unwrap();
        let right = grid.iter().map(|x| x.0).max().unwrap();
        let left = grid.iter().map(|x| x.0).min().unwrap();
        let best_h: usize = best_h.try_into()?;
        let best_w: usize = (right - left).try_into()?;
        let mut img = vec![vec![false; best_w + 1]; 1 + best_h];
        for (x, y) in grid {
            img[(y - bottom) as usize][(x - left) as usize] = true;
        }
        Ok(img
            .iter()
            .map(|l| {
                l.iter()
                    .map(|&b| if b { "#" } else { " " })
                    .chain(["\n"])
                    .collect::<String>()
            })
            .collect())
    }

    fn part2(&mut self, _input: &Self::Input) -> Result<String> {
        Ok(self.len.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "\
position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "\
#   #  ###
#   #   # 
#   #   # 
#####   # 
#   #   # 
#   #   # 
#   #   # 
#   #  ###
";
        let data = d.gen(INPUT).unwrap();
        println!("{}", d.part1(&data).unwrap());
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "3";
        let data = d.gen(INPUT).unwrap();
        d.part1(&data).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
