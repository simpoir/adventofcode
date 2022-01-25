#[derive(Default)]
pub struct Day {}

impl crate::cli::Day for Day {
    type Input = String;

    fn gen(&self, data: &str) -> Self::Input {
        data.to_string()
    }

    fn part1(&self, input: &Self::Input) -> String {
        let mut res = input.clone();
        for _ in 0..40 {
            res = expand(&res);
        }
        res.len().to_string()
    }

    fn part2(&self, input: &Self::Input) -> String {
        let mut res = input.clone();
        for _ in 0..50 {
            res = expand(&res);
        }
        res.len().to_string()
    }
}

fn expand(res: &str) -> String {
    let mut it = res.chars();
    let mut prev = it.next().unwrap();
    let mut count = 1;
    let mut result = String::new();
    for c in it {
        if c == prev {
            count += 1;
        } else {
            result.push_str(&count.to_string());
            result.push(prev);
            prev = c;
            count = 1;
        }
    }
    result.push_str(&count.to_string());
    result.push(prev);
    result
}
