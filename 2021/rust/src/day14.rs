use std::collections::BTreeMap;

pub struct Day {}

impl crate::Day for Day {
    type Input = (Vec<usize>, [[usize; 26]; 26]);

    fn gen(&self, data: &str) -> Self::Input {
        let (start, map_data) = data.trim_end().split_once("\n\n").unwrap();
        let start = start.bytes().map(|c| (c - b'A') as usize).collect();
        let mut map = [[0; 26]; 26];
        map_data.lines().for_each(|l| {
            let l = l.as_bytes();
            map[(l[0] - b'A') as usize][(l[1] - b'A') as usize] = (l[6] - b'A') as usize;
        });
        (start, map)
    }

    fn part1(&self, input: &Self::Input) -> String {
        let mut chain = input.0.clone();
        let map = &input.1;
        for _ in 0..10 {
            let last = *chain.last().unwrap();
            chain = chain
                .windows(2)
                .map(|x| [x[0], map[x[0]][x[1]]])
                .flatten()
                .collect();
            chain.push(last);
        }

        let mut counts = [0; 26];
        for c in &chain {
            counts[*c] += 1;
        }
        let max = counts.iter().max().unwrap();
        let min = counts.iter().filter(|x| **x != 0).min().unwrap();

        format!("{}", max - min)
    }

    fn part2(&self, input: &Self::Input) -> String {
        let map = &input.1;
        let mut sets: BTreeMap<(usize, usize), usize> = BTreeMap::new();
        input.0.windows(2).for_each(|x| {
            *sets.entry((x[0], x[1])).or_insert(0) += 1;
        });
        for _ in 0..40 {
            let cur_sets = sets;
            sets = BTreeMap::new();
            for ((a, b), count) in cur_sets {
                let mid = map[a][b];
                *sets.entry((a, mid)).or_insert(0) += count;
                *sets.entry((mid, b)).or_insert(0) += count;
            }
        }

        let mut counts = [0; 26];
        for ((a, b), count) in sets {
            counts[a] += count;
            counts[b] += count;
        }
        // first and last are one less
        counts[input.0[0]] += 1;
        counts[*input.0.last().unwrap()] += 1;

        let max = counts.iter().max().unwrap() / 2;
        let min = counts.iter().filter(|x| **x != 0).min().unwrap() / 2;

        format!("{}", max - min)
    }
}
