const MOD: u64 = 20201227;

day! {
    day25;
    type INPUT = (u64, u64);

    fn gen(file: &mut impl BufRead) -> Result<Self::INPUT> {
        let mut res = file.lines().take(2).map(|l| l.unwrap().parse::<u64>().unwrap());
        Ok((res.next().unwrap(), res.next().unwrap()))
    }

    fn part1((first, second): &Self::INPUT) -> Result<String> {
        const SUBJECT: u64 = 7;
        let mut value = 1;
        let mut secret = 0;
        while value != *first {
            secret += 1;
            value = (value * SUBJECT) % MOD;
        }
        value = 1;
        for _ in 0..secret {
            value = (value * second) % MOD;
        }
        Ok(value.to_string())
    }

    fn part2(_: &Self::INPUT) -> Result<String> {
        Ok("".into())
    }
}
