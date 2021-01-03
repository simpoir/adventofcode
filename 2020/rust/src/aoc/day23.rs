use std::iter::empty;

day! {
    day23;
    type INPUT = Vec<u8>; // data probably too small for LinkedList

    fn gen(file: &mut impl BufRead) -> Result<Self::INPUT> {
        let l = file.lines().next().unwrap().unwrap();
        Ok(l.as_bytes().iter().map(|c| c - b'0').collect())
    }

    fn part1(cups: &Self::INPUT) -> Result<String> {
        // I left this shitty impl for the gist of it, even if the
        // linked thing is faster than splicing tiny arrays.
        let mut cups: Vec<u8> = cups.clone();
        let n_cups = cups.len();

        for _ in 0..100 {
            let current = cups[0];
            let mut picked: Vec<u8> = cups.splice(1..=3, empty()).collect();

            let mut dest = current - 1;
            let insert_point;
            while dest == 0 || picked.contains(&dest)  {
                dest = dest.checked_sub(1).unwrap_or(n_cups as u8);
            }

            insert_point = cups.iter().position(|x| *x == dest).unwrap() + 1;
            cups.splice(insert_point..insert_point, picked.drain(..));
            cups.rotate_left(1);
        }

        let it = cups.iter().cycle();
        let res: String = it.skip_while(|x| **x != 1)
            .skip(1)
            .take(n_cups-1)
            .map(|x| x.to_string())
            .collect();
        Ok(res)
    }

    fn part2(input: &Self::INPUT) -> Result<String> {
        let mut res = vec![0usize; 1000001];  // yeah, arrays would stack-overflow
        play(input, 10000000, &mut res);
        let first = res[1];
        Ok(format!("{:?}", first * res[first]))
    }
}

/// Play.
/// `cups` is just a buffer of size N+1. It'll contain the end configuration.
/// It's an indexed linked list in a stack-allocated slice. Should be fast.
fn play(input: &Vec<u8>, rounds: usize, cups: &mut [usize]) {
    let n_cups = cups.len();
    let mut it = input
        .iter()
        .map(|x| *x as usize)
        .chain((input.len() + 1)..n_cups);
    let first = it.next().expect("not empty input");
    let mut prev = first;
    for cup in it {
        cups[prev] = cup;
        prev = cup;
    }
    cups[prev] = first;

    let mut current = first;
    for _ in 0..rounds {
        // unrolled extraction
        let picked1 = cups[current];
        let picked2 = cups[picked1];
        let picked3 = cups[picked2];
        cups[current] = cups[picked3];

        let mut dest = current - 1;
        loop {
            if dest == 0 {
                dest = n_cups - 1;
            }
            if dest != picked3 && dest != picked2 && dest != picked1 {
                break;
            }
            dest = dest - 1;
        }

        cups[picked3] = cups[dest];
        cups[dest] = picked1;
        current = cups[current];
    }
}
