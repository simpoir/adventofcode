#[derive(Default)]
pub struct Day {}

impl crate::cli::Day for Day {
    type Input = ();

    fn gen(&self, _data: &str) -> Self::Input {}

    /// Not solving this one generically. If you want to see a VM, check 2021.
    fn part1(&self, _input: &Self::Input) -> String {
        // cpy 1 a     a = 1
        // cpy 1 b     b = 2
        // cpy 26 d    d = 26
        // jnz c 2     if c != 0 {   // noop
        // jnz 1 5
        // cpy 7 c       c = 7
        // inc d         do { d ++
        // dec c              c --
        // jnz c -2      } while c != 0}
        // cpy a c     do { c = a
        // inc a            do { a ++
        // dec b                 b --
        // jnz b -2         } while b =! 0
        // cpy c b          b = c
        // dec d            d --
        // jnz d -6    } while d != 0  // fancy fibonnaci d (26) times
        // cpy 17 c    c = 17
        // cpy 18 d    { d = 18
        // inc a         do { a++
        // dec d              d--
        // jnz d -2      } while d != 0
        // dec c         c--
        // jnz c -5    } while c != 0  // 18 * 17
        (fib(1, 2, 26) + 17 * 18).to_string()
    }

    fn part2(&self, _input: &Self::Input) -> String {
        (fib(1, 2, 26 + 7) + 17 * 18).to_string()
    }
}
fn fib(a: usize, b: usize, n: usize) -> usize {
    if n == 0 {
        return a;
    }
    fib(b, a + b, n - 1)
}
