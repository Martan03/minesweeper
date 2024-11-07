use std::io::{stdout, Write};

use crossterm::event::{KeyCode, KeyEvent};
use termint::{
    enums::{bg::Bg, fg::Fg},
    geometry::constrain::Constrain,
    widgets::{layout::Layout, spacer::Spacer, span::StrSpanExtension},
};

use crate::{
    app::App,
    error::Error,
    game_state::{GameState, Screen},
};

use super::widgets::border::Border;

impl App {
    /// Renders the game screen
    pub fn render_game(&self) -> Layout {
        let mut bot_bar = Layout::horizontal();
        bot_bar.add_child(
            "🛈 Press i for help".fg(Fg::Hex(0x303030)),
            Constrain::Min(0),
        );

        let border = Border::new(
            self.board.get_element(self.state == GameState::GameOver),
            false,
        )
        .top_bar(self.get_stats())
        .bot_bar(bot_bar);

        let mut layout = Layout::vertical().center();
        layout.add_child(border, Constrain::Length(self.board.height * 3 + 6));

        let mut main = Layout::horizontal().center();
        main.add_child(layout, Constrain::Length(self.board.width * 6 + 7));

        if self.board.width * 6 + 9 >= self.size.x
            || self.board.height * 3 + 6 >= self.size.y
        {
            main = Self::small_screen();
        }
        main
    }

    /// Listens to keyboard events when in game screen
    pub fn listen_game(&mut self, event: KeyEvent) -> Result<(), Error> {
        let playing = self.state == GameState::Playing;
        match event.code {
            KeyCode::Esc | KeyCode::Char('q') => return Err(Error::ExitErr),
            KeyCode::Char('c') => self.board.center(),
            KeyCode::Enter | KeyCode::Char('d') if playing => {
                if !self.board.reveal() {
                    self.state = GameState::GameOver;
                    self.board.reveal_mines();
                }
                if self.board.win() {
                    self.state = GameState::Win;
                }
            }
            KeyCode::Char('f') if playing => {
                self.board.flag();
                if self.board.win() {
                    self.state = GameState::Win;
                }
            }
            KeyCode::Char('r') => {
                self.board.reset();
                self.state = GameState::Playing;
            }
            KeyCode::Char('i') => {
                print!("\x1b[H\x1b[J");
                _ = stdout().flush();
                self.screen = Screen::Help;
            }
            KeyCode::Tab => {
                print!("\x1b[H\x1b[J");
                _ = stdout().flush();
                self.screen = Screen::Picker;
            }
            KeyCode::Up => self.board.cur_up(),
            KeyCode::Down => self.board.cur_down(),
            KeyCode::Left => self.board.cur_left(),
            KeyCode::Right => self.board.cur_right(),
            _ => return Ok(()),
        }
        self.render();
        Ok(())
    }
}

impl App {
    /// Gets stats layout
    fn get_stats(&self) -> Layout {
        let mut layout = Layout::horizontal();
        layout.add_child(
            format!("{}", self.board.flags_left()).fg(Fg::Hex(0x303030)),
            Constrain::Min(0),
        );
        layout.add_child(Spacer::new(), Constrain::Fill);

        if self.state == GameState::Win {
            layout.add_child(
                "Victory!".fg(Fg::Hex(0x303030)).bg(Bg::Hex(0xbcbcbc)),
                Constrain::Min(0),
            );
        }
        layout
    }
}
