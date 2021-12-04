use std::collections::BTreeSet;

type Board = [[u16; 5]; 5];

#[derive(Debug)]
pub struct Game {
    nums: Vec<u16>,
    boards: Vec<Board>,
}

pub struct Day {}

impl crate::Day for Day {
    type Input = Game;

    fn gen(&self, data: &str) -> Self::Input {
        let mut blocks = data.split("\n\n");
        let nums = blocks
            .next()
            .unwrap()
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        let boards = blocks
            .map(|block| {
                let mut board = [[0; 5]; 5];
                block.lines().enumerate().for_each(|(j, l)| {
                    l.split_ascii_whitespace()
                        .filter_map(|c| {
                            if c.is_empty() {
                                None
                            } else {
                                Some(c.parse().unwrap())
                            }
                        })
                        .enumerate()
                        .for_each(|(i, x)| board[j][i] = x);
                });
                board
            })
            .collect();
        Game { nums, boards }
    }

    fn part1(&self, input: &Self::Input) -> String {
        let mut state: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; 5]; 5]; input.boards.len()];

        for n in &input.nums {
            // play
            for (state, board) in state.iter_mut().zip(&input.boards) {
                state.iter_mut().zip(board).for_each(|(s_row, b_row)| {
                    s_row.iter_mut().zip(b_row).for_each(|(s, b)| {
                        if b == n {
                            *s = true
                        }
                    })
                });
            }

            // XXX check win, probably should only check modified row/col.
            for (state, board) in state.iter().zip(&input.boards) {
                if did_win(state) {
                    let score = state.iter().zip(board).fold(0, |score, (s_row, b_row)| {
                        score
                            + s_row
                                .iter()
                                .zip(b_row)
                                .fold(0, |score, (s, b)| score + if !s { *b } else { 0 })
                    });
                    return format!("{}", n * score);
                }
            }
        }

        unimplemented!();
    }

    fn part2(&self, input: &Self::Input) -> String {
        let mut state: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; 5]; 5]; input.boards.len()];
        let mut active_boards: BTreeSet<*const Board> =
            input.boards.iter().map(|b| b as *const _).collect();

        for n in &input.nums {
            // play
            for (state, board) in state.iter_mut().zip(&input.boards) {
                // yep, we don't check if the board is inactive in the loop.
                // The extra conditional breaks cache/vectorization and
                // makes things actually slower.
                state.iter_mut().zip(board).for_each(|(s_row, b_row)| {
                    s_row.iter_mut().zip(b_row).for_each(|(s, b)| {
                        if b == n {
                            *s = true
                        }
                    })
                });
            }

            // check win
            for (state, board) in state.iter().zip(&input.boards) {
                // same comment about auto-vectorization
                if did_win(state) {
                    if active_boards.len() > 1 {
                        active_boards.remove(&(board as *const _));
                    } else {
                        if !active_boards.contains(&(board as *const _)) {
                            continue;
                        }
                        let score = state.iter().zip(board).fold(0, |score, (s_row, b_row)| {
                            score
                                + s_row
                                    .iter()
                                    .zip(b_row)
                                    .fold(0, |score, (s, b)| score + if !s { *b } else { 0 })
                        });
                        return format!("{}", n * score);
                    }
                }
            }
        }

        unimplemented!();
    }
}

#[allow(dead_code)]
fn print_board<T: std::fmt::Debug>(board: &[Vec<T>]) {
    println!();
    board.iter().for_each(|l| println!("{:?}", l));
}

#[allow(dead_code)]
fn print_state(board: &[Vec<bool>]) {
    println!();
    board.iter().for_each(|l| {
        println!(
            "{}",
            l.iter()
                .map(|c| if *c { 'O' } else { '.' })
                .collect::<String>()
        )
    });
}

fn did_win(board: &[Vec<bool>]) -> bool {
    board.iter().map(|row| row.iter().all(|x| *x)).any(|x| x)
        || (0..5)
            .map(|i| board.iter().map(|l| l[i]).all(|x| x))
            .any(|x| x)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_did_win() {
        assert!(did_win(&vec![vec![true, true, true, true, true]][..]));
        assert!(did_win(
            &vec![vec![true, false, false, false, false]; 5][..]
        ));
        assert!(!did_win(
            &vec![vec![false, false, false, false, false]; 5][..]
        ));
    }
}
