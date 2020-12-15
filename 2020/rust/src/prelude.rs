use std::fs::File;
use std::io::Read;
use std::time::Instant;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub trait Challenge {
    type INPUT;
    const NAME: &'static str;

    fn gen(file: &mut impl Read) -> Result<Self::INPUT>;
    fn part1(input: &Self::INPUT) -> Result<String>;
    fn part2(input: &Self::INPUT) -> Result<String>;

    fn run() -> Result<()> {
        println!("\n{}:", Self::NAME);
        let t0 = Instant::now();
        let data = Self::gen(&mut File::open(format!("../data/{}.txt", Self::NAME))?)?;
        let t1 = Instant::now();
        println!("\tpart 1: {}", Self::part1(&data)?);
        let t2 = Instant::now();
        println!("\tpart 2: {}", Self::part2(&data)?);
        let t3 = Instant::now();
        println!(
            "\ttiming (µsec) gen: {}   part1: {}   part2: {}",
            (t1 - t0).as_micros(),
            (t2 - t1).as_micros(),
            (t3 - t2).as_micros()
        );
        Ok(())
    }

    fn test() {
        use std::io::{BufRead, BufReader};
        let mut f = BufReader::new(File::open(format!("../data/{}.test", Self::NAME)).unwrap());
        let test = {
            let mut line = String::new();
            f.read_line(&mut line).unwrap();
            json::parse(&line).unwrap()
        };
        let data = Self::gen(&mut f).unwrap();
        assert_eq!(
            test["expected"][0].as_str().unwrap(),
            Self::part1(&data).unwrap(),
            "Part 1 failed."
        );
        assert_eq!(
            test["expected"][1].as_str().unwrap(),
            Self::part2(&data).unwrap(),
            "Part 2 failed."
        );
    }
}

macro_rules! day {
    ($i:ident;$($v:tt)*) => {
        const NAME: &'static str = stringify!($i);
        day! {
            $($v)*
        }
    };
    ($($v:tt)*) => {
        use crate::prelude::*;
        use std::io::Read;

        pub struct Day {}

        #[test]
        fn test() {
            Day::test();
        }

        impl Challenge for Day {
            const NAME: &'static str = NAME;
            $($v)*
        }
    };
}

macro_rules! day_mod {
    {$i:ident; $($body:tt)*} => {
        pub mod $i {
            const NAME: &'static str = stringify!($i);
            $($body)*
        }
    }
}
