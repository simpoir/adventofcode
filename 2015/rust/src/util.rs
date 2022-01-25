#[macro_export]
macro_rules! timed {
    ($($code:tt)+) => {{
        let t0 = std::time::Instant::now();
        let res = { $($code)+ };
        let t1 = std::time::Instant::now();
        let delta = (t1 - t0).as_secs_f32();
        print!("{}{}{}{}{}",
               ansi_escapes::CursorSavePosition,
               ansi_escapes::CursorPrevLine,
               ansi_escapes::CursorTo::AbsoluteX(30),
               match delta {
                   x if x < 0.000_001 => format!("({}ns)", x * 1000_000_000.0),
                   x if x < 0.001 => format!("({}µs)", x * 1000_000.0),
                   x if x < 1.0 => format!("({}ms)", x * 1000.0),
                   x => format!("({} seconds)", x),
               },
               ansi_escapes::CursorRestorePosition);
        res
    }};
}

#[allow(unused)]
pub fn progress<T>(out: &T)
where
    T: std::fmt::Display,
{
    static LAST: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    let mut last = LAST.load(std::sync::atomic::Ordering::Relaxed);
    let now = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    if last + 1 > now {
        return;
    }
    LAST.store(now, std::sync::atomic::Ordering::Relaxed);
    print!(
        "{}{}{}{}",
        ansi_escapes::CursorSavePosition,
        out,
        ansi_escapes::EraseEndLine,
        ansi_escapes::CursorRestorePosition
    );
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
}

#[allow(dead_code)]
pub fn print_grid<'a, R>(grid: &'a [R], n: usize)
where
    &'a R: IntoIterator<Item = &'a bool>,
{
    println!();
    grid.iter().take(n).for_each(|l| {
        println!(
            "{}",
            l.into_iter()
                .map(|c| if *c { '#' } else { '.' })
                .collect::<String>()
        )
    });
}

/// Walk through all combination of things and call f(buf) with each.
/// Breaks the loop if f returns false.
#[allow(unused)]
pub fn combine<'t, T, F>(things: &'t [T], buf: &mut [T], f: &mut F) -> bool
where
    T: Copy + std::fmt::Debug,
    F: FnMut(&[T]) -> bool,
{
    combine_(things, buf, 0, f)
}

fn combine_<'t, T, F>(things: &'t [T], buf: &mut [T], idx: usize, f: &mut F) -> bool
where
    T: Copy + std::fmt::Debug,
    F: FnMut(&[T]) -> bool,
{
    let next = idx + 1;
    for (i, x) in things
        .iter()
        .enumerate()
        .take(things.len() + next + 1 - buf.len())
    {
        buf[idx] = *x;
        let cont = if buf.len() > next {
            combine_(&things[(i + 1)..], buf, next, f)
        } else {
            f(buf)
        };
        if !cont {
            return false;
        }
    }
    true
}
