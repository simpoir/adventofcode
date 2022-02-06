use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

mod cli;
mod days;
mod util;

fn main() {
    let args = cli::Args::parse();
    let days = days::days();
    let days = days.iter().enumerate();
    let days: Box<dyn Iterator<Item = _>> = if args.last {
        Box::new(days.rev().take(1))
    } else {
        Box::new(days)
    };

    for (d, m) in days {
        let d = d + 1;
        if let Some(day) = args.day {
            if day != d {
                continue;
            }
        }
        let challenge_dir = PathBuf::from("../data");

        let mut input = String::new();
        File::open(challenge_dir.join(format!("day{d}.txt")))
            .expect("opening input")
            .read_to_string(&mut input)
            .unwrap();
        m(d as u8, &args, input.trim_end());
    }
}
