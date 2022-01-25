#[derive(Default)]
pub struct Day {}

impl crate::cli::Day for Day {
    type Input = Vec<Vec<usize>>;

    fn gen(&self, data: &str) -> Self::Input {
        let mut places: Vec<&str> = vec![];
        data.lines().for_each(|l| {
            let mut chunks = l.split(' ');
            let c = chunks.next().unwrap();
            if !places.contains(&c) {
                places.push(c);
            }
            let c = chunks.nth(1).unwrap();
            if !places.contains(&c) {
                places.push(c);
            }
        });
        let mut res = vec![vec![usize::MIN; places.len()]; places.len()];
        data.lines().for_each(|l| {
            let (from, tail) = l.split_once(" to ").unwrap();
            let (to, distance) = tail.split_once(" = ").unwrap();
            let from = places.iter().position(|x| *x == from).unwrap();
            let to = places.iter().position(|x| *x == to).unwrap();
            let distance = distance.parse().unwrap();
            res[from][to] = distance;
            res[to][from] = distance;
        });
        res
    }

    fn part1(&self, input: &Self::Input) -> String {
        let places: Vec<usize> = (0..input.len()).collect();
        let mut res = usize::MAX;
        for place in &places {
            let places: Vec<usize> = places.iter().copied().filter(|p| p != place).collect();
            res = usize::min(res, walk(0, *place, &places, input, usize::min));
        }
        res.to_string()
    }

    fn part2(&self, input: &Self::Input) -> String {
        let places: Vec<usize> = (0..input.len()).collect();
        let mut res = usize::MIN;
        for place in &places {
            let places: Vec<usize> = places.iter().copied().filter(|p| p != place).collect();
            res = usize::max(res, walk(0, *place, &places, input, usize::max));
        }
        res.to_string()
    }
}

fn walk(
    total: usize,
    prev: usize,
    places: &[usize],
    map: &[Vec<usize>],
    predicate: fn(usize, usize) -> usize,
) -> usize {
    if places.len() == 1 {
        return total + map[prev][places[0]];
    }

    let mut res = usize::MAX;
    for (i, place) in places.iter().enumerate() {
        let total = total + map[prev][*place];
        let places: Vec<usize> = places.iter().copied().filter(|p| p != place).collect();
        if i != 0 {
            res = predicate(res, walk(total, *place, &places, map, predicate));
        } else {
            res = walk(total, *place, &places, map, predicate);
        }
    }
    res
}
