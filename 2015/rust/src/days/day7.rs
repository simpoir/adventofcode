use std::collections::HashMap;

#[derive(Default)]
pub struct Day {}

impl crate::cli::Day for Day {
    type Input = HashMap<[u8; 2], String>;

    fn gen(&self, data: &str) -> Self::Input {
        let mut wires = HashMap::new();
        data.lines().for_each(|l| {
            let (lhs, rhs) = l.split_once(" -> ").unwrap();
            wires.insert(arrayd(rhs), lhs.to_string());
        });
        wires
    }

    fn part1(&self, wires: &Self::Input) -> String {
        walk(*b"a\0", wires, &mut HashMap::new()).to_string()
    }

    fn part2(&self, wires: &Self::Input) -> String {
        let mut cache = HashMap::new();
        cache.insert(*b"b\0", walk(*b"a\0", wires, &mut HashMap::new()));
        walk(*b"a\0", wires, &mut cache).to_string()
    }
}

fn walk(arg: [u8; 2], wires: &HashMap<[u8; 2], String>, cache: &mut HashMap<[u8; 2], u16>) -> u16 {
    if let Some(v) = cache.get(&arg) {
        return *v;
    }

    let lhs: Vec<_> = wires.get(&arg).unwrap().split(' ').collect();
    let v = match lhs[..] {
        [a] => get_operand(a, wires, cache),
        ["NOT", a] => !get_operand(a, wires, cache),
        [a, "AND", b] => get_operand(a, wires, cache) & get_operand(b, wires, cache),
        [a, "OR", b] => get_operand(a, wires, cache) | get_operand(b, wires, cache),
        [a, "LSHIFT", b] => get_operand(a, wires, cache) << get_operand(b, wires, cache),
        [a, "RSHIFT", b] => get_operand(a, wires, cache) >> get_operand(b, wires, cache),
        _ => unreachable!(),
    };

    cache.insert(arg, v);
    v
}

fn arrayd(rhs: &str) -> [u8; 2] {
    if rhs.len() == 1 {
        [rhs.as_bytes()[0], 0]
    } else {
        let b = rhs.as_bytes();
        [b[0], b[1]]
    }
}

fn get_operand(
    name: &str,
    wires: &HashMap<[u8; 2], String>,
    cache: &mut HashMap<[u8; 2], u16>,
) -> u16 {
    name.parse()
        .unwrap_or_else(|_| walk(arrayd(name), wires, cache))
}
