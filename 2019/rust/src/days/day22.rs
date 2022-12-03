use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

pub enum Ops {
    Increment(usize),
    Reverse,
    Cut(isize),
}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<Ops>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        data.lines()
            .map(|l| {
                if l.starts_with("deal with") {
                    Ok(Ops::Increment(l.rsplit_once(' ').unwrap().1.parse()?))
                } else if l.starts_with("deal into") {
                    Ok(Ops::Reverse)
                } else {
                    Ok(Ops::Cut(l.rsplit_once(' ').unwrap().1.parse()?))
                }
            })
            .collect()
    }

    fn part1(&mut self, ops: &Self::Input) -> Result<String> {
        let mut deck: Vec<u32> = (0..=10006).collect();
        deal(&mut deck, ops);
        Ok(deck.iter().position(|n| *n == 2019).unwrap().to_string())
    }

    fn part2(&mut self, _input: &Self::Input) -> Result<String> {
        Ok("".to_string())
    }
}

fn deal(deck: &mut Vec<u32>, ops: &[Ops]) {
    for op in ops {
        match op {
            Ops::Reverse => deck.reverse(),
            Ops::Cut(n) => {
                let mut tail = deck
                    .splice(0..(n.rem_euclid(deck.len() as isize) as usize), [])
                    .collect();
                deck.append(&mut tail)
            }
            Ops::Increment(n) => {
                let mut new_deck = vec![0; deck.len()];
                deck.iter()
                    .copied()
                    .zip((0..(deck.len())).cycle().step_by(*n))
                    .for_each(|(x, i)| new_deck[i] = x);
                *deck = new_deck;
            }
        }
    }
}

/*
* Looking at my deck of 10 cards, I made a few realizations:
* 1. it's all about offsets and steps (modulo deck_size).
*   - Reversing is shifting the offset with the end and stepping by -1.
*   - cutting is shifting the offset
*   - increment deal is all about the steps.
*     * only 1 3 7 9 are coprime with 10 and yield a valid deck without overlap
*     * (1, 11, etc) and (9, -1, etc) are straightforward
*     * going forward, we (end_pos * shift) % deck_size -> pos_value
*     * 3 is the inverse multiplicative of 7 and vice-versa
*     * if we go backward, we need the inverse.
*   - those operations are not commutative
* 2. So we can represent the state of the deck as (offset, step).
* 3. Also, that state is reversible.
*/
fn traceback(pos: usize, deck_len: usize, ops: &[Ops]) -> usize {
    let mut pos = pos;
    for op in ops.iter().rev() {
        pos = match op {
            Ops::Reverse => deck_len - pos - 1,
            Ops::Cut(n) => (pos as isize + *n).rem_euclid(deck_len as isize) as usize,
            Ops::Increment(n) => (n * pos) % deck_len,
            // pos = (n * val) % deck_len
            // val =
        }
    }
    pos
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();

        let input = "\
deal with increment 7
deal into new stack
deal into new stack
";
        let data = d.gen(input).unwrap();
        let mut deck: Vec<u32> = (0..=9).collect();
        deal(&mut deck, &data);
        let res: String = deck.iter().map(|n| n.to_string()).collect();
        assert_eq!("0369258147", &res);

        let input = "\
cut 6
deal with increment 7
deal into new stack
";
        let expected = "3074185296";
        let data = d.gen(input).unwrap();
        let mut deck: Vec<u32> = (0..=9).collect();
        deal(&mut deck, &data);
        let res: String = deck.iter().map(|n| n.to_string()).collect();
        assert_eq!(expected, &res);

        let input = "\
deal with increment 7
deal with increment 9
cut -2
";
        let data = d.gen(input).unwrap();
        let mut deck: Vec<u32> = (0..=9).collect();
        deal(&mut deck, &data);
        let res: String = deck.iter().map(|n| n.to_string()).collect();
        assert_eq!("6307418529", &res);

        let input = "\
deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1";
        let data = d.gen(input).unwrap();
        let mut deck: Vec<u32> = (0..=9).collect();
        deal(&mut deck, &data);
        let res: String = deck.iter().map(|n| n.to_string()).collect();
        assert_eq!("9258147036", &res);
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let input = "\
deal with increment 7
";
        let data = d.gen(input).unwrap();
        assert_eq!(6, traceback(9, 10, &data))
    }
}
