use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};

use clap::Arg;

pub static TIMINGS: AtomicBool = AtomicBool::new(true);
static BENCH: AtomicBool = AtomicBool::new(false);
static PART1: AtomicBool = AtomicBool::new(true);
static PART2: AtomicBool = AtomicBool::new(true);

#[macro_export]
macro_rules! timed {
    ($($code:tt)+) => {{
        let t0 = std::time::Instant::now();
        let res = { $($code)+ };
        let t1 = std::time::Instant::now();
        if crate::TIMINGS.load(std::sync::atomic::Ordering::Relaxed) {
            let delta = (t1 - t0).as_secs_f32();
            print!("{}{}{}{}{}",
                   ansi_escapes::CursorSavePosition,
                   ansi_escapes::CursorPrevLine,
                   ansi_escapes::CursorTo::AbsoluteX(30),
                   match delta {
                       x if x < 0.000_001 => format!("({}ns)", x * 1000_000_000.0),
                       x if x < 0.001 => format!("({}µs)", x * 1000_000.0),
                       x if x < 1.0 => format!("({}ms)", x * 1000.0),
                       x => format!("({} seconds)", x),
                   },
                   ansi_escapes::CursorRestorePosition);
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
        println!("  Generator");
        let input = timed! {self.gen(data)};
        let mut res = String::new();
        let bench = if BENCH.load(Ordering::Relaxed) {
            10000
        } else {
            1
        };
        if PART1.load(Ordering::Relaxed) {
            timed! {
                for _ in 0..bench {
                    res = self.part1(&input);
                }
                println!("  Part 1: {}", res);
            };
            if let Some((e, _)) = expected {
                assert_eq!(e, res, "Differs from expected value of {:?}", e);
            }
        }
        if PART2.load(Ordering::Relaxed) {
            timed! {
                for _ in 0..bench {
                    res = self.part2(&input);
                }
                println!("  Part 2: {}", res);
            };
            if let Some((_, e)) = expected {
                assert_eq!(e, res, "Differs from expected value of {:?}", e);
            }
        }
    }
}

aoc::make_registry!();

fn main() {
    let args = clap::App::new("Advent of code")
        .version("2021")
        .arg(Arg::with_name("day"))
        .arg(Arg::with_name("part1").short("1").help("don't run part 2"))
        .arg(Arg::with_name("part2").short("2").help("don't run part 1"))
        .arg(Arg::with_name("bench").long("bench").takes_value(false))
        .arg(
            Arg::with_name("no-test")
                .short("T")
                .help("Don't test expected values")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("no-timing")
                .short("t")
                .help("Don't show timings")
                .takes_value(false),
        )
        .get_matches();

    TIMINGS.store(!args.is_present("no-timing"), Ordering::Relaxed);
    PART1.store(!args.is_present("part2"), Ordering::Relaxed);
    PART2.store(!args.is_present("part1"), Ordering::Relaxed);
    BENCH.store(args.is_present("bench"), Ordering::Relaxed);

    DAYS.with(|days| {
        for (d, m) in days {
            if let Some(day_name) = args.value_of("day") {
                if &day_name != d {
                    continue;
                }
            }
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
            let expected = if args.is_present("no-test") {
                None
            } else {
                Some(
                    expected
                        .trim()
                        .split_once('\n')
                        .unwrap_or((expected.trim(), "")),
                )
            };
            println!(" Sample:");
            m(sample, expected);

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
