use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::time::Instant;

macro_rules! timed {
    ($code:block) => {{
        let t0 = Instant::now();
        let res = $code;
        let t1 = Instant::now();
        let delta = (t1 - t0).as_secs_f32();
        print!("\x1B[1000D\x1B[30C"); // right align
        match delta {
            x if x < 0.000_001 => println!("  ({}ns)", x * 1000_000_000.0),
            x if x < 0.001 => println!("  ({}µs)", x * 1000_000.0),
            x if x < 1.0 => println!("  ({}ms)", x * 1000.0),
            x => println!("  ({} seconds)", x),
        }
        res
    }};
}

pub trait Day {
    type Input;

    fn gen(&self, data: &str) -> Self::Input;
    fn part1(&self, input: &Self::Input) -> String;
    fn part2(&self, input: &Self::Input) -> String;

    fn run(&self, data: &str, expected: Option<(&str, &str)>) {
        print!("  Generator");
        let input = timed! {{self.gen(data)}};
        let mut res;
        timed! {{
            res = self.part1(&input);
            print!("  Part 1: {}", res);
        }};
        if let Some((e, _)) = expected {
            assert_eq!(e, res);
        }
        timed! {{
            res = self.part2(&input);
            print!("  Part 2: {}", res);
        }};
        if let Some((_, e)) = expected {
            assert_eq!(e, res);
        }
    }
}

aoc::make_registry!();

fn main() {
    DAYS.with(|days| {
        for (d, m) in days {
            println!("\n{}:", d);
            let challenge_dir = PathBuf::from(&format!("../data/{}", d));

            let mut sample = String::new();
            File::open(challenge_dir.join("sample"))
                .expect("opening sample")
                .read_to_string(&mut sample)
                .unwrap();
            let mut expected = String::new();
            File::open(challenge_dir.join("expected"))
                .expect("opening sample expected result")
                .read_to_string(&mut expected)
                .unwrap();
            let expected = expected
                .trim()
                .split_once('\n')
                .unwrap_or((expected.trim(), ""));
            println!(" Sample:");
            m(sample, Some(expected));

            let mut input = String::new();
            File::open(challenge_dir.join("input"))
                .expect("opening input")
                .read_to_string(&mut input)
                .unwrap();
            println!(" Challenge:");
            m(input, None);
        }
    });
}
