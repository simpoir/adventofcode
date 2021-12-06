pub struct Day {}

impl crate::Day for Day {
    type Input = Vec<u8>;

    fn gen(&self, data: &str) -> Self::Input {
        data.trim().split(',').map(|x| x.parse().unwrap()).collect()
    }

    fn part1(&self, input: &Self::Input) -> String {
        format!("{}", sim(input, 80))
    }

    fn part2(&self, input: &Self::Input) -> String {
        format!("{}", sim(input, 256))
    }
}

fn sim(input: &[u8], days: usize) -> usize {
    let mut data = input.iter().fold([0usize; 9], |tot, x| {
        let mut tot = tot;
        tot[*x as usize] += 1;
        tot
    });
    for _ in 0..days {
        let new = data[0];
        for i in 1..9 {
            data[i - 1] = data[i];
        }
        data[8] = new;
        data[6] += new;
    }
    data.iter().sum()
}
