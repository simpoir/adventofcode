#[derive(Default)]
pub struct Day {}

impl crate::Day for Day {
    type Input = Vec<Vec<u8>>;

    fn gen(&self, data: &str) -> Self::Input {
        data.trim_end()
            .lines()
            .map(|l| l.bytes().map(|c| c - b'0').collect())
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> String {
        let height = input.len();
        let width = input[0].len();
        let mut input2 = vec![vec![9u8; width + 2]];
        for l in input {
            input2.push([vec![9u8], l.to_vec(), vec![9u8]].concat());
        }
        input2.append(&mut vec![vec![9u8; width + 2]]);
        let input = input2;

        let mut res = 0usize;
        for j in 1..=height {
            for i in 1..=width {
                let is_min = input[j][i] < input[j][i + 1]
                    && input[j][i] < input[j][i - 1]
                    && input[j][i] < input[j + 1][i]
                    && input[j][i] < input[j - 1][i];
                if is_min {
                    res += (input[j][i] + 1) as usize;
                }
            }
        }
        format!("{}", res)
    }
    fn part2(&self, input: &Self::Input) -> String {
        let height = input.len();
        let width = input[0].len();
        let mut input2 = vec![vec![9u8; width + 2]];
        for l in input {
            input2.push([vec![9u8], l.to_vec(), vec![9u8]].concat());
        }
        input2.append(&mut vec![vec![9u8; width + 2]]);

        let mut basin_start = vec![];
        let mut input = input2;
        for j in 1..=height {
            for i in 1..=width {
                let is_min = input[j][i] < input[j][i + 1]
                    && input[j][i] < input[j][i - 1]
                    && input[j][i] < input[j + 1][i]
                    && input[j][i] < input[j - 1][i];
                if is_min {
                    basin_start.push((i, j));
                }
            }
        }

        let mut basins = vec![];
        // for each point check if adjacent are 9.
        // if not add to stack and increment size by 1
        for start in basin_start {
            let mut basin_size = 0;
            let mut to_check = vec![start];
            while !to_check.is_empty() {
                let (i, j) = to_check.pop().unwrap();
                if input[j][i] != 9 {
                    basin_size += 1;
                    input[j][i] = 9;
                    to_check.push((i + 1, j));
                    to_check.push((i - 1, j));
                    to_check.push((i, j + 1));
                    to_check.push((i, j - 1));
                }
            }
            basins.push(basin_size);
        }
        basins.sort_unstable();
        let res: usize = basins.iter().rev().take(3).product();
        format!("{}", res)
    }
}
