use std::cell::Cell;

#[derive(Default)]
pub struct Day {}

#[derive(Clone)]
pub struct Stats {
    hp: u16,
    atk: u16,
}

impl crate::cli::Day for Day {
    type Input = Stats;

    fn gen(&self, data: &str) -> Self::Input {
        let mut lines = data.lines();
        let hp = lines
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .parse()
            .unwrap();
        let atk = lines
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .parse()
            .unwrap();
        Stats { hp, atk }
    }

    fn part1(&self, boss: &Self::Input) -> String {
        let effects = [0, 0, 0]; // shield, poison, recharge
        let best = Cell::new(u16::MAX);
        turn::<_, false>(
            &effects,
            50,
            boss,
            true,
            500,
            0,
            &mut |vict, cost| {
                if vict {
                    best.set(best.get().min(cost));
                }
            },
            &best,
        );
        best.get().to_string()
    }

    fn part2(&self, boss: &Self::Input) -> String {
        let effects = [0, 0, 0]; // shield, poison, recharge
        let best = Cell::new(u16::MAX);
        turn::<_, true>(
            &effects,
            50,
            boss,
            true,
            500,
            0,
            &mut |vict, cost| {
                if vict {
                    best.set(best.get().min(cost));
                }
            },
            &best,
        );
        best.get().to_string()
    }
}

#[allow(clippy::too_many_arguments)] // meh.
fn turn<F, const HARDMODE: bool>(
    statuses: &[u8; 3],
    pc: u16,
    boss: &Stats,
    player_turn: bool,
    mana_left: u16,
    mana_used: u16,
    on_end: &mut F,
    cutoff: &Cell<u16>,
) where
    F: FnMut(/*pc_win: */ bool, /*mana_used: */ u16),
{
    if mana_used > cutoff.get() {
        return;
    }
    let mut boss: Stats = boss.clone();
    let mut statuses = *statuses;
    let mut mana_left = mana_left;
    if statuses[0] > 0 {
        statuses[0] -= 1;
    }
    if statuses[1] > 0 {
        boss.hp = boss.hp.saturating_sub(3);
        statuses[1] -= 1;
    }
    if boss.hp == 0 {
        on_end(true, mana_used);
        return;
    }
    if statuses[2] > 0 {
        mana_left += 101;
        statuses[2] -= 1;
    }

    if !player_turn {
        let pc =
            pc.saturating_sub(1.max(boss.atk.saturating_sub(if statuses[0] > 0 { 7 } else { 0 })));
        if pc == 0 {
            on_end(false, mana_used);
            return;
        }
        turn::<F, HARDMODE>(
            &statuses, pc, &boss, true, mana_left, mana_used, on_end, cutoff,
        );
    } else {
        let pc = if HARDMODE {
            if pc == 1 {
                on_end(false, mana_used);
                return;
            }
            pc - 1
        } else {
            pc
        };
        // missile
        {
            if let Some(mana_left) = mana_left.checked_sub(53) {
                let mana_used = mana_used + 53;
                let mut boss = boss.clone();
                boss.hp = boss.hp.saturating_sub(4);
                turn::<F, HARDMODE>(
                    &statuses, pc, &boss, false, mana_left, mana_used, on_end, cutoff,
                );
            } else {
                return;
            }
        }
        // drain
        {
            if let Some(mana_left) = mana_left.checked_sub(73) {
                let mana_used = mana_used + 73;
                let mut boss = boss.clone();
                boss.hp = boss.hp.saturating_sub(2);
                let pc = pc + 2;
                turn::<F, HARDMODE>(
                    &statuses, pc, &boss, false, mana_left, mana_used, on_end, cutoff,
                );
            } else {
                return;
            }
        }
        // shield
        if statuses[0] == 0 {
            if let Some(mana_left) = mana_left.checked_sub(113) {
                let mana_used = mana_used + 113;
                let mut statuses = statuses;
                statuses[0] = 6;
                turn::<F, HARDMODE>(
                    &statuses, pc, &boss, false, mana_left, mana_used, on_end, cutoff,
                );
            } else {
                return;
            }
        }
        // poison
        if statuses[1] == 0 {
            if let Some(mana_left) = mana_left.checked_sub(173) {
                let mana_used = mana_used + 173;
                let mut statuses = statuses;
                statuses[1] = 6;
                turn::<F, HARDMODE>(
                    &statuses, pc, &boss, false, mana_left, mana_used, on_end, cutoff,
                );
            } else {
                return;
            }
        }
        // recharge
        if statuses[2] == 0 {
            if let Some(mana_left) = mana_left.checked_sub(229) {
                let mana_used = mana_used + 229;
                let mut statuses = statuses;
                statuses[2] = 5;
                turn::<F, HARDMODE>(
                    &statuses, pc, &boss, false, mana_left, mana_used, on_end, cutoff,
                );
            }
        }
    }
}
