use crossterm::event::KeyCode;
use termint::{
    enums::fg::Fg,
    geometry::{constrain::Constrain, direction::Direction},
    term::Term,
    widgets::{
        block::Block, border::BorderType, layout::Layout, spacer::Spacer,
        span::Span,
    },
};

use crate::{error::Error, game::Game, game_state::GameScreen};

use super::raw_span::RawSpan;

/// Help page
impl Game {
    /// Renders help page
    pub fn render_help(&self) {
        let mut block = Block::new()
            .title("Minesweeper")
            .direction(Direction::Horizontal)
            .border_type(BorderType::Thicker)
            .center();

        let mut layout = Layout::vertical().center();
        layout.add_child(
            Self::help_item("←↑↓→", 11, "cursor movement"),
            Constrain::Length(1),
        );
        layout.add_child(
            Self::help_item("f", 11, "toggle flag"),
            Constrain::Length(1),
        );
        layout.add_child(
            Self::help_item("d/Enter", 11, "display/reveal cell"),
            Constrain::Length(1),
        );
        layout.add_child(
            Self::help_item("r", 11, "restart game"),
            Constrain::Length(1),
        );
        layout.add_child(
            Self::help_item("i", 11, "toggle help"),
            Constrain::Length(1),
        );
        layout.add_child(
            Self::help_item("Esc", 11, "quit game"),
            Constrain::Length(1),
        );

        block.add_child(layout, Constrain::Length(29));
        let term = Term::new();
        _ = term.render(block);
    }

    /// Key listener for help page
    pub fn help_key_listen(&mut self, code: KeyCode) -> Result<(), Error> {
        match code {
            KeyCode::Esc => return Err(Error::ExitErr),
            KeyCode::Char('i') => self.screen = GameScreen::Game,
            _ => {}
        }
        self.render();
        Ok(())
    }

    /// Gets help item layout
    fn help_item(key: &str, key_len: usize, action: &str) -> Layout {
        let mut layout = Layout::horizontal();
        let space = key_len.saturating_sub(key.chars().count() + 2);
        layout.add_child(
            RawSpan::new(format!("{key}:")).fg(Fg::Cyan),
            Constrain::Min(0),
        );
        layout.add_child(Spacer::new(), Constrain::Length(space));
        layout.add_child(Span::new(action), Constrain::Fill);
        layout
    }
}
