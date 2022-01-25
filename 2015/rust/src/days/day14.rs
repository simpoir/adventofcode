#[derive(Default)]
pub struct Day {}

const RUNTIME: usize = 2503;

impl crate::cli::Day for Day {
    type Input = Vec<(usize, usize, usize)>;

    fn gen(&self, data: &str) -> Self::Input {
        data.lines()
            .map(|l| {
                let mut chunks = l.split(' ');
                (
                    chunks.nth(3).unwrap().parse().unwrap(),
                    chunks.nth(2).unwrap().parse().unwrap(),
                    chunks.nth_back(1).unwrap().parse().unwrap(),
                )
            })
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> String {
        input
            .iter()
            .map(|(speed, run, pause)| {
                let runtime = run + pause;
                (RUNTIME / runtime) * run * speed + (RUNTIME % runtime).min(*run) * speed
            })
            .max()
            .unwrap()
            .to_string()
    }

    fn part2(&self, input: &Self::Input) -> String {
        let mut states: Vec<(usize, usize, bool)> = vec![(0, 0, false); input.len()];
        let mut scores = vec![0; input.len()];
        for _ in 0..RUNTIME {
            for (i, state) in states.iter_mut().enumerate() {
                match state {
                    (_d, 0, true) => {
                        state.2 = false;
                        state.1 = input[i].2
                    }
                    (d, 0, false) => {
                        *d += input[i].0;
                        state.2 = true;
                        state.1 = input[i].1
                    }
                    (d, _r, true) => {
                        *d += input[i].0;
                    }
                    (_d, _s, false) => (),
                }
                state.1 -= 1;
            }
            let max = states.iter().map(|l| l.0).max().unwrap();
            for (i, state) in states.iter().enumerate() {
                if state.0 == max {
                    scores[i] += 1;
                }
            }
        }
        scores.iter().max().unwrap().to_string()
    }
}
