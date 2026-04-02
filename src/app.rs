use termint::{
    enums::Modifier,
    geometry::{Constraint, TextAlign, Vec2},
    prelude::Event,
    term::{Action, Application, Frame},
    widgets::{Element, Layout, ToSpan},
};

use crate::{
    args::Difficulty,
    board::board_struct::Board,
    game_state::{GameState, Screen},
    message::Message,
};

#[derive(Debug)]
pub struct App {
    pub board: Board,
    pub state: GameState,
    pub screen: Screen,
    pub picker_state: usize,
}

impl App {
    /// Creates new [`App`]
    pub fn new(diff: Option<Difficulty>) -> Self {
        let (board, screen) = match diff {
            Some(dif) => {
                let (w, h, m) = dif.config();
                (Board::new(Vec2::new(w, h), m), Screen::Game)
            }
            None => (Board::new(Vec2::new(0, 0), 0), Screen::DiffPicker),
        };

        Self {
            board,
            state: GameState::Playing,
            screen,
            picker_state: 0,
        }
    }

    /// Small screen to be displayed, when game can't fit
    pub fn small_screen() -> Layout<Message> {
        let mut layout = Layout::vertical().center();
        layout.push(
            "Terminal too small!"
                .modifier(Modifier::BOLD)
                .align(TextAlign::Center),
            Constraint::Min(0),
        );
        layout.push(
            "You have to increase terminal size".align(TextAlign::Center),
            Constraint::Min(0),
        );
        layout
    }
}

impl Application for App {
    type Message = Message;

    fn view(&self, _frame: &Frame) -> Element<Self::Message> {
        match &self.screen {
            Screen::Game => self.render_game(),
            Screen::Help => self.render_help(),
            Screen::DiffPicker => self.render_dp(),
        }
    }

    fn event(&mut self, event: Event) -> Action {
        let Event::Key(key) = event else {
            return Action::NONE;
        };

        match &self.screen {
            Screen::Game => self.listen_game(key),
            Screen::Help => self.listen_help(key),
            Screen::DiffPicker => self.listen_dp(key),
        }
    }

    fn message(&mut self, message: Self::Message) -> Action {
        match &self.screen {
            Screen::Game => self.message_game(message),
            Screen::DiffPicker => self.message_dp(message),
            Screen::Help => Action::NONE,
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            board: Board::new(Vec2::new(0, 0), 0),
            state: GameState::Playing,
            screen: Screen::DiffPicker,
            picker_state: 0,
        }
    }
}
