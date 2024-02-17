use std::io::Stdout;
use crossterm::cursor;
use crossterm::terminal::{self, ClearType};
use crossterm::style::{self, Color};
use crossterm::ExecutableCommand;

pub struct TerminalSize {
    pub n_rows: u16,
    pub n_cols: u16
}

pub struct Terminal {
    pub size: TerminalSize,
    pub color: Color
}

impl Terminal {
    pub fn new(width:f32, height:f32, color: Color ) -> Self {
        let size = {
            let size = terminal::size().unwrap(); 
            let n_rows = (  width * size.1 as f32) as u16;
            let n_cols = ( height * size.0 as f32) as u16;
            TerminalSize { n_rows, n_cols }
        };
        Self { 
            size,
            color 
        }
    }

    pub fn initialize(&mut self, stdout: &mut Stdout) {
        terminal::enable_raw_mode().unwrap();
        stdout.execute(terminal::SetSize(self.size.n_rows + 3, self.size.n_cols + 3)).unwrap()
            .execute(terminal::Clear(terminal::ClearType::All)).unwrap()
            .execute(crossterm::cursor::Hide).unwrap();
    }

    pub fn reset(&mut self, stdout: &mut Stdout) {
        stdout.execute(terminal::Clear(ClearType::All)).unwrap()
            .execute(cursor::Show).unwrap()
            .execute(style::ResetColor).unwrap();
        terminal::disable_raw_mode().unwrap();
    }
}


