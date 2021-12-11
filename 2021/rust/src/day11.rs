pub struct Day {}

impl crate::Day for Day {
    type Input = [[u8; 10]; 10];

    fn gen(&self, data: &str) -> Self::Input {
        let mut res = [[0; 10]; 10];
        data.trim_end().lines().enumerate().for_each(|(j, l)| {
            l.bytes()
                .enumerate()
                .for_each(|(i, c)| res[j][i] = c - b'0')
        });
        res
    }

    fn part1(&self, input: &Self::Input) -> String {
        let mut data = *input;
        let flashes: usize = (0..100).map(|_| step(&mut data)).sum();
        format!("{}", flashes)
    }

    fn part2(&self, input: &Self::Input) -> String {
        let mut data = *input;
        let res = (1..).find(|_| step(&mut data) == 100).unwrap();
        format!("{}", res)
    }
}

fn step(data: &mut [[u8; 10]; 10]) -> usize {
    let mut flashed = [[false; 10]; 10];
    let mut queue = vec![];
    let mut flashes = 0;
    for i in 0..10 {
        (0..10).for_each(|j| {
            queue.push((i, j));
        });
    }
    while let Some((i, j)) = queue.pop() {
        if data[j][i] >= 9 && !flashed[j][i] {
            flashed[j][i] = true;
            flashes += 1;
            if j > 0 && i > 0 {
                queue.push((i - 1, j - 1));
            }
            if j > 0 {
                queue.push((i, j - 1));
            }
            if j > 0 && i < 9 {
                queue.push((i + 1, j - 1));
            }
            if i > 0 {
                queue.push((i - 1, j));
            }
            if i < 9 {
                queue.push((i + 1, j));
            }
            if j < 9 && i > 0 {
                queue.push((i - 1, j + 1));
            }
            if j < 9 {
                queue.push((i, j + 1));
            }
            if j < 9 && i < 9 {
                queue.push((i + 1, j + 1));
            }
        }
        data[j][i] += 1;
    }
    for i in 0..10 {
        (0..10).for_each(|j| {
            if flashed[j][i] {
                data[j][i] = 0;
            }
        });
    }
    flashes
}

#[allow(dead_code)]
fn print_block(d: &[[u8; 10]; 10]) {
    for l in d {
        for c in l {
            print!("{}", c);
        }
        println!();
    }
    println!();
}
