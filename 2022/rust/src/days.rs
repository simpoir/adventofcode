type Runnable = Box<dyn Fn(u8, &crate::cli::Args, &str) -> Result<(), anyhow::Error>>;

use crate::cli::Day;
mod day1;


pub fn days() -> Vec<Runnable> {
    vec![
       Box::new(|d, args, data| day1::Day::run(d, args, data)),

    ]
}