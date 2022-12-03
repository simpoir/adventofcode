use std::sync::mpsc::channel;

use crate::cli::Result;
use crate::days::day5::run;

#[derive(Default)]
pub struct Day {}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<isize>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        Ok(data.split(',').map(|c| c.parse().unwrap()).collect())
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let (cpu_out, out) = channel();
        let (entry, cpu_in) = channel();

        std::thread::scope(|s| {
            s.spawn(|| {
                run(input, cpu_in, cpu_out);
            });
        });

        // manually crafted
        let precmd = "\
east
east
take semiconductor
north
north
take antenna
south
west
take food ration
west
west
take monolith
east
east
east
south
east
south
south
south
east
east
";

        precmd.bytes().for_each(|c| entry.send(c.into()).unwrap());

        std::thread::spawn(move || {
            for line in std::io::stdin().lines() {
                for c in line.unwrap().bytes() {
                    entry.send(c.into()).unwrap();
                }
                entry.send(10).unwrap();
            }
        });

        while let Ok(val) = out.recv() {
            print!("{}", char::from_u32(val.try_into()?).unwrap())
        }

        Ok("".to_string())
    }

    fn part2(&mut self, _input: &Self::Input) -> Result<String> {
        Ok("⭐".to_string())
    }
}
