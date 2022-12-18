use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<(i32, i32, i32, i32)>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        data.lines()
            .map(|l| {
                let mut chunks = l.split(&['=', ',', ':']);
                Ok((
                    chunks.nth(1).unwrap().parse()?,
                    chunks.nth(1).unwrap().parse()?,
                    chunks.nth(1).unwrap().parse()?,
                    chunks.nth(1).unwrap().parse()?,
                ))
            })
            .collect()
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        Ok(isnt(input, 2000000).to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let pos = tune(input, 4000000);
        Ok((pos.0 as i64 * 4000000 + pos.1 as i64).to_string())
    }
}

fn isnt(input: &[(i32, i32, i32, i32)], y: i32) -> usize {
    let (min_x, max_x) = input.iter().fold((i32::MAX, i32::MIN), |best, b| {
        let dx = b.2.abs_diff(b.0) as i32;
        (best.0.min(b.2 - dx), best.1.max(b.2 + dx))
    });
    (min_x..=max_x)
        .filter(|x| {
            input
                .iter()
                .filter(|s| (*x, y) != (s.2, s.3))
                .any(|s| x.abs_diff(s.0) + y.abs_diff(s.1) <= s.0.abs_diff(s.2) + s.1.abs_diff(s.3))
        })
        .count()
}

fn tune(input: &[(i32, i32, i32, i32)], n: i32) -> (i32, i32) {
    let input: Vec<(i32, i32, u32)> = input
        .iter()
        .map(|i| (i.0, i.1, (i.0.abs_diff(i.2) + i.1.abs_diff(i.3))))
        .collect();
    // I could partition into cells and divide them, like last year... or I could just bunny-hop
    // around like a brute. a smart brute. hoppy bunnydays!
    let mut x = -1;
    while x < n {
        let mut y = -1;
        x += 1;
        while y < n {
            y += 1;

            let best = input
                .iter()
                .map(|s| s.2 as i32 - (s.0.abs_diff(x) + s.1.abs_diff(y)) as i32)
                .max()
                .unwrap();
            // bunny
            if best < 0 {
                return (x, y);
            }
            // hop
            y += best;
        }
    }
    unimplemented!()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = 26;
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, isnt(&data, 10));
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = (14, 11);
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, tune(&data, 20));
    }
}
