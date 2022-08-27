use std::collections::HashSet;

use super::day19::do_op;
use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = (usize, Vec<(&'i str, [usize; 3])>);

    fn gen(&mut self, data: &'i str) -> Result<Self::Input> {
        super::day19::Day {}.gen(data)
    }

    fn part1(&mut self, (ip, code): &Self::Input) -> Result<String> {
        // #ip 4
        // 00 seti 123 0 2           { r2 = 123
        // 01 bani 2 456 2             r2 &= 456
        // 02 eqri 2 72 2            } while r2 != 72
        // 03 addr 2 4 4             --
        // 04 seti 0 0 4             --
        // 05 seti 0 1 2             r2 = 0
        // 06 bori 2 65536 5         r5 = r2 | 0x10000
        // 07 seti 16123384 4 2      r2 = 0xf605f8
        // 08 bani 5 255 3           r3 = r5 & 0xff
        // 09 addr 2 3 2             r2 += r3
        // 10 bani 2 16777215 2      r2 &= 0xffffff
        // 11 muli 2 65899 2         r2 *= 65899
        // 12 bani 2 16777215 2      r2 &= 0xffffff
        // 13 gtir 256 5 3           if 256 > r5
        // 14 addr 3 4 4               --
        // 15 addi 4 1 4               --
        // 16 seti 27 6 4              GOTO 28
        // 17 seti 0 3 3             r3 = 0
        // 18 addi 3 1 1             while (r3 + 1) * 256 <= r5
        // 19 muli 1 256 1             --
        // 20 gtrr 1 5 1               --
        // 21 addr 1 4 4               --
        // 22 addi 4 1 4               --
        // 23 seti 25 6 4              --
        // 24 addi 3 1 3               r3 += 1
        // 25 seti 17 3 4              --
        // 26 setr 3 8 5             r5 = r3      // r5 /= 256
        // 27 seti 7 2 4             GOTO 8
        // 28 eqrr 2 0 3             if r0 == r2
        // 29 addr 3 4 4               EXIT
        // 30 seti 5 3 4             GOTO 6
        let mut registers = [0; 6];

        while let Some((op, code)) = code.get(registers[*ip]) {
            registers[code[2]] = do_op(op, code, &registers);
            registers[*ip] += 1;
            if registers[*ip] == 28 {
                break;
            }
        }
        Ok(registers[2].to_string())
    }

    fn part2(&mut self, (ip, code): &Self::Input) -> Result<String> {
        let mut registers = [0; 6];
        let mut seen = HashSet::new();
        let mut prev = 0;

        while let Some((op, code)) = code.get(registers[*ip]) {
            registers[code[2]] = do_op(op, code, &registers);
            registers[*ip] += 1;
            if registers[*ip] == 28 {
                if !seen.insert(registers[2]) {
                    break;
                }
                prev = registers[2];
            }
        }
        Ok(prev.to_string())
    }
}
