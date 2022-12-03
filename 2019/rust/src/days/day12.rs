use crate::cli::Result;

pub struct Day {
    steps: usize,
}

impl Default for Day {
    fn default() -> Self {
        Self { steps: 1000 }
    }
}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<[isize; 3]>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        data.lines()
            .map(|l| {
                let mut chunks = l.split(['=', ',', '>']);
                Ok([
                    chunks.nth(1).unwrap().parse()?,
                    chunks.nth(1).unwrap().parse()?,
                    chunks.nth(1).unwrap().parse()?,
                ])
            })
            .collect()
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let mut pos = input.to_vec();
        let mut vel = [[0isize, 0isize, 0isize]; 4];
        for _ in 0..self.steps {
            step(&mut pos, &mut vel);
        }
        Ok(pos
            .iter()
            .zip(vel)
            .map(|(pos, vel)| {
                pos.iter().copied().map(isize::abs).sum::<isize>()
                    * vel.iter().copied().map(isize::abs).sum::<isize>()
            })
            .sum::<isize>()
            .to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let mut pos = input.to_vec();
        let mut vel = [[0isize, 0isize, 0isize]; 4];
        let mut loops = [0; 3];
        for i in 1.. {
            step(&mut pos, &mut vel);
            crate::util::progress(&i);
            // Axis are independent, therefore the number we are looking for
            // is the smallest common multiple of their individual period.
            (0..3).for_each(|axis| {
                if (0..4).all(|moon| {
                    loops[axis] == 0 && vel[moon][axis] == 0 && pos[moon][axis] == input[moon][axis]
                }) {
                    loops[axis] = i;
                }
            });
            if loops.iter().all(|x| *x > 0) {
                break;
            }
        }
        Ok(loops.into_iter().reduce(ppcm).unwrap().to_string())
    }
}

fn ppcm(a: isize, b: isize) -> isize {
    a * b / pgcd(a.max(b), a.min(b))
}

fn pgcd(a: isize, b: isize) -> isize {
    let r = a % b;
    if r == 0 {
        b
    } else {
        pgcd(b, r)
    }
}

fn step(pos: &mut [[isize; 3]], vel: &mut [[isize; 3]; 4]) {
    crate::util::subsets(&[0, 1, 2, 3], &mut [0, 0], &mut |indexes| {
        let a = indexes[0];
        let b = indexes[1];
        for (i, ax) in pos[a].iter().enumerate() {
            match ax.cmp(&pos[b][i]) {
                std::cmp::Ordering::Less => {
                    vel[a][i] += 1;
                    vel[b][i] -= 1;
                }
                std::cmp::Ordering::Greater => {
                    vel[a][i] -= 1;
                    vel[b][i] += 1;
                }
                _ => (),
            }
        }
        true
    });
    pos.iter_mut()
        .zip(vel)
        .for_each(|(pos, vel)| pos.iter_mut().zip(vel).for_each(|(p, dv)| *p += *dv));
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";

    #[test]
    fn test_part1() {
        let mut d = Day { steps: 10 };
        let expected = "179";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "2772";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
