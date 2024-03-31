use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use termint::{
    enums::modifier::Modifier,
    geometry::{
        constrain::Constrain, direction::Direction, text_align::TextAlign,
    },
    term::Term,
    widgets::{
        block::Block,
        layout::Layout,
        spacer::Spacer,
        span::{Span, StrSpanExtension},
    },
};

use crate::{
    board::board::Board,
    error::Error,
    game_state::{GameScreen, GameState},
    tui::raw_span::RawSpan,
};

/// Struct containing game info and implementing the game loop
#[derive(Debug, Clone)]
pub struct Game {
    board: Board,
    state: GameState,
    pub screen: GameScreen,
}

impl Game {
    /// Creates new [`Game`] struct
    pub fn new(width: usize, height: usize, mines: usize) -> Self {
        Self {
            board: Board::new(width, height, mines),
            state: GameState::Playing,
            screen: GameScreen::Game,
        }
    }

    /// Main game loop
    pub fn game_loop(&mut self) -> Result<(), Error> {
        self.render();
        loop {
            if poll(Duration::from_millis(100))? {
                self.key_listener()?;
            }
        }
    }
}

// Private methods implementations
impl Game {
    fn render(&self) {
        match self.screen {
            GameScreen::Game => self.render_g(),
            GameScreen::Help => self.render_help(),
        }
    }

    /// Renders game
    fn render_g(&self) {
        print!("\x1b[H\x1b[J");
        let layout = match Term::get_size() {
            Some((w, h))
                if self.board.width * 5 + 2 >= w
                    || self.board.height * 3 + 2 >= h =>
            {
                self.render_small_msg()
            }
            _ => self.render_game(),
        };

        let mut block = Block::new()
            .title("Minesweeper")
            .center()
            .direction(Direction::Horizontal);
        block.add_child(layout, Constrain::Length(self.board.width * 5));

        let term = Term::new();
        _ = term.render(block);
    }

    fn render_game(&self) -> Layout {
        let mut layout = Layout::vertical().center();
        layout.add_child(self.render_stats(), Constrain::Length(1));
        layout.add_child(
            self.board.get_element(),
            Constrain::Length(self.board.height * 3),
        );
        layout
            .add_child(RawSpan::new(" 🛈 Press i for help"), Constrain::Min(0));
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

    /// Renders stats
    fn render_stats(&self) -> Layout {
        let mut layout = Layout::horizontal().padding((0, 1));
        layout.add_child(
            Span::new(self.board.flags_left().to_string()),
            Constrain::Min(0),
        );
        layout.add_child(Spacer::new(), Constrain::Fill);

        if self.state == GameState::Win {
            layout.add_child("Victory!", Constrain::Min(0));
        }
        layout
    }

    fn key_listener(&mut self) -> Result<(), Error> {
        let Event::Key(KeyEvent { code, .. }) = read()? else {
            return Ok(());
        };

        if self.state != GameState::Playing {
            return self.over_key_listener(code);
        }

        match code {
            KeyCode::Esc => return Err(Error::ExitErr),
            KeyCode::Enter | KeyCode::Char('d') => {
                if !self.board.reveal() {
                    self.state = GameState::GameOver;
                    self.board.reveal_mines();
                }
                if self.board.win() {
                    self.state = GameState::Win;
                }
            }
            KeyCode::Char('f') => {
                self.board.flag();
                if self.board.win() {
                    self.state = GameState::Win;
                }
            }
            KeyCode::Char('r') => self.board.reset(),
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

    /// Game over key listener
    fn over_key_listener(&mut self, code: KeyCode) -> Result<(), Error> {
        match code {
            KeyCode::Esc => return Err(Error::ExitErr),
            KeyCode::Char('r') => {
                self.board.reset();
                self.state = GameState::Playing;
            }
            _ => return Ok(()),
        }

        self.render();
        Ok(())
    }
}
