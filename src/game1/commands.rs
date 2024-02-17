use std::time::Duration;

#[allow(unused_imports)]
use crossterm::ExecutableCommand;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

// #[derive(Debug, Error)]
// #[error("GetCommandError {0}" )]
// pub struct InvalidMoveError(#[from] crossterm::ErrorKind);

// impl From<crossterm::ErrorKind> for InvalidMoveError {
//     fn from(val: crossterm::ErrorKind) -> Self {
//         InvalidMoveError(format!("GetCommandError {}", val))
//     }
// }

pub struct InvalidDirection;
struct InvalidCommand;
// impl From<InvalidCommand> for InvalidMoveError {
//     fn from(_: InvalidCommand) -> Self {
//         InvalidMoveError("GetCommandError: Couldn't get command".to_string())
//     }
// }

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn is_opposite(&mut self, other: &Direction) -> bool {
        match (self, other) {
            (Direction::Up, Direction::Down) => true,
            (Direction::Right, Direction::Left) => true,
            (Direction::Down, Direction::Up) => true,
            (Direction::Left, Direction::Right) => true,
            _ => false,
        }
    }
    fn move_to(&mut self, other_direction: Direction) {
        if !self.is_opposite(&other_direction) {
            *self = other_direction;
        }
    }
}


#[derive(Debug)]
pub enum Command {
    Move(Direction),
    Quit,
}

pub struct InvalidCommandError;

impl TryFrom<KeyEvent> for Command {
    type Error = InvalidCommandError;
    fn try_from(key_event: KeyEvent) -> Result<Self, InvalidCommandError> {
        match key_event.modifiers {
            KeyModifiers::NONE => match key_event.code {
                KeyCode::Char('q') => Ok(Command::Quit),
                KeyCode::Char('j') => Ok(Command::Move(Direction::Down)),
                KeyCode::Char('k') => Ok(Command::Move(Direction::Up)),
                KeyCode::Char('h') => Ok(Command::Move(Direction::Left)),
                KeyCode::Char('l') => Ok(Command::Move(Direction::Right)),
                _ => Err(InvalidCommandError)

            },
            KeyModifiers::CONTROL => match key_event.code {
                KeyCode::Char('c') => Ok(Command::Quit),
                _ => Err(InvalidCommandError),

            },
            _ => Err(InvalidCommandError)
        }

    }
}

impl Command {

    #[allow(unused_results)] 
    pub fn wait_for_next_move(&mut self) -> Result<(), crossterm::ErrorKind> {
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(keyevent) = event::read()? {
                match keyevent.try_into() {
                    Ok(command) => {
                        *self = command;
                    },
                    Err(_) => ()
                }
            }
        }
        // thread::sleep(Duration::from_millis(500));
        Ok(())
    }
}

// if event::poll(timout)? {
//     if let Event::Key(key) = event::read()? {
//         return Ok(key.try_into().unwrap());
//     } else {
//         return Ok(self);
//     }
// }
// return Ok(*self);
