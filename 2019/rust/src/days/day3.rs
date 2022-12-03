use crate::cli::Result;

#[derive(Default)]
pub struct Day {
    part2: usize,
}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<Vec<(char, isize)>>;

    fn need_part1() -> bool {
        true
    }

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        data.lines()
            .map(|l| {
                l.split(',')
                    .map(|chunk| {
                        Ok((
                            chunk.chars().next().unwrap(),
                            chunk.get(1..).unwrap().parse()?,
                        ))
                    })
                    .collect()
            })
            .collect()
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let a = expand(&input[0]);
        let b = expand(&input[1]);
        fn sort_points(a: &&(isize, isize), b: &&(isize, isize)) -> std::cmp::Ordering {
            a.0.cmp(&b.0).then(a.1.cmp(&b.1))
        }

        let mut intersect_steps = vec![];

        let mut intersects = vec![];
        let mut a_step_count = 0;
        a.windows(2).for_each(|aa| {
            let a1 = aa.iter().min_by(sort_points).unwrap();
            let a2 = aa.iter().max_by(sort_points).unwrap();

            let mut step_count = a_step_count;
            b.windows(2).for_each(|bb| {
                let b1 = bb.iter().min_by(sort_points).unwrap();
                let b2 = bb.iter().max_by(sort_points).unwrap();

                if (a1.0..=a2.0).contains(&b2.0)
                    && (b1.1..=b2.1).contains(&a2.1)
                    && (a2.1 != 0 || b2.0 != 0/* filter origin */)
                {
                    intersects.push((b1.0, a1.1));
                    intersect_steps
                        .push(step_count + aa[0].0.abs_diff(b1.0) + bb[0].1.abs_diff(a1.1));
                } else if (a1.1..=a2.1).contains(&b2.1)
                    && (b1.0..=b2.0).contains(&a2.0)
                    && (b2.1 != 0 || a2.0 != 0/* filter origin */)
                {
                    intersects.push((a1.0, b1.1));
                    intersect_steps
                        .push(step_count + aa[0].1.abs_diff(b1.1) + bb[0].0.abs_diff(a1.0));
                };

                step_count += (b1.0..b2.0).count() + (b1.1..b2.1).count();
            });
            a_step_count += (a1.0..a2.0).count() + (a1.1..a2.1).count();
        });

        self.part2 = *intersect_steps.iter().min().unwrap();

        Ok(intersects
            .iter()
            .map(|p| (p.0 + p.1).abs())
            .min()
            .unwrap()
            .to_string())
    }

    fn part2(&mut self, _input: &Self::Input) -> Result<String> {
        Ok(self.part2.to_string())
    }
}

fn expand(moves: &[(char, isize)]) -> Vec<(isize, isize)> {
    let mut result = vec![(0, 0)];
    for (dir, steps) in moves {
        let prev = result.last().unwrap();
        result.push(match dir {
            'U' => (prev.0, prev.1 + steps),
            'D' => (prev.0, prev.1 - steps),
            'L' => (prev.0 - steps, prev.1),
            'R' => (prev.0 + steps, prev.1),
            _ => unimplemented!(),
        });
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "R8,U5,L5,D3
U7,R6,D4,L4";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "6";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "30";
        let data = d.gen(INPUT).unwrap();
        d.part1(&data).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());

        let data = d
            .gen(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72
U62,R66,U55,R34,D71,R55,D58,R83",
            )
            .unwrap();
        d.part1(&data).unwrap();
        assert_eq!(610, d.part2);

        let data = d
            .gen(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51
U98,R91,D20,R16,D67,R40,U7,R15,U6,R7",
            )
            .unwrap();
        d.part1(&data).unwrap();
        assert_eq!(410, d.part2);
    }
}
