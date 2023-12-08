type Runnable = Box<dyn Fn(u8, &crate::cli::Args, &str) -> Result<(), anyhow::Error>>;

use crate::cli::Day;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;


pub fn days() -> Vec<Runnable> {
    vec![
       Box::new(|d, args, data| day1::Day::run(d, args, data)),
       Box::new(|d, args, data| day2::Day::run(d, args, data)),
       Box::new(|d, args, data| day3::Day::run(d, args, data)),
       Box::new(|d, args, data| day4::Day::run(d, args, data)),
       Box::new(|d, args, data| day5::Day::run(d, args, data)),
       Box::new(|d, args, data| day6::Day::run(d, args, data)),
       Box::new(|d, args, data| day7::Day::run(d, args, data)),
       Box::new(|d, args, data| day8::Day::run(d, args, data)),

    ]
}