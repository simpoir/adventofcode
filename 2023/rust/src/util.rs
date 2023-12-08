pub static AROUND: [[isize; 2]; 8] = [
    [-1, -1],
    [-1, 0],
    [-1, 1],
    [0, -1],
    [0, 1],
    [1, -1],
    [1, 0],
    [1, 1],
];

#[macro_export]
macro_rules! timed {
    ($($code:tt)+) => {{
        let t0 = std::time::Instant::now();
        let res = { $($code)+ };
        let t1 = std::time::Instant::now();
        let delta = (t1 - t0).as_secs_f32();
        (
            format!("{}",
                    match delta {
                        x if x < 0.000_001 => format!("({:>5.1}ns)", x * 1000_000_000.0),
                        x if x < 0.001 => format!("({:>5.1}µs)", x * 1000_000.0),
                        x if x < 1.0 => format!("({:>5.1}ms)", x * 1000.0),
                        x => format!("({:>5.1}s )", x),
                    }),
            res
        )
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

#[allow(unused)]
pub fn progressd<T>(out: &T)
where
    T: std::fmt::Debug,
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
        "{}{:?}{}{}",
        ansi_escapes::CursorSavePosition,
        out,
        ansi_escapes::EraseEndLine,
        ansi_escapes::CursorRestorePosition
    );
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
}

#[allow(unused)]
pub fn pause() {
    let mut _buf = String::new();
    std::io::stdin().read_line(&mut _buf).unwrap();
}

/// plus petit commun multiple
pub fn ppcm(a: isize, b: isize) -> isize {
    a * b / pgcd(a.max(b), a.min(b))
}

/// plus grand commun diviseur
fn pgcd(a: isize, b: isize) -> isize {
    let r = a % b;
    if r == 0 {
        b
    } else {
        pgcd(b, r)
    }
}

#[allow(unused)]
pub const DIRS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, -1), (0, 1)];

#[allow(dead_code)]
pub fn print_grid<'a, G, R>(grid: G, n: usize, x: usize, y: usize)
where
    G: IntoIterator<Item = R>,
    R: IntoIterator<Item = &'a bool>,
{
    println!();
    grid.into_iter().skip(y).take(n).for_each(|l| {
        println!(
            "{}",
            l.into_iter()
                .skip(x)
                .take(n)
                .map(|c| if *c { '#' } else { '.' })
                .collect::<String>()
        )
    });
}

/// Walk through all subsets of the things set and call f(buf) with each.
/// Breaks the loop if f returns false.
#[allow(unused)]
pub fn subsets<T, F>(things: &[T], buf: &mut [T], f: &mut F) -> bool
where
    T: Copy + std::fmt::Debug,
    F: FnMut(&[T]) -> bool,
{
    subsets_(things, buf, 0, f)
}

fn subsets_<T, F>(things: &[T], buf: &mut [T], idx: usize, f: &mut F) -> bool
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
            subsets_(&things[(i + 1)..], buf, next, f)
        } else {
            f(buf)
        };
        if !cont {
            return false;
        }
    }
    true
}

#[allow(unused)]
/// Try all permutations of things. Return false to break early.
pub fn permutations<T, F>(things: &[T], f: &mut F)
where
    T: Copy + std::fmt::Debug,
    F: FnMut(&[T]) -> bool,
{
    let mut buf = things.to_vec();
    if f(&buf) {
        permutations_(&mut buf, things.len() - 1, f);
    }
}

fn permutations_<T, F>(buf: &mut [T], pos: usize, f: &mut F) -> bool
where
    T: Copy + std::fmt::Debug,
    F: FnMut(&[T]) -> bool,
{
    if pos > 1 && !permutations_(buf, pos - 1, f) {
        return false;
    }
    for i in 0..pos {
        buf.swap(pos, i);
        if !f(buf) {
            return false;
        }
        if pos > 1 && !permutations_(buf, pos - 1, f) {
            return false;
        }
        buf.swap(pos, i);
    }
    true
}

#[cfg(test)]
mod test {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn test_permutations() {
        let input = [1, 2, 3];
        let mut res: BTreeSet<Vec<usize>> = BTreeSet::new();
        permutations(&input, &mut |perm| {
            res.insert(perm.to_vec());
            true
        });
        let expected = BTreeSet::from([
            vec![1, 2, 3],
            vec![2, 1, 3],
            vec![3, 2, 1],
            vec![2, 3, 1],
            vec![1, 3, 2],
            vec![3, 1, 2],
        ]);
        assert_eq!(expected, res);

        res.clear();
        permutations(&[1, 2, 3, 4], &mut |perm| {
            println!("{perm:?}");
            assert!(res.insert(perm.to_vec()));
            true
        });
        assert_eq!(24, res.len());
    }
}
