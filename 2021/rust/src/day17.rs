#[derive(Default)]
pub struct Day {}

impl crate::Day for Day {
    type Input = ((isize, isize), (isize, isize));

    fn gen(&self, data: &str) -> Self::Input {
        let (_, line) = data.trim_end().split_once('=').unwrap();
        let (x, y) = line.split_once(", y=").unwrap();
        let (x0, x1) = x.split_once("..").unwrap();
        let (y0, y1) = y.split_once("..").unwrap();
        (
            (x0.parse().unwrap(), x1.parse().unwrap()),
            (y0.parse().unwrap(), y1.parse().unwrap()),
        )
    }

    fn part1(&self, input: &Self::Input) -> String {
        let mut best = (0, 0);
        let x = ((input.0 .0 * 2) as f32).sqrt() as isize;
        for i in 1..=(input.1 .0.abs()) {
            let res = sim((x, i), input);
            match res {
                Res::Hit => {
                    best = (x, i);
                }
                Res::Under => continue,
                Res::Through => continue,
                Res::Over => break,
            }
        }

        let res = best.1 * (best.1 + 1) / 2;
        format!("{}", res)
    }

    fn part2(&self, input: &Self::Input) -> String {
        let mut count = 0;
        let xmin = ((input.0 .0 * 2) as f32).sqrt() as isize;
        for y in (-input.1 .0.abs())..=(input.1 .0.abs()) {
            for x in xmin..=input.0 .1 {
                let res = sim((x, y), input);
                match res {
                    Res::Hit => {
                        count += 1;
                    }
                    _ => continue,
                }
            }
        }

        format!("{}", count)
    }
}

#[derive(Debug)]
enum Res {
    Over,
    Under,
    Through,
    Hit,
}

fn sim(init: (isize, isize), target: &((isize, isize), (isize, isize))) -> Res {
    let mut vel = init;
    let mut pos = (0, 0);
    let &((x0, x1), (y0, y1)) = target;
    while pos.1 >= y0 {
        if pos.0 >= x0 && pos.0 <= x1 && pos.1 <= y1 {
            return Res::Hit;
        }

        pos.0 += vel.0;
        pos.1 += vel.1;
        vel.1 -= 1;
        if vel.0 > 0 {
            vel.0 -= 1;
        }
    }
    if pos.0 < x0 {
        Res::Under
    } else if pos.0 > x1 {
        Res::Over
    } else {
        Res::Through
    }
}
