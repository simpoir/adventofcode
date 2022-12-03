use std::sync::mpsc::{channel, Sender};

use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<isize>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        data.split(',').map(|chunk| Ok(chunk.parse()?)).collect()
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let out = channel();
        let result = run(input, [1], out.0).to_string();
        assert_eq!(
            1,
            out.1.iter().skip_while(|x| *x == 0).count(),
            "diag outputs should be 0"
        );
        Ok(result)
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let out = channel();
        let result = run(input, [5], out.0).to_string();
        assert_eq!(
            1,
            out.1.iter().skip_while(|x| *x == 0).count(),
            "diag outputs should be 0"
        );
        Ok(result)
    }
}

fn val(mem: &mut [isize], ip: usize, arg: usize, base: usize) -> &mut isize {
    let mode = mem[ip] as usize / (10 * 10usize.pow(arg as u32)) % 10;
    let idx = ip + arg;
    &mut mem[match mode {
        0 => mem[idx] as usize,
        1 => idx,
        2 => (base as isize + mem[idx]) as usize,
        _ => unimplemented!(),
    }]
}

// Note to time travellers. This function includes functionality from future days.
pub fn run<R>(code: &[isize], input: R, output: Sender<isize>) -> isize
where
    R: IntoIterator<Item = isize>,
{
    let mut input = input.into_iter();
    let mut mem = [0; 10000];
    mem[..code.len()].copy_from_slice(code);
    let mut ip = 0;
    let mut last_output = 0;
    let mut rel_base = 0;
    loop {
        let intcode = mem[ip];
        let op = intcode % 100;
        let mem = &mut mem;
        match op {
            1 => {
                *val(mem, ip, 3, rel_base) =
                    *val(mem, ip, 1, rel_base) + *val(mem, ip, 2, rel_base);
                ip += 4;
            }
            2 => {
                *val(mem, ip, 3, rel_base) =
                    *val(mem, ip, 1, rel_base) * *val(mem, ip, 2, rel_base);
                ip += 4;
            }
            3 => {
                let new_val = if let Some(val) = input.next() {
                    val
                } else {
                    return 0;
                };
                *val(mem, ip, 1, rel_base) = new_val;
                ip += 2;
            }
            4 => {
                last_output = *val(mem, ip, 1, rel_base);
                output.send(last_output).unwrap();
                ip += 2;
            }
            5 => {
                if *val(mem, ip, 1, rel_base) != 0 {
                    ip = *val(mem, ip, 2, rel_base) as usize;
                } else {
                    ip += 3;
                }
            }
            6 => {
                if *val(mem, ip, 1, rel_base) == 0 {
                    ip = *val(mem, ip, 2, rel_base) as usize;
                } else {
                    ip += 3;
                }
            }
            7 => {
                *val(mem, ip, 3, rel_base) =
                    (*val(mem, ip, 1, rel_base) < *val(mem, ip, 2, rel_base)) as isize;
                ip += 4;
            }
            8 => {
                *val(mem, ip, 3, rel_base) =
                    (*val(mem, ip, 1, rel_base) == *val(mem, ip, 2, rel_base)) as isize;
                ip += 4;
            }
            9 => {
                rel_base = (rel_base as isize + *val(mem, ip, 1, rel_base)) as usize;
                ip += 2;
            }
            99 => return last_output,
            _ => unimplemented!(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let input = "1002,6,3,6,4,6,33";
        let expected = "99";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let input = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,\
1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,\
999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";
        let expected = "999";
        let data = d.gen(input).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
