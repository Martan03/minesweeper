use termint::{
    enums::Color,
    geometry::{Constraint, TextAlign, Vec2},
    prelude::{KeyCode, KeyEvent},
    term::Action,
    widgets::{Button as TButton, Layout, ToSpan},
};

use crate::{
    app::App,
    args::Difficulty,
    board::board_struct::Board,
    game_state::Screen,
    message::Message,
    tui::{
        widgets::{border::Border, button::Button},
        Element,
    },
};

impl App {
    /// Renders difficulty picker
    pub fn render_dp(&self) -> Element {
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
    pub fn listen_dp(&mut self, event: KeyEvent) -> Action {
        match event.code {
            KeyCode::Up | KeyCode::Char('k') => {
                self.picker_state = self.picker_state.saturating_sub(1)
            }
            KeyCode::Down | KeyCode::Char('j') => {
                self.picker_state += (self.picker_state < 2) as usize
            }
            KeyCode::Enter => self.eval_diff(self.picker_state),
            KeyCode::Esc | KeyCode::Char('q') => return Action::QUIT,
            _ => return Action::NONE,
        };
        Action::RENDER
    }

    pub fn message_dp(&mut self, message: Message) -> Action {
        match message {
            Message::DiffSel(id) => self.eval_diff(id),
            _ => return Action::NONE,
        }
        Action::RENDER
    }

    fn eval_diff(&mut self, id: usize) {
        let diff = Difficulty::from_index(id);
        let (w, h, m) = diff.config();
        self.board = Board::new(Vec2::new(w, h), m);
        self.screen = Screen::Game;
    }

    /// Difficulty picker button getter
    fn get_button(&self, text: &str, id: usize) -> TButton<Message> {
        let btn = Button::new(
            text.fg(Color::Hex(0x303030)).align(TextAlign::Center),
        )
        .selected(id == self.picker_state);
        TButton::new(btn).on_click(Message::DiffSel(id))
    }
}
