use std::{
    env,
    fs::create_dir_all,
    io::{stdout, Write},
    process::{Command, ExitCode},
};

use app::App;
use args::Action;
use config::{config_dir, config_file, Config};
use error::Result;
use help::print_help;
use pareg::Pareg;
use termint::{enums::fg::Fg, widgets::span::StrSpanExtension};

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
