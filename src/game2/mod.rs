mod candy_spawner;
mod commands;
mod render;
mod snake;
use crate::game2::snake::Snake;

use thiserror::Error;

use crossterm::{cursor, style, ExecutableCommand};
use std::{
    sync::{mpsc, Arc, Mutex},
    thread, collections::BTreeSet
};

use crate::terminal::Terminal;

use self::candy_spawner::CandySpawner;

pub struct Game {
    term: Terminal,
}

#[derive(Error, Debug)]
#[error("hit the wall")]
pub struct InvalidMoveError;

impl Game {
    pub fn new(term: Terminal) -> Self {
        Self { term }
    }
    fn render_background(&mut self) -> Result<(), crossterm::ErrorKind> {
        self.term
            .stdout
            .lock()
            .unwrap()
            .execute(style::SetForegroundColor(self.term.color))?;
        for col in 1..self.term.n_cols() / 2 {
            for row in 1..self.term.n_rows() {
                self.term
                    .stdout
                    .lock()
                    .unwrap()
                    .execute(cursor::MoveTo(col * 2, row))?
                    .execute(style::Print(" "))?
                    .execute(cursor::MoveRight(1))?
                    .execute(style::Print(" "))?;
            }
        }
        Ok(())
    }

    fn render_frame(&mut self) {
        let (n, m) = self.term.size;

        (1..m * 2)
            .map(|c| ((c, 0), '═'))
            .chain((1..m * 2).map(|c| ((c, n), '═')))
            .chain((1..n).map(|r| ((0, r), '║')))
            .chain((1..n).map(|r| ((m * 2, r), '║')))
            .chain(vec![
                ((0, 0), '╔'),
                ((0, n), '╚'),
                ((m * 2, 0), '╗'),
                ((m * 2, n), '╝'),
            ])
            .for_each(|((c, r), symbol)| {
                self.term
                    .stdout
                    .lock()
                    .unwrap()
                    .execute(cursor::MoveTo(c, r))
                    .unwrap()
                    .execute(style::Print(symbol))
                    .unwrap();
            })
    }

    pub fn run(&mut self) -> Result<(), crossterm::ErrorKind> {
        let (sender, receiver) = mpsc::channel();

        self.term.initialize();
        self.render_background().unwrap();
        self.render_frame();

        let mut candy_list = Arc::new(Mutex::new(BTreeSet::<(u16, u16)>::new()));

        let mut snake = Snake::new(self.term.size, Arc::clone(&mut self.term.stdout));

        let mut candy_spawner = CandySpawner::new(
            self.term.size,
            Arc::clone(&mut self.term.stdout),
            Arc::clone(&mut candy_list),
        );

        // let candylist = Arc::clone(&mut candy_spawner.locs);

        let candy_thread = thread::spawn(move || candy_spawner.run(receiver));
        let snake_thread = thread::spawn(move || snake.run(Arc::clone(&mut candy_list)));

        match snake_thread.join() {
            Ok(_) => println!("snake thread successfully exited"),
            Err(_) => println!("snake thread exit failed!!")
        };
        sender.send("Stop").unwrap();

        self.term.reset();
        match candy_thread.join() {
            Ok(_) => println!("candy thread successfully exited"),
            Err(_) => println!("candy thread exit failed!!")
        };


        Ok(())
    }
}
