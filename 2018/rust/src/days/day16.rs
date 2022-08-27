use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = (Vec<([usize; 4], [usize; 4], [usize; 4])>, Vec<[usize; 4]>);

    fn gen(&mut self, data: &'i str) -> Result<Self::Input> {
        let (part1, part2) = data.split_once("\n\n\n\n").unwrap();
        let mut res1 = vec![];
        let mut chunks = part1.lines();
        loop {
            let mut a = [0; 4];
            if let Some(l) = chunks.next() {
                l[9..19]
                    .split(", ")
                    .take(4)
                    .enumerate()
                    .for_each(|(i, c)| a[i] = c.parse().unwrap());
            } else {
                break;
            }
            let mut b = [0; 4];
            chunks
                .next()
                .unwrap()
                .split_whitespace()
                .enumerate()
                .for_each(|(i, c)| b[i] = c.parse().unwrap());
            let mut c = [0; 4];
            chunks.next().unwrap()[9..19]
                .split(", ")
                .take(4)
                .enumerate()
                .for_each(|(i, x)| c[i] = x.parse().unwrap());
            chunks.next();
            res1.push((a, b, c));
        }
        Ok((
            res1,
            part2
                .lines()
                .map(|l| {
                    let mut chunks = l.split_ascii_whitespace();
                    Ok([
                        chunks.next().unwrap().parse::<usize>()?,
                        chunks.next().unwrap().parse::<usize>()?,
                        chunks.next().unwrap().parse::<usize>()?,
                        chunks.next().unwrap().parse::<usize>()?,
                    ])
                })
                .collect::<Result<Vec<[usize; 4]>>>()?,
        ))
    }

    fn part1(&mut self, (part1, _): &Self::Input) -> Result<String> {
        Ok(part1
            .iter()
            .filter(|(input, op, output)| try_ops(input, op, output).len() >= 3)
            .count()
            .to_string())
    }

    fn part2(&mut self, (part1, part2): &Self::Input) -> Result<String> {
        let mut map = [""; 16];
        while map.iter().any(|x| x == &"") {
            for (input, op, output) in part1 {
                let mut candidates = try_ops(input, op, output);
                candidates = candidates
                    .iter()
                    .filter(|x| !map.contains(x))
                    .copied()
                    .collect();
                if candidates.len() == 1 {
                    map[op[0]] = candidates[0];
                }
            }
        }
        let mut registers = [0; 4];
        for op in part2 {
            registers[op[3]] = do_op(map[op[0]], op, &registers);
        }
        Ok(registers[0].to_string())
    }
}

fn do_op(op: &'static str, opcode: &[usize; 4], input: &[usize; 4]) -> usize {
    let r1 = input[opcode[1]];
    let v1 = opcode[1];
    let r2 = input[opcode[2]];
    let v2 = opcode[2];
    match op {
        "addr" => r1 + r2,
        "addi" => r1 + v2,
        "mulr" => r1 * r2,
        "muli" => r1 * v2,
        "banr" => r1 & r2,
        "bani" => r1 & v2,
        "borr" => r1 | r2,
        "bori" => r1 | v2,
        "setr" => r1,
        "seti" => v1,
        "gtir" => (v1 > r2) as usize,
        "gtri" => (r1 > v2) as usize,
        "gtrr" => (r1 > r2) as usize,
        "eqir" => (v1 == r2) as usize,
        "eqri" => (r1 == v2) as usize,
        "eqrr" => (r1 == r2) as usize,
        _ => unimplemented!(),
    }
}

fn try_ops(input: &[usize; 4], op: &[usize; 4], output: &[usize; 4]) -> Vec<&'static str> {
    let mut res = vec![];
    let out = output[op[3]];

    for instruction in [
        "addr", "addi", "mulr", "muli", "banr", "bani", "borr", "bori", "setr", "seti", "gtir",
        "gtri", "gtrr", "eqir", "eqri", "eqrr",
    ] {
        if out == do_op(instruction, op, input) {
            res.push(instruction);
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let input = "\
Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]



";
        let expected = "1";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }
}
