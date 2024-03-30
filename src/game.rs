use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use termint::{
    geometry::{constrain::Constrain, direction::Direction},
    term::Term,
    widgets::{block::Block, spacer::Spacer},
};

use crate::{board::board::Board, error::Error, game_state::GameState};

/// Struct containing game info and implementing the game loop
#[derive(Debug, Clone)]
pub struct Game {
    board: Board,
    state: GameState,
}

impl Game {
    /// Creates new [`Game`] struct
    pub fn new(width: usize, height: usize, mines: usize) -> Self {
        Self {
            board: Board::new(width, height, mines),
            state: GameState::Playing,
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
        print!("\x1b[H\x1b[J");
        let mut block = Block::new()
            .title("Minesweeper")
            .direction(Direction::Vertical)
            .center();

        if self.state == GameState::Win {
            block.add_child("Victory!", Constrain::Length(1));
        } else {
            block.add_child(Spacer::new(), Constrain::Length(1))
        }

        block.add_child(
            self.board.get_element(),
            Constrain::Length(self.board.height * 3),
        );

        let term = Term::new();
        _ = term.render(block);
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
            KeyCode::Char('f') => self.board.flag(),
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
