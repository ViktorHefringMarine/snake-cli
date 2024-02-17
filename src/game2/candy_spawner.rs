use std::{
    collections::BTreeSet,
    io,
    sync::{mpsc, Arc, Mutex},
    time,
};

use crossterm::{cursor, style, ExecutableCommand};
use rand::Rng;

pub struct CandySpawner {
    pub locs: Arc<Mutex<BTreeSet<(u16, u16)>>>,
    stdout: Arc<Mutex<io::Stdout>>,
    term_size: (u16, u16),
}

impl CandySpawner {
    pub fn new(
        term_size: (u16, u16),
        stdout: Arc<Mutex<io::Stdout>>,
        candy_list: Arc<Mutex<BTreeSet<(u16, u16)>>>,
    ) -> Self {
        Self {
            locs: candy_list,
            stdout,
            term_size,
        }
    }

    fn get_new_candy(&self) -> (u16, u16) {
        let get_candy = || {
            (
                rand::thread_rng().gen_range(2, (self.term_size.0 - 2) / 2) * 2,
                rand::thread_rng().gen_range(2, self.term_size.1 - 2),
            )
        };
        loop {
            let candy = get_candy();
            if self.locs.lock().unwrap().contains(&candy) {
                continue;
            }
            return candy;
        }
    }

    pub fn add_new_candy(&mut self) {
        let (c, r) = self.get_new_candy();
        self.locs.lock().unwrap().insert((c, r));
        self.stdout
            .lock()
            .unwrap()
            .execute(cursor::MoveTo(c, r))
            .unwrap()
            .execute(style::Print(''))
            .unwrap();
    }

    fn received_stop_signal(receiver: &mpsc::Receiver<&str>) -> bool {
        match receiver.try_recv() {
            Ok(value) => value == "Stop",
            _ => false,
        }
    }

    pub fn run(&mut self, receiver: mpsc::Receiver<&str>) {
        self.add_new_candy();
        let mut tp = time::Instant::now();
        while false == CandySpawner::received_stop_signal(&receiver) {
            if (time::Instant::now() - tp) < time::Duration::from_secs(4) {
                continue;
            }
            tp = time::Instant::now();
            self.add_new_candy();
        }
    }
}
