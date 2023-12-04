use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<Vec<[usize; 3]>>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        Ok(data
            .lines()
            .map(|l| {
                l.split_once(": ")
                    .unwrap()
                    .1
                    .split("; ")
                    .map(|l| {
                        l.split(", ")
                            .map(|chunk| {
                                let (h, t) = chunk.split_once(' ').unwrap();
                                let n = h.parse().unwrap();
                                match t {
                                    "blue" => [0, 0, n],
                                    "green" => [0, n, 0],
                                    "red" => [n, 0, 0],
                                    _ => unimplemented!(),
                                }
                            })
                            .fold([0, 0, 0], |a, b| [a[0] + b[0], a[1] + b[1], a[2] + b[2]])
                    })
                    .collect()
            })
            .collect())
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        Ok(input
            .iter()
            .enumerate()
            .filter(|(_, game)| {
                !game
                    .iter()
                    .any(|group| group[0] > 12 || group[1] > 13 || group[2] > 14)
            })
            .map(|(i, _)| i + 1)
            .sum::<usize>()
            .to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        Ok(input
            .iter()
            .map(|game| {
                game.iter().fold([0usize, 0, 0], |a, b| {
                    [a[0].max(b[0]), a[1].max(b[1]), a[2].max(b[2])]
                })
            })
            .map(|pw| pw.iter().product::<usize>())
            .sum::<usize>()
            .to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "8";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "2286";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
