use crossterm::event::{KeyCode, KeyEvent};
use termint::{
    enums::Color,
    geometry::Constraint,
    widgets::{Element, Layout, Spacer, ToSpan},
};

use crate::{
    app::{App, Screen},
    error::Error,
    game_state::GameState,
};

use super::widgets::border::Border;

impl App {
    pub fn render_game(&mut self) -> Element {
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

    pub fn listen_game(&mut self, event: KeyEvent) -> Result<(), Error> {
        match event.code {
            KeyCode::Up | KeyCode::Char('k') => self.board.cur_up(),
            KeyCode::Down | KeyCode::Char('j') => self.board.cur_down(),
            KeyCode::Left | KeyCode::Char('h') => self.board.cur_left(),
            KeyCode::Right | KeyCode::Char('l') => self.board.cur_right(),
            KeyCode::Enter | KeyCode::Char('d') if self.state.is_playing() => {
                if !self.board.reveal() {
                    self.state = GameState::GameOver;
                    self.board.reveal_mines();
                }
                if self.board.win() {
                    self.state = GameState::Win;
                }
            }
            KeyCode::Char('f') if self.state.is_playing() => {
                self.board.flag();
                if self.board.win() {
                    self.state = GameState::Win;
                }
            }
            KeyCode::Char('r') => {
                self.board.reset();
                self.state = GameState::Playing;
            }
            KeyCode::Char('c') => self.board.center(),
            KeyCode::Char('i') => self.screen = Screen::Help,
            KeyCode::Tab => self.screen = Screen::DiffPicker,
            KeyCode::Char('q') | KeyCode::Esc => return Err(Error::Exit),
            _ => return Ok(()),
        }
        self.render()
    }

    fn get_stats(&self) -> Layout {
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
}
