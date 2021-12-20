#[derive(Default)]
pub struct Day {}

fn score(c: char) -> usize {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        _ => 25137,
    }
}

fn score2(c: char) -> usize {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        _ => 4,
    }
}

impl crate::Day for Day {
    type Input = Vec<String>;

    fn gen(&self, data: &str) -> Self::Input {
        data.trim_end().lines().map(|s| s.into()).collect()
    }

    fn part1(&self, input: &Self::Input) -> String {
        let mut res = 0;
        for line in input {
            let mut stack: Vec<char> = vec![];
            for c in line.chars() {
                match c {
                    '(' => stack.push(')'),
                    '[' => stack.push(']'),
                    '{' => stack.push('}'),
                    '<' => stack.push('>'),
                    c => {
                        let expected = stack.pop().unwrap();
                        if c != expected {
                            res += score(c);
                        }
                    }
                }
            }
        }
        format!("{}", res)
    }
    fn part2(&self, input: &Self::Input) -> String {
        let mut scores: Vec<usize> = vec![];
        'lines: for line in input {
            let mut stack: Vec<char> = vec![];
            for c in line.chars() {
                match c {
                    '(' => stack.push(')'),
                    '[' => stack.push(']'),
                    '{' => stack.push('}'),
                    '<' => stack.push('>'),
                    c => {
                        let expected = stack.pop().unwrap();
                        if c != expected {
                            continue 'lines;
                        }
                    }
                }
            }
            scores.push(stack.iter().rev().fold(0, |t, x| t * 5 + score2(*x)));
        }

        scores.sort_unstable();
        let res = scores[scores.len() / 2];
        format!("{}", res)
    }
}
