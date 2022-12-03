use std::collections::HashMap;

use anyhow::Ok;

use crate::cli::Result;

#[derive(Default)]
pub struct Day {
    res1: usize,
}

type Products<'k> = HashMap<&'k str, (usize, Vec<(usize, &'k str)>)>;

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Products<'i>;

    fn need_part1() -> bool {
        true
    }

    fn gen(&mut self, data: &'i str) -> Result<Self::Input> {
        let res = data
            .lines()
            .map(|l| {
                let (src, dst) = l.split_once(" => ").unwrap();
                let (h, t) = dst.split_once(' ').unwrap();
                let ingredients = src
                    .split(", ")
                    .map(|chunk| {
                        let (h, t) = chunk.split_once(' ').unwrap();
                        (h.parse().unwrap(), t)
                    })
                    .collect();
                (t, (h.parse().unwrap(), ingredients))
            })
            .collect();
        Ok(res)
    }

    fn part1(&mut self, recipes: &Self::Input) -> Result<String> {
        let ores = pump(recipes, 1);
        self.res1 = ores;
        Ok(ores.to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let mut prev = 1;
        let mut inc = 1;
        loop {
            if inc == 0 {
                return Ok(prev.to_string());
            }
            let val = pump(input, prev + inc);
            if val > 1_000_000_000_000 {
                inc /= 2;
            } else {
                prev += inc;
                inc *= 2;
            }
        }
    }
}

fn pump(recipes: &Products, wants: usize) -> usize {
    let mut has: HashMap<_, usize> = HashMap::new();
    let mut wants = vec![(wants, "FUEL")];
    let mut ores = 0;
    while let Some((amt, tgt)) = wants.pop() {
        let (produced, ingredients) = recipes.get(tgt).unwrap();
        let currently_has = *has.get(tgt).unwrap_or(&0);
        // maybe we already have that amount
        if amt <= currently_has {
            has.insert(tgt, currently_has - amt);
            continue;
        }
        let needs = amt - currently_has;
        let factor = (needs + produced - 1) / produced; // ceil div
        has.insert(tgt, factor * produced - needs);

        ingredients.iter().for_each(|(amt, name)| {
            let amt = *amt * factor;
            if *name == "ORE" {
                ores += amt
            } else {
                wants.push((amt, name));
            }
        });
    }
    ores
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let input = "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";
        let expected = "31";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    const INPUT2: &str = "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "460664";
        let data = d.gen(INPUT2).unwrap();
        assert_eq!("2210736", d.part1(&data).unwrap());
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
