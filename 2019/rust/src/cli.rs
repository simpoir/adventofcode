use crate::timed;
use clap::Parser;
use std::io::Write;

pub type Result<T> = std::result::Result<T, anyhow::Error>;
#[allow(unused)]
pub type Output = Result<String>;

/// AdventOfCode runner
#[derive(Parser, Debug)]
#[clap(about, version, author)]
pub struct Args {
    /// Run a single day instead of all.
    pub day: Option<usize>,

    /// Run just the last day
    #[clap(short = 'l', long)]
    pub last: bool,

    /// Don't run part 2.
    #[clap(short = '1', long)]
    pub part_1: bool,

    /// Don't run part 1.
    #[clap(short = '2', long)]
    pub part_2: bool,
    // This year, there is no sample runner. Sample goes in unit tests.
}

impl Args {
    pub fn parse() -> Args {
        Parser::parse()
    }
}

pub trait Day<'i>: Default
where
    Self::Input: 'i,
{
    type Input;
    /// If part2 requires some side-effect from part1, set this true.
    fn need_part1() -> bool {
        false
    }
    fn gen(&mut self, data: &'i str) -> Result<Self::Input>;
    fn part1(&mut self, input: &Self::Input) -> Result<String>;
    fn part2(&mut self, input: &Self::Input) -> Result<String>;

    fn run(d: u8, args: &crate::cli::Args, data: &'i str) -> Result<()>
    where
        Self: 'i,
        Self::Input: 'i,
    {
        let mut day = Self::default();
        print!("🎁 Day {d:>2}  ");
        print!("⚙ Input  ");
        std::io::stdout().flush()?;
        let (t, input) = timed! { day.gen(data)? };
        printify("", &t);
        if !args.part_2 || Self::need_part1() {
            print!(" 🧩 Part 1: {}", ansi_escapes::CursorSavePosition);
            std::io::stdout().flush()?;
            let (t, res) = timed! { day.part1(&input)? };
            printify(&res, &t);
        }
        if !args.part_1 {
            print!(" 🧩 Part 2: {}", ansi_escapes::CursorSavePosition);
            std::io::stdout().flush()?;
            let (t, res) = timed! { day.part2(&input)? };
            printify(&res, &t);
            std::io::stdout().flush()?;
        }
        println!();
        Ok(())
    }
}

fn printify(res: &str, t: &str) {
    if res.is_empty() {
        print!("\x1b[2m{t}\x1b[0m");
    } else if res.len() <= 12 {
        print!(
            "{}{}{res:>12} \x1b[2m{t}\x1b[0m",
            ansi_escapes::CursorRestorePosition,
            ansi_escapes::EraseEndLine,
        );
    } else {
        print!(
            "{}{}\n{res} \x1b[2m{t}\x1b[0m",
            ansi_escapes::CursorRestorePosition,
            ansi_escapes::EraseEndLine
        );
    }
    std::io::stdout().flush().unwrap();
}
