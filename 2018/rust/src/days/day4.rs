use std::collections::HashMap;

use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

pub type Ts = (u16, u8, u8, u8, u8);

#[derive(PartialOrd, Ord, PartialEq, Eq)]
pub enum Entry {
    Start(u16),
    Asleep,
    Awake,
}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<(Ts, Entry)>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        let mut res = data
            .lines()
            .map(|l| {
                let mut chunks = l.split(['[', ']', '-', ':', ' ', '#']);
                Ok((
                    (
                        chunks.nth(1).unwrap().parse()?,
                        chunks.next().unwrap().parse()?,
                        chunks.next().unwrap().parse()?,
                        chunks.next().unwrap().parse()?,
                        chunks.next().unwrap().parse()?,
                    ),
                    match chunks.nth(1) {
                        Some("Guard") => Entry::Start(chunks.nth(1).unwrap().parse()?),
                        Some("falls") => Entry::Asleep,
                        _ => Entry::Awake,
                    },
                ))
            })
            .collect::<Result<Self::Input>>()?;
        res.sort();
        Ok(res)
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let totals = agg(input);
        let most_asleep = totals
            .iter()
            .max_by_key(|x| x.1.iter().sum::<u32>())
            .unwrap()
            .0;
        let minute = totals
            .get(most_asleep)
            .unwrap()
            .iter()
            .enumerate()
            .max_by_key(|x| x.1)
            .unwrap()
            .0;
        Ok((*most_asleep as u32 * minute as u32).to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let totals = agg(input);
        let (guard, minute, _) = totals
            .iter()
            .flat_map(|(k, v)| v.iter().enumerate().map(|x| (*k, x.0, x.1)))
            .max_by_key(|x| x.2)
            .unwrap();
        Ok((guard as u32 * minute as u32).to_string())
    }
}

fn agg(input: &<Day as crate::cli::Day>::Input) -> HashMap<u16, [u32; 60]> {
    let mut totals = HashMap::new();
    let mut guard = 0;
    let mut start = 0;
    for (ts, entry) in input {
        match entry {
            Entry::Start(g) => guard = *g,
            Entry::Asleep => start = ts.4,
            Entry::Awake => totals.entry(guard).or_insert([0u32; 60])
                [start as usize..ts.4 as usize]
                .iter_mut()
                .for_each(|x| *x += 1),
        }
    }
    totals
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;

    const INPUT: &str = "\
[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "240";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "4455";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
