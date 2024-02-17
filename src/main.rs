
#[allow(unused)]
mod game1;

mod game2;

use crossterm::style;
use game2::Game;

mod terminal;
// use game1::{Game, Settings, terminal::Terminal};
use terminal::Terminal;





fn main() {

    let term = Terminal::new(0.8, 0.8, style::Color::Grey);

    let mut game = Game::new(term);
    
    game.run();

}
