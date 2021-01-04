use std::collections::HashMap;

day! {
    day10;
    type INPUT = Vec<u64>;

    fn gen(file: &mut impl BufRead) -> Result<Self::INPUT> {
        let mut res = String::new();
        file.read_to_string(&mut res)?;
        let mut arr: Vec<u64> = res.lines().map(|line| line.parse().unwrap()).collect();
        arr.sort_unstable();
        Ok(arr)
    }

    fn part1(input: &Self::INPUT) -> Result<String> {
        let res = input.iter().fold(((0, 0, 1), 0), |(diff, last), x| {
            (
                match x - last {
                    1 => (diff.0 + 1, diff.1, diff.2),
                    2 => (diff.0, diff.1 + 1, diff.2),
                    _ => (diff.0, diff.1, diff.2 + 1),
                },
                *x,
            )
        });
        Ok(format!("{}", res.0 .0 * res.0 .2))
    }

    fn part2(input: &Self::INPUT) -> Result<String> {
        let mut paths = HashMap::new();
        paths.insert(input[input.len() - 1] + 3, 1);
        for x in input.iter().rev() {
            paths.insert(*x, ((1..4).filter_map(|i| paths.get(&(x + i)))).sum());
        }

        Ok(format!(
            "{}",
            (1..4).filter_map(|i| paths.get(&i)).sum::<usize>()
        ))
    }
}
