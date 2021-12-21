use std::collections::HashMap;

#[derive(Default)]
pub struct Day {}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Player {
    pos: usize,
    score: usize,
}

impl crate::Day for Day {
    type Input = Vec<usize>;

    fn gen(&self, data: &str) -> Self::Input {
        data.trim_end()
            .lines()
            .map(|l| l.rsplit_once(' ').unwrap().1.parse().unwrap())
            .collect()
    }

    fn part1(&self, input: &Self::Input) -> String {
        let mut players: Vec<Player> = input.iter().map(|p| Player { pos: *p, score: 0 }).collect();
        let mut dice = (1..=100).cycle().enumerate();

        for p in (0..2).cycle() {
            let Player {
                ref mut pos,
                ref mut score,
            } = &mut players[p];
            *pos += dice.next().unwrap().1;
            *pos += dice.next().unwrap().1;
            *pos += dice.next().unwrap().1;
            *pos = 1 + (*pos - 1) % 10;
            *score += *pos;
            if *score >= 1000 {
                break;
            }
        }
        let res = players.iter().map(|p| p.score).min().unwrap() * dice.next().unwrap().0;
        format!("{}", res)
    }

    fn part2(&self, input: &Self::Input) -> String {
        let players: [Player; 2] = input
            .iter()
            .map(|p| Player { pos: *p, score: 0 })
            .collect::<Vec<Player>>()
            .try_into()
            .unwrap();
        let turn = false;
        let (win_a, win_b) = play(players, turn, 0, &mut HashMap::new());

        let res = win_a.max(win_b);
        format!("{}", res)
    }
}

const MAX_SCORE: usize = 21;

fn play(
    players: [Player; 2],
    p: bool,
    dice: usize,
    cache: &mut HashMap<([Player; 2], bool), (u64, u64)>,
) -> (u64, u64) {
    let turn: usize = p.into();
    let mut wins = (0, 0);

    if dice == 0 {
        if let Some(v) = cache.get(&(players.clone(), p)) {
            return *v;
        }
    }

    let (next_p, next_dice) = if dice == 2 { (!p, 0) } else { (p, dice + 1) };
    for d in 0..3 {
        let mut players = players.clone();
        players[turn].pos = 1 + (players[turn].pos + d) % 10;
        if dice == 2 {
            players[turn].score += players[turn].pos;
            if players[turn].score >= MAX_SCORE {
                if p {
                    wins.1 += 1;
                } else {
                    wins.0 += 1;
                }
            }
        }
        if players[turn].score < MAX_SCORE {
            let (a, b) = play(players, next_p, next_dice, cache);
            wins.0 += a;
            wins.1 += b;
        }
    }

    if dice == 0 {
        cache.insert((players, p), wins);
    }

    wins
}
