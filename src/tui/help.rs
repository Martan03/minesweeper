use crossterm::event::{KeyCode, KeyEvent};
use termint::{
    enums::Color,
    geometry::Constraint,
    widgets::{Element, Layout, Span, ToSpan},
};

use crate::{
    app::App, error::Error, game_state::Screen, tui::widgets::border::Border,
};

use super::raw_span::RawSpan;

impl App {
    /// Renders help page
    pub fn render_help(&mut self) -> Element {
        let mut help = Layout::vertical().padding((1, 2));
        help.push(Self::help_item("←↑↓→", 9, "cursor movement"), 1);
        help.push(Self::help_item("f", 9, "toggle flag"), 1);
        help.push(Self::help_item("d/Enter", 9, "display/reveal cell"), 1);
        help.push(Self::help_item("r", 9, "restart game"), 1);
        help.push(Self::help_item("i", 9, "toggle help"), 1);
        help.push(Self::help_item("c", 9, "center the cursor"), 1);
        help.push(Self::help_item("Esc", 9, "quit game"), 1);

        let mut top_bar = Layout::horizontal();
        top_bar.push("Help".fg(Color::Hex(0x303030)), Constraint::Min(0));

        let border = Border::new(help, true).top_bar(top_bar);
        let mut wrapper = Layout::vertical().center();
        wrapper.push(border, Constraint::Length(14));

        let mut layout = Layout::horizontal().center();
        layout.push(wrapper, Constraint::Length(39));
        layout.into()
    }

    /// Key listener for help page
    pub fn listen_help(&mut self, event: KeyEvent) -> Result<(), Error> {
        match event.code {
            KeyCode::Char('i') => self.screen = Screen::Game,
            KeyCode::Esc | KeyCode::Char('q') => return Err(Error::ExitErr),
            _ => return Ok(()),
        }
        self.render()
    }
}

impl App {
    /// Gets help item layout
    fn help_item(key: &str, key_len: usize, action: &str) -> Layout {
        let mut layout = Layout::horizontal();
        layout.push(
            RawSpan::new(format!("{key}:"))
                .fg(Color::Hex(0x0000ff))
                .bg(Color::Hex(0xbcbcbc)),
            Constraint::Length(key_len),
        );
        layout.push(
            Span::new(action)
                .fg(Color::Hex(0x404040))
                .bg(Color::Hex(0xbcbcbc)),
            Constraint::Fill(1),
        );
        layout
    }
}
