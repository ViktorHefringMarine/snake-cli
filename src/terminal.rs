use std::io;
use std::sync::{Arc, Mutex};
use crossterm::{cursor, ExecutableCommand};
use crossterm::terminal::{self, ClearType};
use crossterm::style::{self, Color};


pub struct Terminal {
    pub size: (u16, u16),
    pub color: Color,
    pub stdout: Arc<Mutex<io::Stdout>>
}

impl Terminal {
    pub fn n_cols(&self) -> u16 {
        self.size.0
    }

    pub fn n_rows(&self) -> u16 {
        self.size.1
    }

    pub fn new(width:f32, height:f32, color: Color) -> Self {
        let size = {
            let size = terminal::size().unwrap(); 
            let n_rows = (  width * size.1 as f32) as u16;
            let n_cols = ( height * size.0 as f32) as u16;
            (n_cols, n_rows)
        };
        Self { 
            size,
            color,
            stdout: Arc::new(Mutex::new(io::stdout()))
        }
    }

    pub fn initialize(&mut self) {
        terminal::enable_raw_mode().unwrap();
        self.stdout.lock().unwrap().execute(terminal::SetSize(self.n_rows() + 3, self.n_cols() + 3)).unwrap()
            .execute(terminal::Clear(terminal::ClearType::All)).unwrap()
            .execute(crossterm::cursor::Hide).unwrap();
    }

    pub fn reset(&mut self) {
        self.stdout.lock().unwrap().execute(terminal::Clear(ClearType::All)).unwrap()
            .execute(cursor::Show).unwrap()
            .execute(style::ResetColor).unwrap();
        terminal::disable_raw_mode().unwrap();
    }
}


