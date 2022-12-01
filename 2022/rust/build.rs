use std::fmt::Write;
use std::fs;
use std::path::Path;

fn main() {
    let dest_path = Path::new("src/days.rs");
    let src_path = Path::new("src/days");
    let mut uses = String::new();
    let mut closures = String::new();

    for i in 1..=25 {
        println!("cargo:rerun-if-changed=src/days/day{}.rs", i);
        if src_path.join(format!("day{}.rs", i)).exists() {
            writeln!(uses, "mod day{};", i).unwrap();
            writeln!(
                closures,
                "       Box::new(|d, args, data| day{}::Day::run(d, args, data)),",
                i
            )
            .unwrap();
        } else {
            break;
        }
    }

    fs::write(
        dest_path,
        format!(
            "\
type Runnable = Box<dyn Fn(u8, &crate::cli::Args, &str) -> Result<(), anyhow::Error>>;

use crate::cli::Day;
{uses}

pub fn days() -> Vec<Runnable> {{
    vec![
{closures}
    ]
}}"
        ),
    )
    .unwrap();
}
