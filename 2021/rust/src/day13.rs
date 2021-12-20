use std::collections::BTreeSet;

#[derive(Default)]
pub struct Day {}

#[derive(Debug)]
pub enum Fold {
    X(isize),
    Y(isize),
}

impl crate::Day for Day {
    type Input = (Vec<(isize, isize)>, Vec<Fold>);

    fn gen(&self, data: &str) -> Self::Input {
        let (data_grid, data_folds) = data.trim_end().split_once("\n\n").unwrap();
        let grid = data_grid
            .lines()
            .map(|l| {
                let (x, y) = l.split_once(',').unwrap();
                let x: isize = x.parse().unwrap();
                let y: isize = y.parse().unwrap();
                (x, y)
            })
            .collect();
        let folds: Vec<Fold> = data_folds
            .lines()
            .map(|l| {
                if l.starts_with("fold along y") {
                    Fold::Y(l.split_once('=').unwrap().1.parse().unwrap())
                } else {
                    Fold::X(l.split_once('=').unwrap().1.parse().unwrap())
                }
            })
            .collect();

        (grid, folds)
    }

    fn part1(&self, input: &Self::Input) -> String {
        let mut dots = input.0.clone();
        for fold in &input.1[..1] {
            match fold {
                Fold::X(x) => {
                    let x2 = 2 * x;
                    dots = dots
                        .iter()
                        .map(|(i, j)| if i <= x { (*i, *j) } else { (x2 - *i, *j) })
                        .collect();
                }
                Fold::Y(y) => {
                    let y2 = 2 * y;
                    dots = dots
                        .iter()
                        .map(|(i, j)| if j <= y { (*i, *j) } else { (*i, y2 - *j) })
                        .collect();
                }
            }
        }

        let res = BTreeSet::from_iter(dots).len();
        format!("{}", res)
    }

    fn part2(&self, input: &Self::Input) -> String {
        let mut dots = input.0.clone();
        for fold in &input.1 {
            match fold {
                Fold::X(x) => {
                    let x2 = 2 * x;
                    dots = dots
                        .iter()
                        .map(|(i, j)| if i <= x { (*i, *j) } else { (x2 - *i, *j) })
                        .collect();
                }
                Fold::Y(y) => {
                    let y2 = 2 * y;
                    dots = dots
                        .iter()
                        .map(|(i, j)| if j <= y { (*i, *j) } else { (*i, y2 - *j) })
                        .collect();
                }
            }
        }
        pr(&dots)
    }
}

#[allow(dead_code)]
fn pr(dots: &[(isize, isize)]) -> String {
    let mut grid = vec![['.'; 40]; 7];
    for (i, j) in dots {
        grid[*j as usize][*i as usize] = '#';
    }
    let max_x = *dots.iter().map(|(x, _)| x).max().unwrap() as usize;

    let mut res = String::from("\n");
    for l in grid {
        for c in &l[..=max_x] {
            res.push(*c);
        }
        res.push('\n');
    }
    res.pop();
    res
}
