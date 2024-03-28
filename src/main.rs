use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use error::Error;
use game::Game;

mod board;
mod error;
mod game;

fn main() {
    // Saves screen, clears screen and hides cursor
    println!("\x1b[?1049h\x1b[2J\x1b[?25l");
    _ = start_game();
    // Restores screen
    println!("\x1b[?1049l\x1b[?25h");
}

fn start_game() -> Result<(), Error> {
    let mut game = Game::new(16, 16);

    enable_raw_mode()?;
    _ = game.game_loop();
    Ok(disable_raw_mode()?)
}
