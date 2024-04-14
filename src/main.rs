use std::io::{stdout, Write};

use args::Difficulty;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use diff_picker::diff_picker;
use error::Error;
use game::Game;

use crate::args::Args;

mod args;
mod board;
mod diff_picker;
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
    print!("\x1b[?1049h\x1b[2J\x1b[?25l");
    _ = stdout().flush();
    _ = start_game(args);
    // Restores screen
    print!("\x1b[?1049l\x1b[?25h");
    _ = stdout().flush();
    Ok(())
}

fn start_game(args: Args) -> Result<(), Error> {
    enable_raw_mode()?;
    let diff = match args.diff {
        Some(diff) => diff,
        None => diff_picker()?,
    };
    let mut game = match diff {
        Difficulty::Easy => Game::new(9, 9, 10),
        Difficulty::Medium => Game::new(16, 16, 40),
        Difficulty::Hard => Game::new(30, 16, 99),
        Difficulty::Custom(w, h, m) => Game::new(w, h, m),
    };

    _ = game.game_loop();
    Ok(disable_raw_mode()?)
}
