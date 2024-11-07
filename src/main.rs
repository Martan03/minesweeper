use std::{
    env, fs::create_dir_all, io::{stdout, Write}, process::{Command, ExitCode}
};

use args::{Action, Difficulty};
use config::{config_dir, config_file, Config};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, is_raw_mode_enabled,
};
use error::Result;
use game::Game;
use help::print_help;
use pareg::Pareg;
use termint::{enums::fg::Fg, widgets::span::StrSpanExtension};
use tui::diff_picker::diff_picker;

use crate::args::Args;

mod args;
mod board;
mod config;
mod error;
mod game;
mod game_state;
mod help;
mod tui;

fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            if is_raw_mode_enabled().unwrap_or(true) {
                _ = disable_raw_mode();
                // Restores screen
                print!("\x1b[?1049l\x1b[?25h");
                _ = stdout().flush();
            }
            eprintln!("{} {}", "Error:".fg(Fg::Red), e);
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    let args = Args::parse(Pareg::args())?;
    match args.action {
        Action::Play => play(args),
        Action::Help => {
            print_help();
            Ok(())
        }
        Action::Config => config(),
    }
}

fn play(args: Args) -> Result<()> {
    start_game(args, Config::from_default_json())?;
    _ = stdout().flush();
    Ok(())
}

fn start_game(args: Args, conf: Config) -> Result<()> {
    enable_raw_mode()?;
    print!("\x1b[?1049h\x1b[2J\x1b[?25l");
    _ = stdout().flush();

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

fn config() -> Result<()> {
    let editor = env::var("EDITOR").unwrap_or("vi".to_string());
    create_dir_all(config_dir())?;
    let file = config_file();
    if !file.exists() {
        Config::default().to_default_json()?;
    }

    Command::new(editor).arg(file).spawn()?.wait()?;
    Ok(())
}
