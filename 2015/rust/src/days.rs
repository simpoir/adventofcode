
            type Runnable = Box<dyn Fn(&crate::cli::Args, &str, Option<(&str, &str)>)>;

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
mod day23;
mod day24;
mod day25;


            pub fn days() -> Vec<Runnable> {
                vec![
                    Box::new(|args, data, expected| day1::Day::run(args, data, expected)),
Box::new(|args, data, expected| day2::Day::run(args, data, expected)),
Box::new(|args, data, expected| day3::Day::run(args, data, expected)),
Box::new(|args, data, expected| day4::Day::run(args, data, expected)),
Box::new(|args, data, expected| day5::Day::run(args, data, expected)),
Box::new(|args, data, expected| day6::Day::run(args, data, expected)),
Box::new(|args, data, expected| day7::Day::run(args, data, expected)),
Box::new(|args, data, expected| day8::Day::run(args, data, expected)),
Box::new(|args, data, expected| day9::Day::run(args, data, expected)),
Box::new(|args, data, expected| day10::Day::run(args, data, expected)),
Box::new(|args, data, expected| day11::Day::run(args, data, expected)),
Box::new(|args, data, expected| day12::Day::run(args, data, expected)),
Box::new(|args, data, expected| day13::Day::run(args, data, expected)),
Box::new(|args, data, expected| day14::Day::run(args, data, expected)),
Box::new(|args, data, expected| day15::Day::run(args, data, expected)),
Box::new(|args, data, expected| day16::Day::run(args, data, expected)),
Box::new(|args, data, expected| day17::Day::run(args, data, expected)),
Box::new(|args, data, expected| day18::Day::run(args, data, expected)),
Box::new(|args, data, expected| day19::Day::run(args, data, expected)),
Box::new(|args, data, expected| day20::Day::run(args, data, expected)),
Box::new(|args, data, expected| day21::Day::run(args, data, expected)),
Box::new(|args, data, expected| day22::Day::run(args, data, expected)),
Box::new(|args, data, expected| day23::Day::run(args, data, expected)),
Box::new(|args, data, expected| day24::Day::run(args, data, expected)),
Box::new(|args, data, expected| day25::Day::run(args, data, expected)),

                ]
            }