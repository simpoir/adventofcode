use std::collections::HashSet;
use std::collections::VecDeque;

day! {
    day22;
    type INPUT = (Vec<u8>, Vec<u8>);

    fn gen(file: &mut impl BufRead) -> Result<Self::INPUT> {
        let mut it = file.lines();
        it.next();
        let mut mine = vec![];
        let mut his = vec![];
        loop {
            let l = it.next().unwrap().unwrap();
            if l.is_empty() {
                break
            }
            mine.push(l.parse().unwrap());
        }
        it.next();
        for l in it {
            let l = l.unwrap();
            his.push(l.parse().unwrap());
        }
        Ok((mine, his))
    }

    fn part1(input: &Self::INPUT) -> Result<String> {
        let mut mine: VecDeque<u8> = input.0.iter().copied().collect();
        let mut his: VecDeque<u8> = input.1.iter().copied().collect();
        let winner = play(&mut mine, &mut his);
        Ok(format!("{}", score(winner)))
    }

    fn part2(input: &Self::INPUT) -> Result<String> {
        let mut mine: VecDeque<u8> = input.0.iter().copied().collect();
        let mut his: VecDeque<u8> = input.1.iter().copied().collect();
        let winner = match play_rec(&mut mine, &mut his) {
            Player::First => &mine,
            Player::Second => &his,
        };

        Ok(format!("{}", score(winner)))
    }
}

fn play<'a>(first: &'a mut VecDeque<u8>, second: &'a mut VecDeque<u8>) -> &'a VecDeque<u8> {
    loop {
        let a = if let Some(x) = first.pop_front() {
            x
        } else {
            return second;
        };
        let b = if let Some(x) = second.pop_front() {
            x
        } else {
            first.push_front(a);
            return first;
        };
        if a > b {
            first.push_back(a);
            first.push_back(b);
        } else {
            second.push_back(b);
            second.push_back(a);
        }
    }
}

enum Player {
    First,
    Second,
}

fn play_rec(first: &mut VecDeque<u8>, second: &mut VecDeque<u8>) -> Player {
    let mut breaker = HashSet::new();
    loop {
        if !breaker.insert((first.clone(), second.clone())) {
            return Player::First;
        }

        let a = if let Some(x) = first.pop_front() {
            x
        } else {
            return Player::Second;
        };
        let b = if let Some(x) = second.pop_front() {
            x
        } else {
            first.push_front(a);
            return Player::First;
        };

        if first.len() >= a as usize && second.len() >= b as usize {
            // yay! recursion
            let mut sub_first: VecDeque<u8> = first.iter().take(a as usize).copied().collect();
            let mut sub_second: VecDeque<u8> = second.iter().take(b as usize).copied().collect();
            match play_rec(&mut sub_first, &mut sub_second) {
                Player::First => {
                    first.push_back(a);
                    first.push_back(b);
                }
                Player::Second => {
                    second.push_back(b);
                    second.push_back(a);
                }
            }
        } else {
            if a > b {
                first.push_back(a);
                first.push_back(b);
            } else {
                second.push_back(b);
                second.push_back(a);
            }
        }
    }
}

fn score(deck: &VecDeque<u8>) -> u64 {
    let mut score = 0;
    for (i, card) in (1..).zip(deck.iter().rev()) {
        score += (*card as u64) * (i as u64);
    }
    score
}
