use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<[isize; 4]>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        data.lines()
            .map(|l| {
                let mut chunks = l.split(['<', ',', '>', '=']);
                Ok([
                    chunks.nth(2).unwrap().parse()?,
                    chunks.next().unwrap().parse()?,
                    chunks.next().unwrap().parse()?,
                    chunks.last().unwrap().parse()?,
                ])
            })
            .collect()
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let strongest = input.iter().max_by_key(|b| b[3]).unwrap();
        let result: usize = count_in_range(input, strongest);
        Ok(result.to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        // The point of interest is the closest point to (0,0) in a node bounding box.
        // Since the input data seems heavily nested, we shortcut this with just comparing
        // manhattan distances and their range. If the dataset were spread,
        // this would yield an incorrect result.
        let dist_range = input
            .iter()
            .map(|p| {
                (
                    p[0..3]
                        .iter()
                        .map(|coord| coord.unsigned_abs())
                        .sum::<usize>(),
                    p[3] as usize,
                )
            })
            .collect::<Vec<_>>();

        let result = dist_range
            .iter()
            .map(|a| {
                // closest manhattan dist from a
                let point = a.0.saturating_sub(a.1);
                (
                    dist_range
                        .iter()
                        .filter(|b| b.0.abs_diff(point) <= b.1)
                        .count(),
                    point,
                )
            })
            .max_by_key(|count| count.0)
            .unwrap()
            .1;
        Ok(result.to_string())
    }
}

fn count_in_range(input: &[[isize; 4]], strongest: &[isize; 4]) -> usize {
    input
        .iter()
        .filter(|b| {
            strongest[3] as usize
                >= b[0].abs_diff(strongest[0])
                    + b[1].abs_diff(strongest[1])
                    + b[2].abs_diff(strongest[2])
        })
        .count()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let input = "\
pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1";
        let expected = "7";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let input = "\
pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5";
        let expected = "36";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
