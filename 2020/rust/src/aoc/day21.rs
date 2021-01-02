use std::collections::{BTreeMap, HashMap, HashSet};

day! {
    day21;
    type INPUT = Vec<(HashSet<String>, Vec<String>)>;

    fn gen(file: &mut impl BufRead) -> Result<Self::INPUT> {
        Ok(file.lines().map(|line| {
            let line = line.unwrap();
            let mut all = line.split(" (contains ");
            (all.next().unwrap().split(' ').map(String::from).collect(),
             all.next().unwrap().trim_end_matches(')').split(", ").map(String::from).collect())
        }).collect())
    }

    fn part1(input: &Self::INPUT) -> Result<String> {
        let possible_matches: HashMap<&String, HashSet<&String>> = matchify(input);
        let allergenic: Vec<&String> = possible_matches.values().flatten().copied().collect();
        let res: usize = input.iter().map(|(ingredients, _)| ingredients.iter().filter(|i| !allergenic.contains(i)).count()).sum();
        Ok(format!("{}", res))
    }

    fn part2(input: &Self::INPUT) -> Result<String> {
        let mut possible_matches: HashMap<&String, HashSet<&String>> = matchify(input);
        let mut identified: BTreeMap<&String, &String> = BTreeMap::new();  // btree is sorted :)
        while !&possible_matches.is_empty() {
            possible_matches.retain(|k, v| {
                if v.len() > 1 {
                    for vv in identified.values() {
                        v.remove(vv);
                    }
                    return true;
                }
                // shamelessly clone refs, like a barbarian.
                identified.insert(k, v.iter().next().unwrap().clone());
                false
            });
        }
        let mut vals = identified.values();
        let mut res: String = (*vals.next().unwrap()).clone();
        for s in vals {
            res.push(',');
            res.push_str(&s);
        }
        Ok(format!("{}", res))
    }
}

fn matchify(input: &Vec<(HashSet<String>, Vec<String>)>) -> HashMap<&String, HashSet<&String>> {
    let mut possible_matches: HashMap<&String, HashSet<&String>> = HashMap::new();
    for (ingredients, allergens) in input {
        for allergen in allergens {
            possible_matches
                .entry(allergen)
                .and_modify(|matches| {
                    matches.retain(|i| ingredients.contains(*i));
                })
                .or_insert_with(|| ingredients.iter().collect());
        }
    }
    possible_matches
}
