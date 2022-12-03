use crate::cli::Result;
use crate::days::day5::run;
use crate::util::progressd;
use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::{Acquire, Relaxed, Release};
use std::sync::mpsc::{channel, Receiver, TryRecvError};
use std::sync::Arc;
use std::thread::scope;
use std::time::Duration;

#[derive(Default)]
pub struct Day {}

struct NonBlockRecv {
    inner: Receiver<(isize, isize)>,
    buf: Option<isize>,
    waiting: Arc<AtomicBool>,
}

impl Iterator for NonBlockRecv {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(val) = self.buf {
            self.buf = None;
            return Some(val);
        }

        match self.inner.try_recv() {
            Err(TryRecvError::Empty) => {
                self.waiting.store(true, Relaxed);
                Some(-1)
            }
            Ok(val) => {
                self.waiting.store(false, Relaxed);
                self.buf = Some(val.1);
                Some(val.0)
            }
            Err(TryRecvError::Disconnected) => None,
        }
    }
}

impl<'i> crate::cli::Day<'i> for Day {
    type Input = Vec<isize>;

    fn gen(&mut self, data: &str) -> Result<Self::Input> {
        Ok(data.split(',').map(|c| c.parse().unwrap()).collect())
    }

    fn part1(&mut self, code: &Self::Input) -> Result<String> {
        let mut result = 0;
        scope(|s| {
            let mut table = HashMap::new();

            for i in 0isize..50 {
                let (cpu_out, out) = channel();
                let (entry, cpu_in) = channel();
                table.insert(i, (entry, out));
                s.spawn(move || {
                    run(
                        code,
                        NonBlockRecv {
                            // address request
                            buf: Some(i),
                            inner: cpu_in,
                            waiting: Default::default(),
                        },
                        cpu_out,
                    )
                });
            }

            loop {
                // route incoming
                for (_i, (_, q)) in table.values().enumerate() {
                    if let Ok(dst) = q.recv_timeout(Duration::from_millis(1)) {
                        let x = q.recv().unwrap();
                        let y = q.recv().unwrap();
                        if dst == 255 {
                            result = y;
                            return;
                        }
                        table[&dst].0.send((x, y)).unwrap();
                    }
                }
            }
        });

        Ok(result.to_string())
    }

    fn part2(&mut self, code: &Self::Input) -> Result<String> {
        let mut result = 0;
        scope(|s| {
            let mut table = HashMap::new();
            let mut nat = (0, 0);
            let mut nat_sent = (0, 0);
            let mut empty_flags = vec![];

            for i in 0isize..50 {
                let (cpu_out, out) = channel();
                let (entry, cpu_in) = channel();
                table.insert(i, (entry, out));
                let flag = Arc::new(AtomicBool::new(false));
                empty_flags.push(flag.clone());
                s.spawn(move || {
                    run(
                        code,
                        NonBlockRecv {
                            // address request
                            buf: Some(i),
                            inner: cpu_in,
                            waiting: flag,
                        },
                        cpu_out,
                    )
                });
            }

            loop {
                // route incoming
                let mut sent_data = false;
                for (_i, (_, q)) in table.values().enumerate() {
                    if let Ok(dst) = q.recv_timeout(Duration::from_millis(1)) {
                        let x = q.recv().unwrap();
                        let y = q.recv().unwrap();
                        if dst == 255 {
                            nat = (x, y);
                            continue;
                        }
                        table[&dst].0.send((x, y)).unwrap();
                        sent_data = true;
                    }
                }

                if !sent_data && empty_flags.iter().all(|f| f.load(Acquire)) {
                    if nat_sent.1 == nat.1 {
                        result = nat.1;
                        return;
                    } else {
                        nat_sent = nat;
                        table[&0].0.send(nat_sent).unwrap();
                        progressd(&nat_sent);
                    }
                    // reset state
                    empty_flags[0].store(false, Release);
                }
            }
        });

        Ok(result.to_string())
    }
}
