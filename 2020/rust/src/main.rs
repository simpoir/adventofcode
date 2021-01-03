use prelude::*;

#[macro_use]
pub mod prelude;
mod aoc;

fn main() -> Result<()> {
    println!("AOC 2020, rustier edition.");
    aoc::day01::Day::run()?;
    aoc::day02::Day::run()?;
    aoc::day03::Day::run()?;
    aoc::day10::Day::run()?;
    aoc::day11::Day::run()?;
    aoc::day12::Day::run()?;
    aoc::day13::Day::run()?;
    aoc::day14::Day::run()?;
    aoc::day15::Day::run()?;
    aoc::day16::Day::run()?;
    aoc::day17::Day::run()?;
    aoc::day18::Day::run()?;
    aoc::day19::Day::run()?;
    aoc::day20::Day::run()?;
    aoc::day21::Day::run()?;
    aoc::day22::Day::run()?;
    Ok(())
}
