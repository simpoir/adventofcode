use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = &'i str;

    fn gen(&mut self, data: &'i str) -> Result<Self::Input> {
        Ok(data)
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let input: usize = input.parse()?;
        let mut recipes = vec![3, 7];
        let (mut a, mut b) = (0, 1);
        while recipes.len() <= input + 10 {
            (a, b) = step(&mut recipes, a, b);
        }
        Ok(recipes
            .iter()
            .skip(input)
            .take(10)
            .map(|c| c.to_string())
            .collect())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let pattern: Vec<u8> = input.bytes().map(|c| c - b'0').collect();
        let mut recipes = vec![3, 7];
        let (mut a, mut b) = (0, 1);
        while !recipes.ends_with(&pattern) && !recipes[..(recipes.len() - 1)].ends_with(&pattern) {
            (a, b) = step(&mut recipes, a, b);
        }
        Ok(
            (recipes.len() - pattern.len() - if recipes.ends_with(&pattern) { 0 } else { 1 })
                .to_string(),
        )
    }
}

fn step(recipes: &mut Vec<u8>, a: usize, b: usize) -> (usize, usize) {
    let val_a = recipes[a];
    let val_b = recipes[b];
    let res = val_a + val_b;
    let (first, second) = (res / 10, res % 10);
    if first != 0 {
        recipes.push(first);
    }
    recipes.push(second);
    (
        (a + val_a as usize + 1) % recipes.len(),
        (b + val_b as usize + 1) % recipes.len(),
    )
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let input = "9";
        let expected = "5158916779";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        assert_eq!("9", d.part2(&"51589").unwrap());
        assert_eq!("5", d.part2(&"01245").unwrap());
        assert_eq!("18", d.part2(&"92510").unwrap());
        assert_eq!("2018", d.part2(&"59414").unwrap());
    }
}
