use crossterm::event::KeyCode;
use termint::{
    enums::{bg::Bg, fg::Fg},
    geometry::{constrain::Constrain, coords::Coords},
    widgets::{
        layout::Layout,
        spacer::Spacer,
        span::{Span, StrSpanExtension},
        widget::Widget,
    },
};

use crate::{
    error::Error, game::Game, game_state::GameScreen,
    tui::widgets::border::Border,
};

use super::raw_span::RawSpan;

/// Help page
impl Game {
    /// Renders help page
    pub fn render_help(&self) {
        let mut help = Layout::vertical().padding((1, 2));
        help.add_child(
            Self::help_item("←↑↓→", 11, "cursor movement"),
            Constrain::Length(1),
        );
        help.add_child(
            Self::help_item("f", 11, "toggle flag"),
            Constrain::Length(1),
        );
        help.add_child(
            Self::help_item("d/Enter", 11, "display/reveal cell"),
            Constrain::Length(1),
        );
        help.add_child(
            Self::help_item("r", 11, "restart game"),
            Constrain::Length(1),
        );
        help.add_child(
            Self::help_item("i", 11, "toggle help"),
            Constrain::Length(1),
        );
        help.add_child(
            Self::help_item("Esc", 11, "quit game"),
            Constrain::Length(1),
        );

        let mut top_bar = Layout::horizontal();
        top_bar.add_child("Help".fg(Fg::Hex(0x303030)), Constrain::Min(0));

        let border = Border::new(help, true).top_bar(top_bar);
        let mut wrapper = Layout::vertical().center();
        wrapper.add_child(border, Constrain::Length(13));

        let mut layout = Layout::horizontal().center();
        layout.add_child(wrapper, Constrain::Length(40));

        print!("\x1b[H\x1b[J");
        layout.render(
            &Coords::new(1, 1),
            &Coords::new(self.size.0, self.size.1),
        );
    }

    /// Key listener for help page
    pub fn help_key_listen(&mut self, code: KeyCode) -> Result<(), Error> {
        match code {
            KeyCode::Esc | KeyCode::Char('q') => return Err(Error::ExitErr),
            KeyCode::Char('i') => {
                print!("\x1b[H\x1b[J");
                self.screen = GameScreen::Game
            }
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
            RawSpan::new(format!("{key}:"))
                .fg(Fg::Hex(0x0000ff))
                .bg(Bg::Hex(0xbcbcbc)),
            Constrain::Min(0),
        );
        layout.add_child(Spacer::new(), Constrain::Length(space));
        layout.add_child(
            Span::new(action)
                .fg(Fg::Hex(0x404040))
                .bg(Bg::Hex(0xbcbcbc)),
            Constrain::Fill,
        );
        layout
    }
}
