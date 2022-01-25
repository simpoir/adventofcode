#[derive(Default)]
pub struct Day {}

impl crate::cli::Day for Day {
    type Input = Vec<String>;

    fn gen(&self, data: &str) -> Self::Input {
        data.lines().map(|l| l.to_string()).collect()
    }

    fn part1(&self, input: &Self::Input) -> String {
        input
            .iter()
            .map(|l| l.len() - parse(l))
            .sum::<usize>()
            .to_string()
    }

    fn part2(&self, input: &Self::Input) -> String {
        input
            .iter()
            .map(|l| unparse(l) - l.len())
            .sum::<usize>()
            .to_string()
    }
}

fn parse(line: &str) -> usize {
    let mut chars = line.chars().peekable();
    let mut count = 0;
    while let Some(c) = chars.next() {
        if c == '\\' {
            match chars.peek() {
                Some('\\' | '"') => {
                    chars.next();
                }
                Some('x') => {
                    chars.next();
                    chars.next();
                    chars.next();
                }
                _ => (),
            }
        }
        count += 1;
    }
    count - 2 // quotes
}

fn unparse(line: &str) -> usize {
    let mut count = 0;
    for c in line.chars() {
        if let '\\' | '"' = c {
            count += 1;
        }
        count += 1;
    }
    count + 2 // quotes
}
