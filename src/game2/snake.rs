use std::{
    collections::{BTreeSet, LinkedList},
    io,
    ops::Div,
    sync::{Arc, Mutex},
    thread, time,
};

use crossterm::{cursor, style, ExecutableCommand};

use super::commands::{Command, Direction};

pub struct Snake {
    cells: LinkedList<(u16, u16)>,
    // term_size: (u16, u16),
    stdout: Arc<Mutex<io::Stdout>>,
    dir: Direction
}

impl Snake {
    pub fn new((n_cols, n_rows): (u16, u16), stdout: Arc<Mutex<io::Stdout>>) -> Self {
        let row_center = n_rows.div(2 as u16);
        let col_center = n_cols.div(4) * 2;
        Self {
            cells: (0..4)
                .into_iter()
                .map(|row| (col_center, row + row_center))
                .collect(),
            // term_size: (n_cols, n_rows),
            stdout: Arc::clone(&stdout),
            dir: Direction::Up
        }
    }

    fn render_snake(prev_dir:Direction, curr_dir:Direction) -> char {
        match prev_dir {
            Direction::Up => match curr_dir {
                Direction::Up => '║',
                Direction::Down => '║',
                Direction::Right => '╔',
                Direction::Left => '╗'
            },
            Direction::Down => match curr_dir {
                Direction::Up => '║',
                Direction::Down => '║',
                Direction::Right => '╚',
                Direction::Left => '╝'
            }
            Direction::Left => match curr_dir {
                Direction::Up => '╚',
                Direction::Down => '╔',
                Direction::Right => '═',
                Direction::Left => '═'
            },
            Direction::Right => match curr_dir {
                Direction::Up => '╝',
                Direction::Down => '╗',
                Direction::Right => '═',
                Direction::Left => '═'
            }
        }
    }

    pub fn move_to(
        &mut self,
        direction: Direction,
    ) -> Result<(u16, u16), crossterm::ErrorKind> {
        let start = self.cells.front().unwrap().clone();

        // 1
        let (c, r) = start;
        self.stdout
            .lock()
            .unwrap()
            .execute(cursor::MoveTo(c, r))?
            .execute(style::Print(Self::render_snake(self.dir, direction)))?;

        // 2
        let (c, r) = match direction {
            Direction::Up => (c, r - 1),
            Direction::Right => (c + 2, r),
            Direction::Down => (c, r + 1),
            Direction::Left => {
                self.stdout
                    .lock()
                    .unwrap()
                    .execute(cursor::MoveTo(c-1, r))?
                    .execute(style::Print('═'))?;
                (c - 2, r)
            }
        };
        self.cells.contains(&(c,r)).then(|| panic!());

        self.cells.push_front((c, r));
        self.stdout
            .lock()
            .unwrap()
            .execute(cursor::MoveTo(c, r))?
            .execute(style::Print("O"))?;
        Ok((c,r))
    }

    pub fn cut_tail_of(&mut self) -> Result<(), crossterm::ErrorKind> {
        let (c, r) = self.cells.pop_back().unwrap();
        self.stdout
            .lock()
            .unwrap()
            .execute(cursor::MoveTo(c, r))?
            .execute(style::Print(" "))?;
        Ok(())
    }

    pub fn run(&mut self, candy_list: Arc<Mutex<BTreeSet<(u16, u16)>>>) {

        let mut render_intvl = time::Duration::from_millis(150);

        loop {
            let new_dir = match Command::new() {
                Some(Command::Move(new_dir)) => {
                    if self.dir.is_opposite(&new_dir) {
                        self.dir
                    } else {
                        new_dir
                    }
                }
                Some(Command::Quit) => break,
                _ => self.dir,
            };

            let cell = self.move_to(new_dir).unwrap();
            self.dir = new_dir;

            // does cell contain candy
            if candy_list.lock().unwrap().contains(&cell) {
                candy_list.lock().unwrap().remove(&cell);
                render_intvl -= time::Duration::from_millis(1);
            } else {
                self.cut_tail_of().unwrap();
            }

            thread::sleep(render_intvl);
        }
    }
}
