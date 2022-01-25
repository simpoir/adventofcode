#[derive(Default)]
pub struct Day {}

const WPN: [(u16, u16); 5] = [(8, 4), (10, 5), (25, 6), (40, 7), (74, 8)];
const ARM: [(u16, u16); 6] = [(0, 0), (13, 1), (31, 2), (53, 3), (75, 4), (102, 5)];
/// cost, dmg, armor
const RNG: [(u16, u16, u16); 8] = [
    (0, 0, 0),
    (0, 0, 0),
    (25, 1, 0),
    (50, 2, 0),
    (100, 3, 0),
    (20, 0, 1),
    (40, 0, 2),
    (80, 0, 3),
];

pub struct Stats {
    hp: u16,
    dmg: u16,
    armor: u16,
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
        let dmg = lines
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .parse()
            .unwrap();
        let armor = lines
            .next()
            .unwrap()
            .split_once(": ")
            .unwrap()
            .1
            .parse()
            .unwrap();
        Stats { hp, dmg, armor }
    }

    fn part1(&self, boss: &Self::Input) -> String {
        let mut min_cost = u16::MAX;
        for wpn in WPN {
            for arm in ARM {
                crate::util::combine(&RNG, &mut [(0, 0, 0); 2], &mut |rings| {
                    let pc = Stats {
                        hp: 100,
                        dmg: wpn.1 + rings[0].1 + rings[1].1,
                        armor: arm.1 + rings[0].2 + rings[1].2,
                    };
                    if encounter(&pc, boss) {
                        min_cost = min_cost.min(wpn.0 + arm.0 + rings[0].0 + rings[1].0);
                    }
                    true
                });
            }
        }
        min_cost.to_string()
    }

    fn part2(&self, boss: &Self::Input) -> String {
        let mut min_cost = u16::MIN;
        for wpn in WPN {
            for arm in ARM {
                crate::util::combine(&RNG, &mut [(0, 0, 0); 2], &mut |rings| {
                    let pc = Stats {
                        hp: 100,
                        dmg: wpn.1 + rings[0].1 + rings[1].1,
                        armor: arm.1 + rings[0].2 + rings[1].2,
                    };
                    if !encounter(&pc, boss) {
                        min_cost = min_cost.max(wpn.0 + arm.0 + rings[0].0 + rings[1].0);
                    }
                    true
                });
            }
        }
        min_cost.to_string()
    }
}

fn encounter(player: &Stats, npc: &Stats) -> bool {
    let mut hp = player.hp;
    let mut ehp = npc.hp;
    let atk = 1.max(player.dmg.saturating_sub(npc.armor));
    let dmg = 1.max(npc.dmg.saturating_sub(player.armor));
    loop {
        ehp = ehp.saturating_sub(atk);
        if ehp == 0 {
            return true;
        }
        hp = hp.saturating_sub(dmg);
        if hp == 0 {
            return false;
        }
    }
}
