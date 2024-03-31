use crossterm::event::KeyCode;
use termint::{
    geometry::{constrain::Constrain, direction::Direction},
    term::Term,
    widgets::{block::Block, layout::Layout},
};

use crate::{error::Error, game::Game, game_state::GameScreen};

/// Help page
impl Game {
    pub fn render_help(&self) {
        let mut block = Block::new().title("Help");

        block.add_child("Arrow keys: movement", Constrain::Min(0));
        block.add_child("F: flag", Constrain::Min(0));
        block.add_child("D/Enter: displays/reveals cell", Constrain::Min(0));
        block.add_child("R: restarts game", Constrain::Min(0));
        block.add_child("Esc: quits game", Constrain::Min(0));

        let term = Term::new();
        _ = term.render(block);
    }

    fn help_key_listen(&mut self, code: KeyCode) -> Result<(), Error> {
        match code {
            KeyCode::Esc => return Err(Error::ExitErr),
            KeyCode::Char('i') => self.screen = GameScreen::Game,
            _ => {}
        }
        Ok(())
    }
}
