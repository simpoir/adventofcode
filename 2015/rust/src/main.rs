use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

mod cli;
mod days;
mod util;

fn main() {
    let args = cli::Args::parse();
    let days = days::days();

    for (d, m) in days.iter().enumerate() {
        let d = d + 1;
        if let Some(day) = args.day {
            if day != d {
                continue;
            }
        }
        println!("\nDay {}:", d);
        let challenge_dir = PathBuf::from(&format!("../data/day{}", d));

        if !args.no_sample {
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
            let expected = if args.no_verify {
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
            m(&args, sample.trim_end(), expected);
        }

        let mut input = String::new();
        File::open(challenge_dir.join("input"))
            .expect("opening input")
            .read_to_string(&mut input)
            .unwrap();
        println!(" Challenge:");
        m(&args, input.trim_end(), None);
    }
}
