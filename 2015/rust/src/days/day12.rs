#[derive(Default)]
pub struct Day {}

impl crate::cli::Day for Day {
    type Input = String;

    fn gen(&self, data: &str) -> Self::Input {
        data.to_string()
    }

    fn part1(&self, input: &Self::Input) -> String {
        let mut total: i64 = 0;
        let mut acc: i64 = 0;
        let mut sub = false;
        input.chars().for_each(|c| match c {
            '0'..='9' => acc = acc * 10 + (c as u8 - b'0') as i64,
            '-' => sub = true,
            _ => {
                if sub {
                    total -= acc;
                } else {
                    total += acc;
                }
                sub = false;
                acc = 0;
            }
        });
        total.to_string()
    }

    fn part2(&self, input: &Self::Input) -> String {
        let parsed: serde_json::Value = serde_json::from_str(input).unwrap();
        walk(&parsed).to_string()
    }
}

fn walk(parsed: &serde_json::Value) -> i64 {
    let marker: serde_json::Value = serde_json::Value::String("red".into());

    match parsed {
        serde_json::Value::Number(n) => n.as_i64().unwrap(),
        serde_json::Value::Array(a) => a.iter().map(walk).sum(),
        serde_json::Value::Object(o) => {
            let mut red = false;
            let total = o
                .iter()
                .map(|item| {
                    red |= item.1 == &marker;
                    walk(item.1)
                })
                .sum();
            if !red {
                total
            } else {
                0
            }
        }
        _ => 0,
    }
}
