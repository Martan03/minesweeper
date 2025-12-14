use app::App;
use std::{
    env,
    fs::create_dir_all,
    io::{stdout, Write},
    panic::{set_hook, take_hook},
    process::{Command, ExitCode},
};
use termint::{enums::Color, widgets::ToSpan};

use args::Action;
use config::{config_dir, config_file, Config};
use crossterm::terminal::{disable_raw_mode, is_raw_mode_enabled};
use error::Result;
use help::print_help;
use pareg::Pareg;

use crate::args::Args;

mod app;
mod args;
mod board;
mod config;
mod error;
mod game_state;
mod help;
mod tui;

fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("{} {}", "Error:".fg(Color::Red), e);
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    // Restore the terminal even when we panic
    register_panic_hook();

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
    let mut app = App::new(args.diff.or(conf.default_difficulty));
    app.run()
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

fn register_panic_hook() {
    let hook = take_hook();
    set_hook(Box::new(move |pi| {
        if is_raw_mode_enabled().unwrap_or_default() {
            // Restores screen
            print!("\x1b[?1049l\x1b[?25h");
            _ = stdout().flush();
            _ = disable_raw_mode();
        }
        hook(pi);
    }));
}
