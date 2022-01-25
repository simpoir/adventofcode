#[derive(Default)]
pub struct Day {}

impl crate::cli::Day for Day {
    type Input = Vec<usize>;

    fn gen(&self, data: &str) -> Self::Input {
        data.lines().map(|l| l.parse().unwrap()).collect()
    }

    fn part1(&self, input: &Self::Input) -> String {
        combine(input, 150).to_string()
    }

    fn part2(&self, input: &Self::Input) -> String {
        let mut res = vec![];
        combine2(input, 150, &mut vec![], &mut |x: &Vec<usize>| {
            res.push(x.clone());
        });
        let min = res.iter().map(|l| l.len()).min().unwrap();
        res.iter().filter(|l| l.len() == min).count().to_string()
    }
}

fn combine(buckets: &[usize], volume: usize) -> usize {
    let mut count = 0;
    for (i, bucket) in buckets.iter().copied().enumerate() {
        match bucket {
            b if b > volume => continue,
            b if b == volume => {
                count += 1;
                continue;
            }
            _ => count += combine(&buckets[i + 1..], volume - bucket),
        }
    }
    count
}

fn combine2<F: FnMut(&Vec<usize>)>(
    buckets: &[usize],
    volume: usize,
    combi: &mut Vec<usize>,
    func: &mut F,
) {
    for (i, bucket) in buckets.iter().copied().enumerate() {
        match bucket {
            b if b > volume => continue,
            b if b == volume => {
                combi.push(bucket);
                func(combi);
                combi.pop();
                continue;
            }
            _ => {
                combi.push(bucket);
                combine2(&buckets[i + 1..], volume - bucket, combi, func);
                combi.pop();
            }
        }
    }
}
