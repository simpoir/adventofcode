use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = (usize, Vec<(&'i str, [usize; 3])>);

    fn gen(&mut self, data: &'i str) -> Result<Self::Input> {
        let mut lines = data.lines();
        let ip = lines
            .next()
            .unwrap()
            .split_ascii_whitespace()
            .last()
            .unwrap()
            .parse()?;
        let code = lines
            .map(|l| {
                let mut chunks = l.split_ascii_whitespace();
                Result::Ok((
                    chunks.next().unwrap(),
                    [
                        chunks.next().unwrap().parse::<usize>()?,
                        chunks.next().unwrap().parse::<usize>()?,
                        chunks.next().unwrap().parse::<usize>()?,
                    ],
                ))
            })
            .collect::<Result<Vec<(&str, [usize; 3])>>>()?;
        Ok((ip, code))
    }

    fn part1(&mut self, (ip, code): &Self::Input) -> Result<String> {
        let mut registers = [0; 6];

        while let Some((op, code)) = code.get(registers[*ip]) {
            registers[code[2]] = do_op(op, code, &registers);
            registers[*ip] += 1;
        }

        Ok(registers[0].to_string())
    }

    fn part2(&mut self, (ip, code): &Self::Input) -> Result<String> {
        // 00 addi 5 16 5     GOTO 17 {
        // 01 seti 1 2 2       r2 = 1
        // 02 seti 1 0 4       r4 = 1           do { do {
        // 03 mulr 2 4 3       r3 = r2 * r4      --
        // 04 eqrr 3 1 3       r3 = r3 == r1     if r2*r4 != r1 {
        // 05 addr 3 5 5       JMP +r3             --
        // 06 addi 5 1 5       GOTO 08             --
        // 07 addr 2 0 0       r0 += r2            r0 += r2}
        // 08 addi 4 1 4       r4 += 1             r4 ++
        // 09 gtrr 4 1 3       r3 = r4 > r1      } while r4 <= r1
        // 10 addr 5 3 5       JMP +r3           --
        // 11 seti 2 4 5       GOTO 02           --
        // 12 addi 2 1 2       r2 += 1            r2 ++
        // 13 gtrr 2 1 3       r3 = r2 > r1      } while r2 <= r1
        // 14 addr 3 5 5       JMP +r3            --
        // 15 seti 1 1 5       GOTO 02            --
        // 16 mulr 5 5 5       JMP 256 // END     exit r0
        // 17 addi 1 2 1     }r1 += 2             --
        // 18 mulr 1 1 1      r1 *= r1            r1 = (r1+2)^2*209 + (r3+6)*22+15
        // 19 mulr 5 1 1      r1 *= 19            --
        // 20 muli 1 11 1     r1 *= 11            --
        // 21 addi 3 6 3      r3 += 6             --
        // 22 mulr 3 5 3      r3 *= 22            --
        // 23 addi 3 15 3     r3 += 15            -- //r3 overwritten
        // 24 addr 1 3 1      r1 += r3            --
        // 25 addr 5 0 5      JMP +r0             if r0 == 1
        // 26 seti 0 7 5      GOTO 01                goto 01
        // 27 setr 5 8 3      r3 = 27             r3 = (27 * 28 + 29) * 30 * 14 * 32
        // 28 mulr 3 5 3      r3 *= 28            --
        // 29 addr 5 3 3      r3 += 29            --
        // 30 mulr 5 3 3      r3 *= 30            --
        // 31 muli 3 14 3     r3 *= 14            --
        // 32 mulr 3 5 3      r3 *= 32            --
        // 33 addr 1 3 1      r1 += r3            r1 += 10550400
        // 34 seti 0 0 0      r0 = 0              r0 == 0
        // 35 seti 0 6 5      GOTO 01

        // In short, we generate a large number at l18 or larger number at l27 for part2.
        // Store that in r1.
        // Increment r2 + r4 to find factors of r1.
        // And sum those factors in r0.
        // So here we'll hijack the vm to generate until l01, then use erathosthene's sieve
        // to short circuit the iteration.

        let mut registers = [1, 0, 0, 0, 0, 0];
        while registers[*ip] != 1 {
            if let Some((op, code)) = code.get(registers[*ip]) {
                registers[code[2]] = do_op(op, code, &registers);
                registers[*ip] += 1;
            } else {
                panic!("your puzzle input doesn't match expectations")
            }
        }
        let tgt = registers[1];
        let result: usize = (1..=((tgt as f32).sqrt().floor() as usize))
            .filter_map(|a| {
                if tgt % a == 0 {
                    Some((tgt / a) + a)
                } else {
                    None
                }
            })
            .sum();

        Ok(result.to_string())
    }
}

pub fn do_op<'i>(op: &'i str, code: &[usize; 3], registers: &[usize; 6]) -> usize {
    let r1 = *registers.get(code[0]).unwrap_or(&0);
    let r2 = *registers.get(code[1]).unwrap_or(&0);
    let v1 = code[0];
    let v2 = code[1];
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let input = "\
#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5";
        let expected = "7";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }
}
