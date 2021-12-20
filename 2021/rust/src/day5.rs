use std::{cmp::Ordering, iter::repeat};

#[derive(Default)]
pub struct Day {}

impl crate::Day for Day {
    type Input = Vec<[usize; 4]>;

    fn gen(&self, data: &str) -> Self::Input {
        data.lines()
            .map(|l| {
                let mut line = [0; 4];
                l.split(&[',', ' ', '-', '>'][..])
                    .filter(|x| !x.is_empty())
                    .enumerate()
                    .for_each(|(i, n)| line[i] = n.parse().unwrap());
                line
            })
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> String {
        let mut grid = [[0; 1000]; 1000];
        for line in input {
            if line[0] == line[2] {
                let x = line[0];
                if line[1] < line[3] {
                    line[1]..=line[3]
                } else {
                    line[3]..=line[1]
                }
                .for_each(|y| grid[y][x] += 1)
            } else if line[1] == line[3] {
                let y = line[1];
                if line[0] < line[2] {
                    line[0]..=line[2]
                } else {
                    line[2]..=line[0]
                }
                .for_each(|x| grid[y][x] += 1)
            }
        }
        let counts = grid.iter().flatten().filter(|x| **x >= 2).count();
        format!("{}", counts)
    }

    fn part2(&self, input: &Self::Input) -> String {
        let mut grid = [[0; 1000]; 1000];
        for line in input {
            let xrange: Box<dyn Iterator<Item = usize>> = match line[0].cmp(&line[2]) {
                Ordering::Less => Box::new(line[0]..=line[2]),
                Ordering::Greater => Box::new((line[2]..=line[0]).rev()),
                Ordering::Equal => Box::new(repeat(line[0])),
            };
            let yrange: Box<dyn Iterator<Item = usize>> = match line[1].cmp(&line[3]) {
                Ordering::Less => Box::new(line[1]..=line[3]),
                Ordering::Greater => Box::new((line[3]..=line[1]).rev()),
                Ordering::Equal => Box::new(repeat(line[1])),
            };
            xrange.zip(yrange).for_each(|(x, y)| grid[y][x] += 1)
        }
        let counts = grid.iter().flatten().filter(|x| **x >= 2).count();
        format!("{}", counts)
    }
}
