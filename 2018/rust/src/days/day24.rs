use std::cmp::Reverse;

use crate::cli::Result;

#[derive(Default)]
pub struct Day {}

#[derive(Clone, Debug)]
pub struct Group<'i> {
    units: usize,
    hp: usize,
    weak: Vec<&'i str>,
    immune: Vec<&'i str>,
    atk: usize,
    atk_type: &'i str,
    initiative: usize,
    team: bool,
}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<Group<'i>>;

    fn gen(&mut self, data: &'i str) -> Result<Self::Input> {
        data.split("\n\n")
            .enumerate()
            .flat_map(|(i, team)| {
                team.lines().skip(1).map(move |l| {
                    let mut weak = vec![];
                    let mut immune = vec![];
                    if let Some(chunk) = l.split(['(', ')']).nth(1) {
                        chunk.split("; ").for_each(|spec| {
                            if let Some(weaks) = spec.strip_prefix("weak to ") {
                                weak.extend(weaks.split(", "));
                            } else if let Some(immunes) = spec.strip_prefix("immune to ") {
                                immune.extend(immunes.split(", "));
                            }
                        });
                    }
                    let mut chunks = l.split_ascii_whitespace();
                    Ok(Group {
                        units: chunks.next().unwrap().parse()?,
                        hp: chunks.nth(3).unwrap().parse()?,
                        initiative: chunks.next_back().unwrap().parse()?,
                        atk_type: chunks.nth_back(3).unwrap(),
                        atk: chunks.next_back().unwrap().parse()?,
                        weak,
                        immune,
                        team: i == 0,
                    })
                })
            })
            .collect()
    }

    fn part1(&mut self, input: &Self::Input) -> Result<String> {
        let mut groups = input.clone();
        while groups.iter().any(|g| g.team) && groups.iter().any(|g| !g.team) {
            let pre_count: usize = groups.iter().map(|g| g.units).sum();
            fight(&mut groups);
            groups.retain(|g| g.units > 0);
            if pre_count == groups.iter().map(|g| g.units).sum() {
                panic!("stalemate");
            }
        }
        Ok(groups.iter().map(|g| g.units).sum::<usize>().to_string())
    }

    fn part2(&mut self, input: &Self::Input) -> Result<String> {
        let mut groups = vec![];

        'boosting: for boost in 0.. {
            groups = input.clone();
            groups.iter_mut().for_each(|g| {
                if g.team {
                    g.atk += boost
                }
            });

            loop {
                let imm_count: usize = groups.iter().filter(|g| g.team).map(|g| g.units).sum();
                let inf_count: usize = groups.iter().filter(|g| !g.team).map(|g| g.units).sum();
                if imm_count == 0 || inf_count == 0 {
                    if inf_count == 0 {
                        break 'boosting;
                    }
                    continue 'boosting;
                }

                fight(&mut groups);
                groups.retain(|g| g.units > 0);

                if groups.iter().map(|g| g.units).sum::<usize>() == imm_count + inf_count {
                    // stale
                    continue 'boosting;
                }
            }
        }
        Ok(groups.iter().map(|g| g.units).sum::<usize>().to_string())
    }
}

impl Group<'_> {
    fn effective_power(&self) -> usize {
        self.units * self.atk
    }
}

fn fight(groups: &mut [Group]) {
    groups.sort_by_key(|g| Reverse((g.effective_power(), g.initiative)));
    let mut atk_map = vec![];
    groups.iter().for_each(|a| {
        let best = groups
            .iter()
            .enumerate()
            .filter(|(i, b)| b.team != a.team && !atk_map.contains(&Some(*i)) && dmg_calc(a, b) > 0)
            .max_by_key(|(_, b)| (dmg_calc(a, b), b.effective_power(), b.initiative))
            .map(|(i, _)| i);
        atk_map.push(best);
    });
    let mut atk_order: Vec<usize> = (0..atk_map.len()).collect();
    atk_order.sort_by_key(|i| Reverse(groups[*i].initiative));
    for i in atk_order {
        if groups[i].hp == 0 {
            // he's dead, Jim
            continue;
        }
        if let Some(tgt_i) = atk_map[i] {
            let dmg = dmg_calc(&groups[i], &groups[tgt_i]);
            let mut tgt = &mut groups[tgt_i];
            tgt.units = tgt.units.saturating_sub(dmg / tgt.hp);
        }
    }
}

fn dmg_calc(from: &Group, to: &Group) -> usize {
    from.effective_power()
        * if to.weak.contains(&from.atk_type) {
            2
        } else if to.immune.contains(&from.atk_type) {
            0
        } else {
            1
        }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::days::Day as _;
    const INPUT: &str = "\
Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";

    #[test]
    fn test_part1() {
        let mut d: Day = Default::default();
        let expected = "5216";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part1(&data).unwrap());
    }

    #[test]
    fn test_dmg() {
        let mut d: Day = Default::default();
        let data = d.gen(INPUT).unwrap();

        assert_eq!(185832, dmg_calc(&data[2], &data[0]));
        assert_eq!(185832, dmg_calc(&data[2], &data[1]));
        assert_eq!(107640, dmg_calc(&data[3], &data[1]));
        assert_eq!(76619, dmg_calc(&data[0], &data[2]));
        assert_eq!(153238, dmg_calc(&data[0], &data[3]));
        assert_eq!(24725, dmg_calc(&data[1], &data[2]));
    }

    #[test]
    fn test_part2() {
        let mut d: Day = Default::default();
        let expected = "51";
        let data = d.gen(INPUT).unwrap();
        assert_eq!(expected, d.part2(&data).unwrap());
    }
}
