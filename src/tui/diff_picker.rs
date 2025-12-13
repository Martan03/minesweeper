use crossterm::event::{KeyCode, KeyEvent};
use termint::{
    enums::Color,
    geometry::{Constraint, TextAlign, Vec2},
    widgets::{Element, Layout, ToSpan},
};

use crate::{
    app::{App, Screen},
    board::board_struct::Board,
    error::Error,
    tui::widgets::{border::Border, button::Button},
};

impl App {
    /// Renders difficulty picker
    pub fn render_dp(&mut self) -> Element {
        let mut layout = Layout::vertical();
        layout.push(self.get_button("Easy", 0), Constraint::Min(0));
        layout.push(self.get_button("Medium", 1), Constraint::Min(0));
        layout.push(self.get_button("Hard", 2), Constraint::Min(0));

        let border = Border::new(layout, false)
            .top_bar("Minesweeper".fg(Color::Hex(0x303030)));

        let mut wrapper = Layout::vertical().center();
        wrapper.push(border, Constraint::Min(0));
        let mut main = Layout::horizontal().center();
        main.push(wrapper, Constraint::Min(0));
        main.into()
    }

    /// Difficulty picker key listener
    pub fn listen_dp(&mut self, event: KeyEvent) -> Result<(), Error> {
        match event.code {
            KeyCode::Up => {
                self.picker_state = self.picker_state.saturating_sub(1)
            }
            KeyCode::Down => {
                self.picker_state += (self.picker_state < 2) as usize
            }
            KeyCode::Enter => self.eval_diff(),
            KeyCode::Esc | KeyCode::Char('q') => return Err(Error::Exit),
            _ => return Ok(()),
        };
        self.render()
    }

    fn eval_diff(&mut self) {
        let (size, mines) = match self.picker_state {
            0 => (Vec2::new(9, 9), 10),
            1 => (Vec2::new(16, 16), 40),
            _ => (Vec2::new(30, 16), 99),
        };
        self.board = Board::new(size, mines);
        self.screen = Screen::Game;
    }

    /// Difficulty picker button getter
    fn get_button(&self, text: &str, id: usize) -> Button {
        Button::new(text.fg(Color::Hex(0x303030)).align(TextAlign::Center))
            .selected(id == self.picker_state)
    }
}
