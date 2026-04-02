use termint::{
    enums::Color,
    geometry::Constraint,
    prelude::{KeyCode, KeyEvent},
    term::Action,
    widgets::{Layout, Spacer, ToSpan},
};

use crate::{
    app::App,
    game_state::{GameState, Screen},
    message::Message,
    tui::Element,
};

use super::widgets::border::Border;

impl App {
    pub fn render_game(&self) -> Element {
        let help = "🛈 Press i for help".fg(Color::Hex(0x303030));

        let grid = self.board.get_element();
        let border = Border::new(grid, false)
            .top_bar(self.get_stats())
            .bot_bar(help);

        let mut layout = Layout::vertical().center();
        layout.push(border, self.board.size.y * 3 + 6);

        let mut main = Layout::horizontal().center();
        main.push(layout, self.board.size.x * 6 + 7);

        main.into()
    }

    pub fn listen_game(&mut self, event: KeyEvent) -> Action {
        match event.code {
            KeyCode::Up | KeyCode::Char('k') => self.board.cur_up(),
            KeyCode::Down | KeyCode::Char('j') => self.board.cur_down(),
            KeyCode::Left | KeyCode::Char('h') => self.board.cur_left(),
            KeyCode::Right | KeyCode::Char('l') => self.board.cur_right(),
            KeyCode::Enter | KeyCode::Char('d') if self.state.is_playing() => {
                self.reveal_board_cell();
            }
            KeyCode::Char('f') if self.state.is_playing() => {
                self.flag_board_cell();
            }
            KeyCode::Char('r') => {
                self.board.reset();
                self.state = GameState::Playing;
            }
            KeyCode::Char('c') => self.board.center(),
            KeyCode::Char('i') => self.screen = Screen::Help,
            KeyCode::Tab => self.screen = Screen::DiffPicker,
            KeyCode::Char('q') | KeyCode::Esc => return Action::QUIT,
            _ => return Action::NONE,
        }
        Action::RENDER
    }

    pub fn message_game(&mut self, message: Message) -> Action {
        match message {
            Message::CellReveal(pos) => {
                self.board.select(pos);
                if self.state.is_playing() {
                    self.reveal_board_cell();
                }
            }
            Message::CellFlag(pos) => {
                self.board.select(pos);
                if self.state.is_playing() {
                    self.flag_board_cell();
                }
            }
            _ => return Action::NONE,
        }
        Action::RENDER
    }

    fn get_stats(&self) -> Layout<Message> {
        let mut layout = Layout::horizontal();
        layout.push(
            format!("{}", self.board.flags_left()).fg(Color::Hex(0x303030)),
            Constraint::Min(0),
        );
        layout.push(Spacer::new(), Constraint::Fill(1));

        if self.state == GameState::Win {
            layout.push(
                "Victory!".fg(Color::Hex(0x303030)).bg(Color::Hex(0xbcbcbc)),
                Constraint::Min(0),
            );
        }
        layout
    }

    fn reveal_board_cell(&mut self) {
        if !self.board.reveal() {
            self.state = GameState::GameOver;
            self.board.reveal_mines();
        }
        if self.board.win() {
            self.state = GameState::Win;
        }
    }

    fn flag_board_cell(&mut self) {
        self.board.flag();
        if self.board.win() {
            self.state = GameState::Win;
        }
    }
}
