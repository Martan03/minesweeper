use app::App;
use args::Difficulty;
use error::Error;
use termint::{enums::Color, geometry::Vec2, widgets::ToSpan};

use crate::args::Args;

mod app;
mod args;
mod board;
mod error;
mod game_state;
mod tui;

fn main() {
    if let Err(e) = run() {
        eprintln!("{} {}", "Error:".fg(Color::Red), e);
        std::process::exit(1);
    };
}

fn run() -> Result<(), Error> {
    let args = Args::parse(std::env::args())?;
    if args.help {
        return Ok(());
    }

    let mut app = App::new(get_diff(args.diff));
    app.run()
}

fn get_diff(diff: Option<Difficulty>) -> Option<(Vec2, usize)> {
    match diff? {
        Difficulty::Easy => (Vec2::new(9, 9), 10),
        Difficulty::Medium => (Vec2::new(16, 16), 40),
        Difficulty::Hard => (Vec2::new(30, 16), 99),
        Difficulty::Custom(w, h, m) => (Vec2::new(w, h), m),
    }
    .into()
}
