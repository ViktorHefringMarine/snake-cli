pub mod commands;
pub mod terminal;
use commands::{Command, Direction};
use std::{collections::LinkedList, thread};
use terminal::Terminal;
use thiserror::Error;

use std::{
    io::Stdout,
    ops::{Div, Range},
    time::Duration,
};

use crossterm::{
    cursor,
    style::{Print, SetForegroundColor},
    ExecutableCommand,
};

use self::terminal::TerminalSize;
type Cell = (u16, u16);

// type Loc = (u16, u16);
const SECOND: f64 = 60.0;

pub struct Settings {
    pub initial_move_frequency: u16,
}

pub struct Game {
    stdout: Stdout,
    term: Terminal,
    settings: Settings,
}

trait Render {
    fn render_cell<T: std::fmt::Display + Clone>(&mut self, n: Cell, char: T);
    fn render_background(&mut self) -> Result<(), crossterm::ErrorKind>;
    fn render_frame(&mut self);
    fn render_points<'a, T: Iterator<Item = &'a Cell>>(&mut self, points: T);
}

impl Render for Game {
    fn render_cell<T: std::fmt::Display + Clone>(&mut self, (r, c): Cell, char: T) {
        self.stdout
            .execute(cursor::MoveTo(r, c))
            .unwrap()
            .execute(Print(char))
            .unwrap();
    }

    fn render_background(&mut self) -> Result<(), crossterm::ErrorKind> {
        self.stdout
            .execute(SetForegroundColor(self.term.color))
            .unwrap();
        for col in 1..self.term.size.n_cols / 2 {
            for row in 1..self.term.size.n_rows {
                self.stdout
                    .execute(cursor::MoveTo(col * 2, row))?
                    .execute(Print("•"))?
                    .execute(cursor::MoveRight(1))?
                    .execute(Print(" "))?;
            }
        }
        Ok(())
    }

    fn render_frame(&mut self) {
        let TerminalSize {
            n_rows: n,
            n_cols: m,
        } = self.term.size;

        (1..m)
            .map(|c| ((c, 0), '─'))
            .chain((1..m).map(|c| ((c, n), '─')))
            .chain((1..n).map(|r| ((0, r), '│')))
            .chain((1..n).map(|r| ((m, r), '│')))
            .chain(vec![
                ((0, 0), '╭'),
                ((0, n), '╰'),
                ((m, 0), '╮'),
                ((m, n), '╯'),
            ])
            .for_each(|(cell, symbol)| {
                self.render_cell(cell, symbol);
            })
    }

    fn render_points<'a, T: Iterator<Item = &'a Cell>>(&mut self, points: T) {
        for (c, r) in points {
            self.render_cell((*c, *r), '');
        }
    }
}

#[derive(Error, Debug)]
#[error("hit the wall")]
pub struct InvalidMoveError;

struct Snake {
    pub cells: LinkedList<Cell>,
}

impl Snake {
    pub fn move_to(&mut self, direction: Direction) -> Result<(), InvalidMoveError> {
        let (r, c) = self.cells.front().unwrap();
        let new_cell: Cell = match direction {
            // Direction::Up => (c.checked_sub(1).ok_or_else(|| InvalidMoveError)?, *r),
            // Direction::Right => (*c, r.checked_add(1).ok_or_else(|| InvalidMoveError)?),
            // Direction::Down => (c.checked_add(1).ok_or_else(|| InvalidMoveError)?, *r),
            // Direction::Left => (*c, r.checked_sub(1).ok_or_else(|| InvalidMoveError)?),
            Direction::Up => (*c, *r + 1),
            Direction::Right => (*c, *r),
            Direction::Down => (*c, *r),
            Direction::Left => (*c, *r),
        };
        self.cells.push_front(new_cell);
        self.cells.pop_back();
        Ok(())
    }

}

impl Game {
    pub fn new(stdout: Stdout, term: Terminal, settings: Settings) -> Self {
        Self {
            stdout,
            term,
            settings,
        }
    }

    pub fn run(&mut self) -> Result<(), crossterm::ErrorKind> {
        // let (mut x, mut y):(u16,u16) = (4,4);

        // println!("{} {}", self.term.size.n_rows,self.term.size.n_cols);
        self.term.initialize(&mut self.stdout);
        self.render_background()?;
        self.render_frame();

        // let mut current_direction = Direction::Right;
        let mut cmd = Command::Move(Direction::Right);

        let mut snake = {
            let row_center = self.term.size.n_rows.div(2);
            let col_center = self.term.size.n_cols.div(2);
            Snake {
                cells: (row_center..row_center + 3)
                    .into_iter()
                    .map(|row| (col_center, row))
                    .collect(),
            }
        };

        loop {
            match cmd.wait_for_next_move() {
                Ok(_) => match cmd {
                    Command::Quit => break,
                    Command::Move(direction) => {
                        if let Err(_) = snake.move_to(direction) {
                            break;
                        }
                    }
                },
                Err(_) => (),
            }
            self.render_background()?;
            self.render_points(snake.cells.iter());
            thread::sleep(Duration::from_millis(100));
        }

        self.term.reset(&mut self.stdout);
        return Ok(());
    }

    fn calculate_movement_frequency(&mut self) -> f64 {
        self.settings.initial_move_frequency as f64
    }

    fn game_speed(&mut self) -> Duration {
        Duration::from_millis(100 * (SECOND / self.calculate_movement_frequency()) as u64)
    }
}

