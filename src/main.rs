use args::Difficulty;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use error::Error;
use game::Game;

use crate::args::Args;

mod args;
mod board;
mod error;
mod game;
mod game_state;
mod tui;

fn main() -> Result<(), String> {
    let args = Args::parse(std::env::args())?;
    if args.help {
        return Ok(());
    }

    // Saves screen, clears screen and hides cursor
    println!("\x1b[?1049h\x1b[2J\x1b[?25l");
    _ = start_game(args);
    // Restores screen
    println!("\x1b[?1049l\x1b[?25h");
    Ok(())
}

fn start_game(args: Args) -> Result<(), Error> {
    let mut game = match args.diff {
        Difficulty::Easy => Game::new(9, 9, 10),
        Difficulty::Medium => Game::new(16, 16, 40),
        Difficulty::Hard => Game::new(30, 16, 99),
        Difficulty::Custom(w, h, m) => Game::new(w, h, m),
    };

    enable_raw_mode()?;
    _ = game.game_loop();
    Ok(disable_raw_mode()?)
}
