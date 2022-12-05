use crate::cli::Result;
use core::ops::Rem;
use num::{BigInt, ToPrimitive};

#[derive(Default)]
pub struct Day {}

// large numbers requires large types, maybe
pub type Card = i128;

pub enum Ops {
    Increment(Card),
    Reverse,
    Cut(Card),
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
        let mut deck: Vec<Card> = (0..=10006).collect();
        deal(&mut deck, ops);
        let result = deck.iter().position(|n| *n == 2019).unwrap();

        // the WAAAAY faster calc, which was supposed to part2 but I messed up and needed not to
        // trace back. I'm keeping this as an assert, just for fun.
        {
            // Offset the end to the value of interest and compute the reverse. You'll get a new
            // position from the start array. Since the start values are the offsets, that gives
            // you the end offset.
            let (_, step) = traceback(0, 1, 10007, ops);
            let (start_offset, _one) = traceback(2019, inv_mult(step, 10007), 10007, ops);
            assert_eq!(result as Card, start_offset * step % 10007);
        }

        Ok(result.to_string())
    }

    // This one is a love/hate. It was a pain to figure the tuple representation was possible,
    // a pain to figure how to combine operations, a pain to find the inverse,
    // a pain to apply to data, a pain to decompose into a geometric series.
    // I didn't knew/remember the math and the many pages of series decomposition
    // were above what normal humans would do.
    // And I did have to fiddle quite a bit with the part1 test result to fix typos.algorithm
    // Yet, I did it over a week, got a result, and now I feel at peace.
    fn part2(&mut self, ops: &Self::Input) -> Result<String> {
        const ITERS: Card = 101_741_582_076_661;
        const BASE: Card = 119_315_717_514_047;

        let (offset, step) = traceback(0, 1, BASE, ops);
        // forward step is the multiplicative inverse
        let step = inv_mult(step, BASE).rem_euclid(BASE);
        let offset = (-offset * step).rem_euclid(BASE);

        // the fiddling...
        // {
        //     let (offset, step) = traceback(0, 1, 10007, ops);
        //     let step = inv_mult(step, 10007).rem_euclid(10007);
        //     let offset = (-offset * step).rem_euclid(10007);
        //     dbg!(&offset, &step);
        //     dbg!(part2(3, (offset, step), 1, 10007));
        // }

        Ok(part2(2020, (offset, step), ITERS, BASE).to_string())
    }
}

fn deal(deck: &mut Vec<Card>, ops: &[Ops]) {
    for op in ops {
        match op {
            Ops::Reverse => deck.reverse(),
            Ops::Cut(n) => {
                let mut tail = deck
                    .splice(0..(n.rem_euclid(deck.len() as Card) as usize), [])
                    .collect();
                deck.append(&mut tail)
            }
            Ops::Increment(n) => {
                let mut new_deck = vec![0; deck.len()];
                deck.iter()
                    .copied()
                    .zip((0..(deck.len() as Card)).cycle().step_by(*n as usize))
                    .for_each(|(x, i)| new_deck[i as usize] = x);
                *deck = new_deck;
            }
        }
    }
}

/*
* Looking at my deck of 10 cards, I made a few realizations:
* 1. it's all about offsets and steps (modulo deck_size).
*   - If I know the first card (offset) and the next one (step), I can infer the
*     rest of the deck.
*   - Reversing is shifting the start offset with the end and stepping by -1.
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
* 4. Whoever made this challenge carefully picked numbers to all be coprime with modulo
*/
fn traceback(mut offset: Card, mut step: Card, modulo: Card, ops: &[Ops]) -> (Card, Card) {
    for op in ops.iter().rev() {
        match op {
            Ops::Reverse => {
                step = (-step).rem_euclid(modulo);
                offset = (offset + step).rem_euclid(modulo);
            }
            Ops::Cut(n) => {
                offset = (offset - n * step).rem_euclid(modulo);
            }
            Ops::Increment(n) => {
                // the reverse op is the offset; how cool is that?!
                step = (step * n).rem_euclid(modulo);
            }
        }
    }
    (offset, step)
}

/* extended Euclid gcd to find the multiplicative mod inverse, as found on wikipedia */
fn inv_mult(mut r1: Card, mut r0: Card) -> Card {
    let (mut s0, mut s1) = (1, 0);
    let (mut t0, mut t1) = (0, 1);

    while r1 > 1 {
        let q = r0.div_euclid(r1);
        let r2 = r0 - q * r1;
        let s2 = s0 - q * s1;
        let t2 = t0 - q * t1;
        (r0, r1, s0, s1, t0, t1) = (r1, r2, s1, s2, t1, t2);
    }
    t1
}

fn part2(pos: Card, (offset, step): (Card, Card), iters: Card, deck: Card) -> Card {
    let offset = BigInt::from(offset);
    let step = BigInt::from(step);
    let base = BigInt::from(deck);
    let iters = BigInt::from(iters);

    let denom: BigInt = inv_mult(
        ((1 - step.to_i64().unwrap()) as Card).rem_euclid(deck),
        deck,
    )
    .rem_euclid(deck)
    .into();

    // Explanation is in test_part2, if I ever need one again.
    // Compound offset and steps.
    let offset: BigInt =
        (&offset * (BigInt::from(1) - step.modpow(&iters, &base)) * denom).rem(&base);
    let step: BigInt = step.modpow(&iters, &base);

    let res: BigInt = (offset + BigInt::from(pos) * step).rem(&base);
    let res = (res + &base).rem(&base);
    res.try_into().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    #[test]
    fn test_trace() {
        let input = "deal into new stack";
        let ops = Day::default().gen(input).unwrap();
        assert_eq!((0, 1), traceback(9, 9, 10, &ops));
        assert_eq!((4, 9), traceback(5, 1, 10, &ops));

        let input = "cut 2";
        let ops = Day::default().gen(input).unwrap();
        assert_eq!((8, 1), traceback(0, 1, 10, &ops));
        let input = "cut -3";
        let ops = Day::default().gen(input).unwrap();
        assert_eq!((0, 1), traceback(7, 1, 10, &ops));

        let input = "deal with increment 7";
        let ops = Day::default().gen(input).unwrap();
        assert_eq!((0, 1), traceback(0, 3, 10, &ops));
    }

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();

        let input = "\
deal with increment 7
deal into new stack
deal into new stack
";
        let data = d.gen(input).unwrap();
        let mut deck: Vec<Card> = (0..=9).collect();
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
        let mut deck: Vec<Card> = (0..=9).collect();
        deal(&mut deck, &data);
        let res: String = deck.iter().map(|n| n.to_string()).collect();
        assert_eq!(expected, &res);

        let input = "\
deal with increment 7
deal with increment 9
cut -2
";
        let data = d.gen(input).unwrap();
        let mut deck: Vec<Card> = (0..=9).collect();
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
        let mut deck: Vec<Card> = (0..=9).collect();
        deal(&mut deck, &data);
        let res: String = deck.iter().map(|n| n.to_string()).collect();
        assert_eq!("9258147036", &res);
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
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
        let ops = d.gen(input).unwrap();
        let mut deck: Vec<Card> = (0..=9).collect();
        const ITERS: Card = 3;
        for _ in 0..ITERS {
            deal(&mut deck, &ops);
        }

        // compound step is: step^iter % base
        // compound offset is: (previous offset * step + step^2) % base
        // Expanding this, that smells of geometric series and wikipedia again helps us remember
        // https://en.wikipedia.org/wiki/Sums_of_powers
        // that Sum over k=i->n (z^k) = (z^i-z^n)/(1-z)
        // ... so our offset n  = n0 * ((1-z^n)/(1-z)) mod base
        // Wrap bits in mod and use mult inverse instead of div to keep numbers small.

        let (offset, step) = traceback(0, 1, 10, &ops);
        // forward step is the multiplicative inverse
        let offset = (offset * step).rem_euclid(10);
        let step = inv_mult(step, 10);

        let offset = (offset * (1 - step.pow(ITERS as u32)) / (1 - step)).rem_euclid(10);
        let step = step.pow(ITERS as u32).rem_euclid(10);
        assert_eq!(
            (deck[0], (deck[1] - deck[0]).rem_euclid(10)),
            (offset, step)
        );
    }
}
