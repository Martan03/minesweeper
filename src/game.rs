use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use termint::{
    geometry::constrain::Constrain, term::Term, widgets::block::Block,
};

use crate::{board::board::Board, error::Error};

/// Struct containing game info and implementing the game loop
pub struct Game {
    board: Board,
}

impl Game {
    /// Creates new [`Game`] struct
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            board: Board::new(width, height),
        }
    }

    /// Main game loop
    pub fn game_loop(&mut self) -> Result<(), Error> {
        self.board.generate(10);
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
        let mut block = Block::new().title("Minesweeper").center();
        block.add_child(
            self.board.get_layout(),
            Constrain::Length(self.board.height * 3),
        );

        let term = Term::new();
        _ = term.render(block);
    }

    fn key_listener(&mut self) -> Result<(), Error> {
        let Event::Key(KeyEvent { code, .. }) = read()? else {
            return Ok(());
        };

        match code {
            KeyCode::Esc => Err(Error::ExitErr),
            _ => Ok(()),
        }
    }
}
