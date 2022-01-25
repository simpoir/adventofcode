use crate::timed;
use clap::Parser;

/// AdventOfCode runner
#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Args {
    /// Run a single day instead of all.
    pub day: Option<usize>,

    /// Don't run part 2.
    #[clap(short = '1', long)]
    pub part_1: bool,

    /// Don't run part 1.
    #[clap(short = '2', long)]
    pub part_2: bool,

    /// Don't run sample data.
    #[clap(short = 'S', long)]
    pub no_sample: bool,

    /// Don't verify sample data.
    #[clap(short = 'V', long)]
    pub no_verify: bool,
}

impl Args {
    pub fn parse() -> Args {
        Parser::parse()
    }
}

pub trait Day: Default {
    type Input;

    fn gen(&self, data: &str) -> Self::Input;
    fn part1(&self, input: &Self::Input) -> String;
    fn part2(&self, input: &Self::Input) -> String;

    fn run(args: &crate::cli::Args, data: &str, expected: Option<(&str, &str)>) {
        let day = Self::default();
        println!("  Generator");
        let input = timed! {day.gen(data)};
        if !args.part_2 {
            let res;
            timed! {
                res = day.part1(&input);
                println!("{}{}  Part 1: {}", ansi_escapes::CursorTo::AbsoluteX(0), ansi_escapes::EraseLine, res);
            };
            if let Some((e, _)) = expected {
                assert_eq!(e, res, "Differs from expected value of {:?}", e);
            }
        }
        if !args.part_1 {
            let res;
            timed! {
                res = day.part2(&input);
                println!("{}{}  Part 2: {}", ansi_escapes::CursorTo::AbsoluteX(0),ansi_escapes::EraseLine, res);
            };
            if let Some((_, e)) = expected {
                assert_eq!(e, res, "Differs from expected value of {:?}", e);
            }
        }
    }
}
