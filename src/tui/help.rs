use crossterm::event::{KeyCode, KeyEvent};
use termint::{
    enums::Color,
    geometry::Constraint,
    widgets::{Element, Layout, Spacer, Span, StrSpanExtension},
};

use crate::{
    app::{App, Screen},
    error::Error,
    tui::widgets::border::Border,
};

use super::raw_span::RawSpan;

impl App {
    /// Renders help page
    pub fn render_help(&mut self) -> Element {
        let mut help = Layout::vertical().padding((1, 2));
        help.push(Self::help_item("←↑↓→", 11, "cursor movement"), 1);
        help.push(Self::help_item("f", 11, "toggle flag"), 1);
        help.push(Self::help_item("d/Enter", 11, "display/reveal cell"), 1);
        help.push(Self::help_item("r", 11, "restart game"), 1);
        help.push(Self::help_item("i", 11, "toggle help"), 1);
        help.push(Self::help_item("c", 11, "center the cursor"), 1);
        help.push(Self::help_item("Esc", 11, "quit game"), 1);

        let mut top_bar = Layout::horizontal();
        top_bar.push("Help".fg(Color::Hex(0x303030)), Constraint::Min(0));

        let border = Border::new(help, true).top_bar(top_bar);
        let mut wrapper = Layout::vertical().center();
        wrapper.push(border, Constraint::Length(14));

        let mut layout = Layout::horizontal().center();
        layout.push(wrapper, Constraint::Length(40));
        layout.into()
    }

    /// Key listener for help page
    pub fn listen_help(&mut self, event: KeyEvent) -> Result<(), Error> {
        match event.code {
            KeyCode::Char('i') => self.screen = Screen::Game,
            KeyCode::Esc | KeyCode::Char('q') => return Err(Error::Exit),
            _ => return Ok(()),
        }
        self.render()
    }

    /// Gets help item layout
    fn help_item(key: &str, key_len: usize, action: &str) -> Layout {
        let mut layout = Layout::horizontal();
        let space = key_len.saturating_sub(key.chars().count() + 2);
        layout.push(
            RawSpan::new(format!("{key}:"))
                .fg(Color::Hex(0x0000ff))
                .bg(Color::Hex(0xbcbcbc)),
            Constraint::Min(0),
        );
        layout.push(Spacer::new(), Constraint::Length(space));
        layout.push(
            Span::new(action)
                .fg(Color::Hex(0x404040))
                .bg(Color::Hex(0xbcbcbc)),
            Constraint::Fill(1),
        );
        layout
    }
}
