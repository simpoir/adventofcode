use std::collections::HashMap;

#[derive(Default)]
pub struct Day {}

impl crate::cli::Day for Day {
    type Input = String;

    fn gen(&self, data: &str) -> Self::Input {
        data.to_owned()
    }

    fn part1(&self, input: &Self::Input) -> String {
        let mut spec = HashMap::new();
        spec.insert("children", 3);
        spec.insert("cats", 7);
        spec.insert("samoyeds", 2);
        spec.insert("pomeranians", 3);
        spec.insert("akitas", 0);
        spec.insert("vizslas", 0);
        spec.insert("goldfish", 5);
        spec.insert("trees", 3);
        spec.insert("cars", 2);
        spec.insert("perfumes", 1);

        'sue: for (i, l) in input.lines().enumerate() {
            for chunk in l.split_once(": ").unwrap().1.split(", ") {
                let (attr, count) = chunk.split_once(": ").unwrap();
                let count: usize = count.parse().unwrap();
                if spec.get(attr) != Some(&count) {
                    continue 'sue;
                }
            }
            return (i + 1).to_string();
        }
        unimplemented!();
    }

    fn part2(&self, input: &Self::Input) -> String {
        let mut spec = HashMap::new();
        spec.insert("children", 3);
        spec.insert("cats", 7);
        spec.insert("samoyeds", 2);
        spec.insert("pomeranians", 3);
        spec.insert("akitas", 0);
        spec.insert("vizslas", 0);
        spec.insert("goldfish", 5);
        spec.insert("trees", 3);
        spec.insert("cars", 2);
        spec.insert("perfumes", 1);

        'sue: for (i, l) in input.lines().enumerate() {
            for chunk in l.split_once(": ").unwrap().1.split(", ") {
                let (attr, count) = chunk.split_once(": ").unwrap();
                let count: usize = count.parse().unwrap();
                if match attr {
                    "cats" | "trees" => spec.get(attr) >= Some(&count),
                    "pomeranians" | "goldfish" => spec.get(attr) <= Some(&count),
                    _ => spec.get(attr) != Some(&count),
                } {
                    continue 'sue;
                }
            }
            return (i + 1).to_string();
        }
        unimplemented!();
    }
}
