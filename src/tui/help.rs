use crossterm::event::KeyCode;
use termint::{
    geometry::constrain::Constrain,
    term::Term,
    widgets::{block::Block, layout::Layout, span::Span},
};

use crate::{error::Error, game::Game, game_state::GameScreen};

/// Help page
impl Game {
    /// Renders help page
    pub fn render_help(&self) {
        let mut block = Block::new().title("Minesweeper").center();

        block.add_child("Arrow keys: cursor movement", Constrain::Min(0));
        block.add_child("F: flag", Constrain::Min(0));
        block.add_child("D/Enter: displays/reveals cell", Constrain::Min(0));
        block.add_child("R: restarts game", Constrain::Min(0));
        block.add_child("Esc: quits game", Constrain::Min(0));

        let term = Term::new();
        _ = term.render(block);
    }

    /// Key listener for help page
    pub fn help_key_listen(&mut self, code: KeyCode) -> Result<(), Error> {
        match code {
            KeyCode::Esc => return Err(Error::ExitErr),
            KeyCode::Char('i') => self.screen = GameScreen::Game,
            _ => {}
        }
        self.render();
        Ok(())
    }

    /// Gets help item layout
    fn help_item(key: &str, action: &str) -> Layout {
        let mut layout = Layout::horizontal();
        layout.add_child(Span::new(format!(" {key} ")), Constrain::Min(0));
        layout.add_child(Span::new(format!(": {action}")), Constrain::Min(0));
        layout
    }
}
