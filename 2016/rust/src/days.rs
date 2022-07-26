
            type Runnable = Box<dyn Fn(u8, &crate::cli::Args, &str)>;

            use crate::cli::Day;
            mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;


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
Box::new(|d, args, data| day9::Day::run(d, args, data)),
Box::new(|d, args, data| day10::Day::run(d, args, data)),
Box::new(|d, args, data| day11::Day::run(d, args, data)),
Box::new(|d, args, data| day12::Day::run(d, args, data)),
Box::new(|d, args, data| day13::Day::run(d, args, data)),
Box::new(|d, args, data| day14::Day::run(d, args, data)),
Box::new(|d, args, data| day15::Day::run(d, args, data)),
Box::new(|d, args, data| day16::Day::run(d, args, data)),
Box::new(|d, args, data| day17::Day::run(d, args, data)),
Box::new(|d, args, data| day18::Day::run(d, args, data)),
Box::new(|d, args, data| day19::Day::run(d, args, data)),
Box::new(|d, args, data| day20::Day::run(d, args, data)),
Box::new(|d, args, data| day21::Day::run(d, args, data)),
Box::new(|d, args, data| day22::Day::run(d, args, data)),

                ]
            }