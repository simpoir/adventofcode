use crate::cli::Result;
use std::collections::HashMap;

#[derive(Default)]
pub struct Day {}

fn slice_map(chunks: &[[i64; 2]], slicers: &[[i64; 3]]) -> Vec<[i64; 2]> {
    let mut chunks = chunks.to_owned();
    let mut ret = vec![];
    for &[dst, src, len] in slicers {
        chunks = chunks
            .into_iter()
            .flat_map(|[mut start, mut chunk_len]| {
                let end = start + chunk_len;
                let mut slices = vec![];
                if start < src {
                    let new_start = end.min(src);
                    let sub_len = new_start - start;
                    slices.push([start, sub_len]);
                    start = new_start;
                    chunk_len -= sub_len;
                }
                if chunk_len > 0 && start < src + len {
                    // no overlap. mid chunk is gold.
                    let mid_len = chunk_len.min(len - (start - src));
                    ret.push([start + dst - src, mid_len]);
                    start += mid_len;
                    chunk_len -= mid_len;
                }
                if chunk_len > 0 {
                    slices.push([start, chunk_len]);
                }
                slices
            })
            .collect();
    }
    // add any unmapped chunk
    ret.append(&mut chunks);

    ret
}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = (Vec<i64>, HashMap<&'i str, (&'i str, Vec<[i64; 3]>)>);

    fn gen(&mut self, data: &'i str) -> Result<Self::Input> {
        let mut lines = data.lines();
        let seeds = lines
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .split_ascii_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        lines.next();
        let mut map: HashMap<&str, (&str, Vec<[i64; 3]>)> = HashMap::new();
        while let Some(header) = lines.next() {
            let (fro, to) = header
                .split_once(' ')
                .unwrap()
                .0
                .split_once("-to-")
                .unwrap();
            let mappings = &mut map.entry(fro).or_insert((to, vec![])).1;
            for line in lines.by_ref() {
                if line.is_empty() {
                    break;
                }
                let mut mm = [0; 3];
                line.split_ascii_whitespace()
                    .enumerate()
                    .for_each(|(i, n)| mm[i] = n.parse().unwrap());
                mappings.push(mm);
            }
        }
        Ok((seeds, map))
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let mut fro = "seed";
        let (from_items, mapping) = &input;
        let mut from_items = from_items.clone();

        while fro != "location" {
            let (to, map) = &mapping[fro];
            from_items = from_items
                .into_iter()
                .map(|item| {
                    for &[dst, src, len] in map {
                        if (src..(src + len)).contains(&item) {
                            return item + dst - src;
                        }
                    }
                    item
                })
                .collect();
            fro = to;
        }

        Ok(from_items.iter().min().unwrap().to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let mut fro = "seed";
        let (init_from_items, mapping) = &input;
        let mut from_items = vec![];
        for chunk in init_from_items.chunks_exact(2) {
            from_items.push([chunk[0], chunk[1]]);
        }

        while fro != "location" {
            let (to, map) = &mapping[fro];
            from_items = slice_map(&from_items, map);
            fro = to;
        }

        assert_eq!(
            input.0.chunks_exact(2).map(|c| c[1]).sum::<i64>(),
            from_items.iter().map(|c| c[1]).sum(),
        );

        Ok(from_items.iter().map(|l| l[0]).min().unwrap().to_string())
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeSet;

    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "35";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "46";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }

    #[test]
    fn test_part2_trace() {
        let mut d: Day = Default::default();
        let expected = "46";
        let mut data = d.gen(INPUT).unwrap();
        data.0 = vec![82, 1];
        assert_eq!(expected, d.part2(&data).unwrap());
    }

    #[test]
    fn test_slice() {
        let expected = BTreeSet::from([[1, 4], [10, 2], [7, 3]]);
        let res = slice_map(&[[1, 9]], &[[10, 5, 2]]);
        assert_eq!(expected, BTreeSet::from_iter(res));

        let expected = vec![[1, 3]];
        let res = slice_map(&[[1, 3]], &[[10, 5, 2]]);
        assert_eq!(expected, res);

        let expected = vec![[11, 1]];
        let res = slice_map(&[[6, 1]], &[[10, 5, 3]]);
        assert_eq!(expected, res);

        let expected = vec![[11, 1], [7, 1]];
        let res = slice_map(&[[6, 2]], &[[10, 5, 2]]);
        assert_eq!(expected, res);

        let expected = vec![[9, 1]];
        let res = slice_map(&[[9, 1]], &[[10, 5, 2]]);
        assert_eq!(expected, res);
    }
}
