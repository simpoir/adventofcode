use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

const KEY: i64 = 811589153;

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<i64>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        data.lines().map(|l| Ok(l.parse()?)).collect()
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let mut output: Vec<usize> = (0..input.len()).collect();
        mix(&mut output, input);

        let zero_pos = input.iter().position(|x| *x == 0).unwrap();
        let zero_pos = output.iter().position(|x| *x == zero_pos).unwrap();
        let res: i64 = [1000, 2000, 3000]
            .into_iter()
            .map(|off| input[output[(zero_pos + off).rem_euclid(input.len())]])
            .sum();
        Ok(res.to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let input: Vec<i64> = input.iter().map(|&v| (v * KEY)).collect();
        let mut output: Vec<usize> = (0..input.len()).collect();
        for _ in 0..10 {
            mix(&mut output, &input);
        }

        let zero_pos = input.iter().position(|x| *x == 0).unwrap();
        let zero_pos = output.iter().position(|x| *x == zero_pos).unwrap();
        let res: i64 = [1000, 2000, 3000]
            .into_iter()
            .map(|off| input[output[(zero_pos + off).rem_euclid(input.len())]])
            .sum();
        Ok(res.to_string())
    }
}

fn mix(output: &mut Vec<usize>, input: &[i64]) {
    let l = input.len() as i64;
    input.iter().enumerate().for_each(|(i, v)| {
        let pos = output.iter().position(|x| *x == i).unwrap();
        let dst = (pos as i64 + *v).rem_euclid(l - 1);
        output.remove(pos);
        output.insert(dst as usize, i);
    });
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "1
2
-3
3
-2
0
4
";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "3";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "1623178306";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
