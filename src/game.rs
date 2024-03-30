use std::time::Duration;

use crossterm::event::{poll, read, Event, KeyCode, KeyEvent};
use termint::{
    geometry::constrain::Constrain, term::Term, widgets::block::Block,
};

use crate::{board::board::Board, error::Error};

/// Struct containing game info and implementing the game loop
pub struct Game {
    board: Board,
    game_over: bool,
}

impl Game {
    /// Creates new [`Game`] struct
    pub fn new(width: usize, height: usize, mines: usize) -> Self {
        Self {
            board: Board::new(width, height, mines),
            game_over: false,
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
        let mut block = Block::new().title("Minesweeper").center();
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

        if self.game_over {
            return self.over_key_listener(code);
        }

        match code {
            KeyCode::Esc => return Err(Error::ExitErr),
            KeyCode::Enter => {
                if !self.board.reveal() {
                    self.game_over = true;
                    self.board.reveal_mines();
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
                self.game_over = false;
            }
            _ => return Ok(()),
        }

        self.render();
        Ok(())
    }
}
