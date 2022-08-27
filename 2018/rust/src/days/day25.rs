use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<[isize; 4]>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        data.lines()
            .map(|l| {
                let mut a = [0; 4];
                for (i, chunk) in l.split(',').enumerate() {
                    a[i] = chunk.parse()?;
                }
                Ok(a)
            })
            .collect()
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let mut cons: Vec<Vec<[isize; 4]>> = vec![];
        for point in input {
            let mut matching = None;
            let mut merged = vec![];
            cons.iter_mut().enumerate().for_each(|(i, con)| {
                if con.iter().any(|other| dist(point, other) <= 3) {
                    if matching.is_none() {
                        con.push(*point);
                        matching = Some(i);
                    } else {
                        merged.push(i);
                    }
                }
            });

            if let Some(matching) = matching {
                for i in merged.drain(..).rev() {
                    let mut popped = cons.remove(i);
                    cons[matching].append(&mut popped);
                }
            } else {
                cons.push(vec![*point]);
            }
        }
        Ok(cons.len().to_string())
    }

    fn part2(&mut self, _input: &Self::Input) -> Result<String> {
        Ok("⭐".to_string())
    }
}

fn dist(a: &[isize; 4], b: &[isize; 4]) -> usize {
    a.iter().zip(b.iter()).map(|(a, b)| a.abs_diff(*b)).sum()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let input = "\
0,0,0,6
0,0,0,0
3,0,0,0
0,3,0,0
0,0,3,0
0,0,0,3
12,0,0,0";
        let expected = "2";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());

        let input = "\
-1,2,2,0
0,0,2,-2
0,0,0,-2
-1,2,0,0
-2,-2,-2,2
3,0,2,-1
-1,3,2,2
-1,0,-1,0
0,2,1,-2
3,0,0,0";
        let expected = "4";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());

        let input = "\
1,-1,-1,-2
-2,-2,0,1
0,2,1,3
-2,3,-2,1
0,2,3,-2
-1,-1,1,-2
0,-2,-1,0
-2,2,3,-1
1,2,2,0
-1,-2,0,-2";
        let expected = "8";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }
}
