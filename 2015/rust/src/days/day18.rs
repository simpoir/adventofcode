#[derive(Default)]
pub struct Day {}

const SIZ: usize = 102;

impl crate::cli::Day for Day {
    type Input = [[bool; SIZ]; SIZ];

    fn gen(&self, data: &str) -> Self::Input {
        let mut res = [[false; SIZ]; SIZ];
        for (y, l) in data.lines().enumerate() {
            for (x, c) in l.chars().enumerate() {
                res[y + 1][x + 1] = c == '#';
            }
        }
        res
    }

    fn part1(&self, grid: &Self::Input) -> String {
        let mut grid = *grid;
        for _ in 0..100 {
            grid = step(grid);
        }
        grid.iter().flatten().filter(|x| **x).count().to_string()
    }

    fn part2(&self, grid: &Self::Input) -> String {
        let mut grid = *grid;
        for _ in 0..100 {
            corner(&mut grid);
            grid = step(grid);
        }
        corner(&mut grid);
        grid.iter().flatten().filter(|x| **x).count().to_string()
    }
}

fn corner(grid: &mut [[bool; SIZ]; SIZ]) {
    grid[1][1] = true;
    grid[1][SIZ - 2] = true;
    grid[SIZ - 2][1] = true;
    grid[SIZ - 2][SIZ - 2] = true;
}

fn step(start: [[bool; SIZ]; SIZ]) -> [[bool; SIZ]; SIZ] {
    let mut end = [[false; SIZ]; SIZ];
    for x in 1..=(SIZ - 2) {
        for y in 1..=(SIZ - 2) {
            let tot = start[x - 1][y - 1] as u8
                + start[x][y - 1] as u8
                + start[x + 1][y - 1] as u8
                + start[x - 1][y] as u8
                + start[x + 1][y] as u8
                + start[x - 1][y + 1] as u8
                + start[x][y + 1] as u8
                + start[x + 1][y + 1] as u8;
            end[x][y] = tot == 3 || (start[x][y] && tot == 2)
        }
    }

    end
}
