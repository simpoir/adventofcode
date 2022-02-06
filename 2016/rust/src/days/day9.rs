#[derive(Default)]
pub struct Day {}

impl crate::cli::Day for Day {
    type Input = String;

    fn gen(&self, data: &str) -> Self::Input {
        data.lines().collect()
    }

    fn part1(&self, input: &Self::Input) -> String {
        unpack(input, false).to_string()
    }

    fn part2(&self, input: &Self::Input) -> String {
        unpack(input, true).to_string()
    }
}

fn unpack(input: &str, recurse: bool) -> usize {
    let mut res = 0;
    let mut it = input.chars();
    while let Some(b) = it.next() {
        match b {
            '(' => {
                let mut nc = 0;
                for n in it.by_ref() {
                    if n.is_digit(10) {
                        nc = nc * 10 + (n as usize - '0' as usize);
                    } else {
                        break;
                    }
                }
                let mut rep = 0;
                for n in it.by_ref() {
                    if n.is_digit(10) {
                        rep = rep * 10 + (n as usize - '0' as usize);
                    } else {
                        break;
                    }
                }
                let mut chunk = String::new();
                for _ in 0..nc {
                    chunk.push(it.next().unwrap());
                }
                if !recurse {
                    res += chunk.len() * rep;
                } else {
                    res += unpack(&chunk, true) * rep;
                }
            }
            _ => res += 1,
        }
    }
    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_expand() {
        let input = "X(8x2)(3x3)ABCY";
        let expected = 18;
        assert_eq!(expected, unpack(input, false));
    }

    #[test]
    fn test_expand2() {
        let input = "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN";
        let expected = 445;
        assert_eq!(expected, unpack(input, true));
    }
}
