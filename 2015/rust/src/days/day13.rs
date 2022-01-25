#[derive(Default)]
pub struct Day {}

impl crate::cli::Day for Day {
    type Input = Vec<Vec<isize>>;

    fn gen(&self, data: &str) -> Self::Input {
        let mut names = vec![];
        data.lines().for_each(|l| {
            let (chunk, _) = l.split_once(' ').unwrap();
            if !names.contains(&chunk) {
                names.push(chunk);
            }
        });
        let mut res = vec![vec![0; names.len()]; names.len()];
        for l in data.lines() {
            let mut chunks = l.split(' ');
            let p1 = chunks.next().unwrap();
            let sign = chunks.nth(1).unwrap() == "gain";
            let hap: isize = if sign {
                chunks.next().unwrap().parse().unwrap()
            } else {
                -chunks.next().unwrap().parse::<isize>().unwrap()
            };
            let p2 = chunks.last().unwrap().strip_suffix('.').unwrap();
            let p1 = names.iter().position(|p| *p == p1).unwrap();
            let p2 = names.iter().position(|p| *p == p2).unwrap();
            res[p1][p2] = hap;
        }
        res
    }

    fn part1(&self, costs: &Self::Input) -> String {
        let mut max = 0;
        let persons: Vec<usize> = (0..costs.len()).collect();
        let mut placements = vec![0; costs.len()];
        combine(&persons, &mut placements, 0, &mut |n| {
            max = max.max(score(n, costs))
        });
        max.to_string()
    }

    fn part2(&self, costs: &Self::Input) -> String {
        let mut xcosts: Self::Input = costs
            .iter()
            .map(|l| {
                let mut l: Vec<isize> = l.clone();
                l.push(0);
                l
            })
            .collect();
        xcosts.push(vec![0; costs.len() + 1]);
        let costs = xcosts;
        let mut max = 0;
        let persons: Vec<usize> = (0..costs.len()).collect();
        let mut placements = vec![0; costs.len()];
        combine(&persons, &mut placements, 0, &mut |n| {
            max = max.max(score(n, &costs))
        });
        max.to_string()
    }
}

fn score(placement: &[usize], costs: &[Vec<isize>]) -> isize {
    let last = costs.len() - 1;
    placement
        .windows(2)
        .map(|a| costs[a[0]][a[1]] + costs[a[1]][a[0]])
        .sum::<isize>()
        + costs[placement[0]][placement[last]]
        + costs[placement[last]][placement[0]]
}

fn combine<F: FnMut(&[usize])>(
    persons: &[usize],
    placement: &mut [usize],
    idx: usize,
    closure: &mut F,
) {
    if persons.is_empty() {
        closure(placement);
        return;
    }
    let next = idx + 1;
    for p in persons {
        placement[idx] = *p;
        let persons: Vec<_> = persons.iter().filter(|x| *x != p).copied().collect();
        combine(&persons, placement, next, closure);
    }
}
