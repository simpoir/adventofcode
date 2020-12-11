use prelude::*;

mod aoc;
pub mod prelude;

fn main() -> Result<()> {
    println!("AOC 2020, rustier edition.");
    aoc::day1::Day::run()?;
    aoc::day2::Day::run()?;
    aoc::day3::Day::run()?;
    aoc::day10::Day::run()?;
    aoc::day11::Day::run()?;
    Ok(())
}
