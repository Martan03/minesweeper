use std::{
    io::{stdout, Write},
    process::ExitCode,
};

use args::Difficulty;
use config::Config;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use error::Result;
use game::Game;
use termint::{enums::fg::Fg, widgets::span::StrSpanExtension};
use tui::diff_picker::diff_picker;

use crate::args::Args;

mod args;
mod board;
mod config;
mod error;
mod game;
mod game_state;
mod tui;

fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            _ = disable_raw_mode();
            // Restores screen
            print!("\x1b[?1049l\x1b[?25h");
            _ = stdout().flush();
            eprintln!("{} {}", "Error:".fg(Fg::Red), e);
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    let args = Args::parse(std::env::args())?;
    if args.help {
        return Ok(());
    }

    // Saves screen, clears screen and hides cursor
    print!("\x1b[?1049h\x1b[2J\x1b[?25l");
    _ = stdout().flush();
    start_game(args, Config::from_default_json())?;
    _ = stdout().flush();
    Ok(())
}

fn start_game(args: Args, conf: Config) -> Result<()> {
    enable_raw_mode()?;
    let diff = match args.diff.or(conf.default_difficulty) {
        Some(diff) => diff,
        None => diff_picker()?,
    };
    let mut game = match diff {
        Difficulty::Easy => Game::new(9, 9, 10),
        Difficulty::Medium => Game::new(16, 16, 40),
        Difficulty::Hard => Game::new(30, 16, 99),
        Difficulty::Custom {
            width,
            height,
            mines,
        } => Game::new(width, height, mines),
    };

    game.game_loop()
}
