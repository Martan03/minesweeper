use crossterm::event::{KeyCode, KeyEvent};
use termint::{
    enums::fg::Fg,
    geometry::{constrain::Constrain, text_align::TextAlign},
    widgets::{layout::Layout, span::StrSpanExtension},
};

use crate::{
    app::App,
    args::Difficulty,
    board::board::Board,
    error::Error,
    game_state::Screen,
    tui::widgets::{border::Border, button::Button},
};

impl App {
    /// Renders the difficulty picker screen
    pub fn render_picker(&self) -> Result<(), Error> {
        let cur = self.picker_cur;

        let mut layout = Layout::vertical();
        layout.add_child(get_button("Easy", cur == 0), Constrain::Min(0));
        layout.add_child(get_button("Medium", cur == 1), Constrain::Min(0));
        layout.add_child(get_button("Hard", cur == 2), Constrain::Min(0));

        let border = Border::new(layout, false)
            .top_bar("Minesweeper".fg(Fg::Hex(0x303030)));

        let mut wrapper = Layout::vertical().center();
        wrapper.add_child(border, Constrain::Min(0));
        let mut main = Layout::horizontal().center();
        main.add_child(wrapper, Constrain::Min(0));

        self.term.render(main)?;
        Ok(())
    }

    /// Listens to keyboard events when in difficulty screen
    pub fn listen_picker(&mut self, event: KeyEvent) -> Result<(), Error> {
        match event.code {
            KeyCode::Up => self.picker_cur = self.picker_cur.saturating_sub(1),
            KeyCode::Down => self.picker_cur += (self.picker_cur < 2) as usize,
            KeyCode::Enter => self.change_dif(),
            KeyCode::Esc | KeyCode::Char('q') => return Err(Error::ExitErr),
            _ => return Ok(()),
        };
        self.render()
    }
}

impl App {
    /// Changes difficulty based on the currently selected difficulty
    fn change_dif(&mut self) {
        let dif = match self.picker_cur {
            0 => Difficulty::Easy,
            1 => Difficulty::Medium,
            _ => Difficulty::Hard,
        };
        let (w, h, m) = dif.config();
        self.board = Board::new(w, h, m);
        self.screen = Screen::Game;
    }
}

/// Difficulty picker button getter
fn get_button(text: &str, sel: bool) -> Button {
    Button::new(text.fg(Fg::Hex(0x303030)).align(TextAlign::Center))
        .selected(sel)
}
