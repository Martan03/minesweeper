use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use termint::{
    enums::{bg::Bg, fg::Fg, modifier::Modifier},
    geometry::{constrain::Constrain, coords::Coords, text_align::TextAlign},
    term::Term,
    widgets::{
        layout::Layout, spacer::Spacer, span::StrSpanExtension, widget::Widget,
    },
};

use crate::{
    board::board::Board,
    error::Error,
    game_state::{GameScreen, GameState},
    tui::widgets::border::Border,
};

/// Struct containing game info and implementing the game loop
#[derive(Debug, Clone)]
pub struct Game {
    board: Board,
    state: GameState,
    pub screen: GameScreen,
    pub size: (usize, usize),
}

impl Game {
    /// Creates new [`Game`] struct
    /// ### Returns:
    /// - Constructed [`Game`]
    pub fn new(width: usize, height: usize, mines: usize) -> Self {
        Self {
            board: Board::new(width, height, mines),
            state: GameState::Playing,
            screen: GameScreen::Game,
            size: Term::get_size().unwrap_or((0, 0)),
        }
    }

    /// Main game loop
    pub fn game_loop(&mut self) -> Result<(), Error> {
        self.render();
        loop {
            if poll(Duration::from_millis(100))? {
                if let Some(size) = Term::get_size() {
                    if size != self.size {
                        self.size = size;
                        print!("\x1b[H\x1b[J");
                        self.render();
                    }
                }
                self.key_listener()?;
            }
        }
    }
}

// Private methods implementations
impl Game {
    /// Renders the game
    pub fn render(&self) {
        match self.screen {
            GameScreen::Game => self.render_game(),
            GameScreen::Help => self.render_help(),
        }
    }

    /// Listens to key presses
    fn key_listener(&mut self) -> Result<(), Error> {
        let Event::Key(KeyEvent { code, .. }) = read()? else {
            return Ok(());
        };

        match self.screen {
            GameScreen::Game => self.game_key_listen(code),
            GameScreen::Help => self.help_key_listen(code),
        }
    }

    /// Renders game
    fn render_game(&self) {
        let layout = if self.board.width * 6 + 9 >= self.size.0
            || self.board.height * 3 + 6 >= self.size.1
        {
            self.render_small_msg()
        } else {
            self.game_layout()
        };

        let mut main = Layout::horizontal().center();
        main.add_child(layout, Constrain::Length(self.board.width * 6 + 7));

        // print!("\x1b[H\x1b[J");
        main.render(
            &Coords::new(1, 1),
            &Coords::new(self.size.0, self.size.1),
        );
    }

    fn game_layout(&self) -> Layout {
        let mut layout = Layout::vertical().center();

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
        layout.add_child(border, Constrain::Length(self.board.height * 3 + 6));
        layout
    }

    fn render_small_msg(&self) -> Layout {
        let mut layout = Layout::vertical().center();
        layout.add_child(
            "Terminal too small!"
                .modifier(vec![Modifier::Bold])
                .align(TextAlign::Center),
            Constrain::Min(0),
        );
        layout.add_child(
            "Resize your terminal or start smaller game"
                .align(TextAlign::Center),
            Constrain::Min(0),
        );
        layout
    }

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

    fn game_key_listen(&mut self, code: KeyCode) -> Result<(), Error> {
        let playing = self.state == GameState::Playing;
        match code {
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
            KeyCode::Char('i') => self.screen = GameScreen::Help,
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
