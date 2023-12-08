use std::collections::HashMap;

use crate::cli::Result;

const CARDS: &[u8; 13] = b"23456789TJQKA";

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
#[repr(u8)]
enum Hand {
    HighCard,
    OnePair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<(Vec<u8>, u32)>;

    fn gen(&mut self, data: &'i str) -> Result<Self::Input> {
        Ok(data
            .lines()
            .map(|l| (l[..5].bytes().collect(), l[6..].parse::<u32>().unwrap()))
            .collect())
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        // sort
        let input: Vec<(Vec<usize>, u32)> = input
            .iter()
            .map(|(hand, bet)| {
                (
                    hand.iter()
                        .copied()
                        .map(|c| CARDS.iter().position(|v| *v == c).unwrap())
                        .collect(),
                    *bet,
                )
            })
            .collect();

        let mut hands: Vec<(Hand, &Vec<usize>, u32)> = input
            .iter()
            .map(|(cards, bet)| (get_hand(cards), cards, *bet))
            .collect();
        hands.sort();
        Ok(hands
            .iter()
            .enumerate()
            .map(|(i, (_, _, bet))| (i + 1) * (*bet as usize))
            .sum::<usize>()
            .to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        // sort differently
        const CARDS: &[u8; 13] = b"J23456789TQKA";
        let input: Vec<(Vec<usize>, u32)> = input
            .iter()
            .map(|(hand, bet)| {
                (
                    hand.iter()
                        .copied()
                        .map(|c| CARDS.iter().position(|v| *v == c).unwrap())
                        .collect(),
                    *bet,
                )
            })
            .collect();

        let mut hands: Vec<(Hand, &Vec<usize>, u32)> = input
            .iter()
            .map(|(cards, bet)| (get_wild_hand(cards), cards, *bet))
            .collect();
        hands.sort();
        Ok(hands
            .iter()
            .enumerate()
            .map(|(i, (_, _, bet))| (i + 1) * (*bet as usize))
            .sum::<usize>()
            .to_string())
    }
}

fn get_hand(cards: &[usize]) -> Hand {
    let mut counts = HashMap::<usize, u8>::new();
    for &c in cards {
        *counts.entry(c).or_default() += 1;
    }
    let counts: Vec<u8> = counts.values().copied().collect();
    if counts.contains(&5) {
        Hand::Five
    } else if counts.contains(&4) {
        Hand::Four
    } else if counts.contains(&3) {
        if counts.contains(&2) {
            Hand::FullHouse
        } else {
            Hand::Three
        }
    } else {
        let pairs = counts.iter().filter(|v| v == &&2).count();
        match pairs {
            2 => Hand::TwoPair,
            1 => Hand::OnePair,
            _ => Hand::HighCard,
        }
    }
}

fn get_wild_hand(cards: &[usize]) -> Hand {
    let mut counts = HashMap::<usize, u8>::new();
    for &c in cards {
        *counts.entry(c).or_default() += 1;
    }
    let jokers = counts.remove(&0).unwrap_or(0);
    let counts: Vec<u8> = counts.values().copied().collect();
    if counts.contains(&5) {
        Hand::Five
    } else if counts.contains(&4) {
        if jokers > 0 {
            Hand::Five
        } else {
            Hand::Four
        }
    } else if counts.contains(&3) {
        match jokers {
            2 => Hand::Five,
            1 => Hand::Four,
            _ => {
                if counts.contains(&2) {
                    Hand::FullHouse
                } else {
                    Hand::Three
                }
            }
        }
    } else {
        let pairs = counts.iter().filter(|v| v == &&2).count();
        match (pairs, jokers) {
            (2, 1) => Hand::FullHouse,
            (2, _) => Hand::TwoPair,
            (1, 3) => Hand::Five,
            (1, 2) => Hand::Four,
            (1, 1) => Hand::Three,
            (1, _) => Hand::OnePair,
            (_, 5) | (_, 4) => Hand::Five,
            (_, 3) => Hand::Four,
            (_, 2) => Hand::Three,
            (_, 1) => Hand::OnePair,
            _ => Hand::HighCard,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "6440";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "5905";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
