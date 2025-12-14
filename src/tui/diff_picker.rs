use crossterm::event::{KeyCode, KeyEvent};
use termint::{
    enums::Color,
    geometry::{Constraint, TextAlign, Vec2},
    widgets::{Element, Layout, ToSpan},
};

use crate::{
    app::App,
    args::Difficulty,
    board::board_struct::Board,
    error::Error,
    game_state::Screen,
    tui::widgets::{border::Border, button::Button},
};

impl App {
    /// Renders difficulty picker
    pub fn render_dp(&mut self) -> Element {
        let mut layout = Layout::vertical();
        layout.push(self.get_button("Easy", 0), Constraint::Min(3));
        layout.push(self.get_button("Medium", 1), Constraint::Min(3));
        layout.push(self.get_button("Hard", 2), Constraint::Min(3));

        let border = Border::new(layout, false)
            .top_bar("Minesweeper".fg(Color::Hex(0x303030)));

        let mut wrapper = Layout::vertical().center();
        wrapper.push(border, Constraint::Min(0));
        let mut main = Layout::horizontal().center();
        main.push(wrapper, Constraint::Min(20));
        main.into()
    }

    /// Difficulty picker key listener
    pub fn listen_dp(&mut self, event: KeyEvent) -> Result<(), Error> {
        match event.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.picker_state = self.picker_state.saturating_sub(1)
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.picker_state += (self.picker_state < 2) as usize
            }
            KeyCode::Enter => self.eval_diff(),
            KeyCode::Esc | KeyCode::Char('q') => return Err(Error::ExitErr),
            _ => return Ok(()),
        };
        self.render()
    }

    fn eval_diff(&mut self) {
        let diff = match self.picker_state {
            0 => Difficulty::Easy,
            1 => Difficulty::Medium,
            _ => Difficulty::Hard,
        };
        let (w, h, m) = diff.config();
        self.board = Board::new(Vec2::new(w, h), m);
        self.screen = Screen::Game;
    }

    /// Difficulty picker button getter
    fn get_button(&self, text: &str, id: usize) -> Button {
        Button::new(text.fg(Color::Hex(0x303030)).align(TextAlign::Center))
            .selected(id == self.picker_state)
    }
}
