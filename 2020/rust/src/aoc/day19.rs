#[derive(Clone)]
pub enum Chunk {
    Chain(usize, usize),
    Chain3(usize, usize, usize),
    Ref(usize),
    Alt(Box<Chunk>, Box<Chunk>),
    Lit(u8),
}

fn compile(data: &str) -> Chunk {
    let sep = data.find(" | ");
    if let Some(p) = sep {
        let (head, tail) = data.split_at(p);
        let (_, tail) = tail.split_at(3);
        return Chunk::Alt(Box::new(compile(head)), Box::new(compile(tail)));
    }
    if data.starts_with('"') {
        return Chunk::Lit(data.bytes().nth(1).unwrap());
    }
    let chain: Vec<&str> = data.split(' ').collect();
    match chain.len() {
        1 => Chunk::Ref(chain[0].parse().unwrap()),
        2 => Chunk::Chain(chain[0].parse().unwrap(), chain[1].parse().unwrap()),
        3 => Chunk::Chain3(
            chain[0].parse().unwrap(),
            chain[1].parse().unwrap(),
            chain[2].parse().unwrap(),
        ),
        _ => unimplemented!(),
    }
}

day! {
    day19;
    type INPUT = (Vec<Chunk>, Vec<String>);

    fn gen(file: &mut impl BufRead) -> Result<Self::INPUT> {
        let mut rule_lines = vec![];
        let mut it = file.lines();
        while let Some(l) = it.next() {
            let l = l?;
            if l.is_empty() {
                break;
            }
            rule_lines.push(l);
        }
        let data_lines = it.filter_map(|x| x.ok()).collect();

        let mut rules = vec![Chunk::Lit(0); usize::max(43, rule_lines.len())];
        for l in rule_lines {
            let h = l.find(':').unwrap();
            let num: usize = l[..h].parse()?;
            rules[num] = compile(&l[h+2..])
        }
        Ok((rules, data_lines))
    }

    fn part1((rules, data): &Self::INPUT) -> Result<std::string::String> {
        let count = data.iter().filter(|d| {
            rules_match(&rules, d.as_bytes())
        }).count();
        Ok(format!("{}", count))
    }

    fn part2((rules, data): &Self::INPUT) -> Result<std::string::String> {
        let mut new_rules = rules.clone();
        // smart. the rules have an endless tail, but thats's the second alt,
        // so we are likely to match or fail before stack overflowing.
        new_rules[8] = compile("42 | 42 8");
        new_rules[11] = compile("42 31 | 42 11 31");
        let count = data.iter().filter(|d| {
            rules_match(&new_rules, d.as_bytes())
        }).count();
        Ok(format!("{}", count))
    }
}

fn rules_match(rules: &[Chunk], expr: &[u8]) -> bool {
    let mut pos;
    let mut rule = &rules[0];
    // DFS rollback stack of (pos, rule). start at 0
    let mut branch_stack = vec![(0, rule, vec![])];
    // current match sequence
    let mut rule_seq;

    'branch: loop {
        if let Some(r) = branch_stack.pop() {
            pos = r.0;
            rule_seq = r.2;
            rule_seq.push(r.1);
        } else {
            return false;
        }
        'seq: loop {
            // can't iterate and mutate, so we loop
            if let Some(r) = rule_seq.pop() {
                rule = r;
            } else {
                // At end exactly and nothing to match. This tastes of success!
                return pos == expr.len();
            }
            match rule {
                Chunk::Lit(l) => {
                    if let Some(c) = expr.get(pos) {
                        if *c != *l {
                            continue 'branch;
                        }
                    } else {
                        continue 'branch;
                    }
                }
                Chunk::Ref(r) => {
                    rule_seq.push(&rules[*r]);
                    continue 'seq;
                }
                Chunk::Alt(a, b) => {
                    branch_stack.push((pos, b, rule_seq.clone()));
                    branch_stack.push((pos, a, rule_seq));
                    continue 'branch;
                }
                Chunk::Chain(r1, r2) => {
                    rule_seq.push(&rules[*r2]);
                    rule_seq.push(&rules[*r1]);
                    continue 'seq;
                }
                Chunk::Chain3(r1, r2, r3) => {
                    rule_seq.push(&rules[*r3]);
                    rule_seq.push(&rules[*r2]);
                    rule_seq.push(&rules[*r1]);
                    continue 'seq;
                }
            };
            pos += 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_simple() {
        let rules = vec![compile("\"a\"")];
        assert!(rules_match(&rules, b"a"), "good lit");
        assert!(!rules_match(&rules, b"b"), "bad lit");
    }

    #[test]
    fn test_alt() {
        let rules = vec![compile("1 | 2"), compile("\"a\""), compile("\"b\"")];
        assert!(rules_match(&rules, b"a"), "good lit");
        assert!(rules_match(&rules, b"b"));
        assert!(!rules_match(&rules, b"c"), "bad lit");
        assert!(!rules_match(&rules, b"ac"), "trail");
    }

    #[test]
    fn test_chain() {
        let rules = vec![compile("1 2"), compile("\"a\""), compile("\"b\"")];
        assert!(rules_match(&rules, b"ab"), "good lit");
        assert!(!rules_match(&rules, b"aa"), "bad lit");
        assert!(!rules_match(&rules, b"bb"), "bad lit");
        assert!(!rules_match(&rules, b"abb"), "bad trail");
    }

    #[test]
    fn test_rollback() {
        let rules = vec![
            compile("1 2 3"),
            compile("2 | 2 2"),
            compile("\"a\""),
            compile("\"b\""),
        ];
        assert!(rules_match(&rules, b"aab"), "aab");
        assert!(rules_match(&rules, b"aaab"), "aaab");
    }
}
