use std::{
    io::{stdout, Write},
    time::Duration,
};

use crossterm::{
    event::{poll, read, Event, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use termint::{
    enums::modifier::Modifier,
    geometry::{constrain::Constrain, coords::Coords, text_align::TextAlign},
    term::Term,
    widgets::{layout::Layout, span::StrSpanExtension, widget::Widget},
};

use crate::{
    args::Difficulty,
    board::board::Board,
    error::Error,
    game_state::{GameState, Screen},
};

#[derive(Debug)]
pub struct App {
    pub board: Board,
    pub state: GameState,
    pub screen: Screen,
    pub size: Coords,
    pub picker_cur: usize,
}

impl App {
    pub fn new(dif: Option<Difficulty>) -> Self {
        let (board, screen) = match dif {
            Some(dif) => {
                let (w, h, m) = dif.config();
                (Board::new(w, h, m), Screen::Game)
            }
            None => (Board::new(0, 0, 0), Screen::Picker),
        };

        Self {
            board,
            state: GameState::Playing,
            screen,
            size: Term::get_size()
                .map(|(w, h)| Coords::new(w, h))
                .unwrap_or(Coords::new(0, 0)),
            picker_cur: 0,
        }
    }

    /// Runs the [`App`]
    pub fn run(&mut self) -> Result<(), Error> {
        enable_raw_mode()?;
        // Swaps print buffer, clears screen and hides cursor
        print!("\x1b[?1049h\x1b[2J\x1b[?25l");
        _ = stdout().flush();

        let res = self.main_loop();

        // Restores screen
        print!("\x1b[?1049l\x1b[?25h");
        _ = stdout().flush();
        disable_raw_mode()?;

        match res {
            Err(Error::ExitErr) => Ok(()),
            _ => res,
        }
    }

    /// Main loop of the [`App`]
    fn main_loop(&mut self) -> Result<(), Error> {
        self.render();
        loop {
            if poll(Duration::from_millis(100))? {
                self.event()?;
            }
        }
    }

    /// Renders the [`App`]
    pub fn render(&mut self) {
        let layout = match self.screen {
            Screen::Game => self.render_game(),
            Screen::Picker => self.render_picker(),
            Screen::Help => self.render_help(),
        };
        layout.render(&Coords::new(1, 1), &self.size);
    }

    /// Handles key listening of the [`App`]
    fn event(&mut self) -> Result<(), Error> {
        match read()? {
            Event::Key(e) => self.key_handler(e),
            Event::Resize(w, h) => {
                print!("\x1b[H\x1b[J");
                _ = stdout().flush();

                self.size = Coords::new(w as usize, h as usize);
                self.render();
                Ok(())
            }
            _ => Ok(()),
        }
    }

    /// Handles key events
    fn key_handler(&mut self, event: KeyEvent) -> Result<(), Error> {
        match self.screen {
            Screen::Game => self.listen_game(event),
            Screen::Picker => self.listen_picker(event),
            Screen::Help => self.listen_help(event),
        }
    }

    /// Small screen to be displayed, when game can't fit
    pub fn small_screen() -> Layout {
        let mut layout = Layout::vertical().center();
        layout.add_child(
            "Terminal too small!"
                .modifier(vec![Modifier::Bold])
                .align(TextAlign::Center),
            Constrain::Min(0),
        );
        layout.add_child(
            "You have to increase terminal size".align(TextAlign::Center),
            Constrain::Min(0),
        );
        layout
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            board: Board::new(1, 1, 1),
            state: GameState::Playing,
            screen: Screen::Picker,
            size: Term::get_size()
                .map(|(w, h)| Coords::new(w, h))
                .unwrap_or(Coords::new(0, 0)),
            picker_cur: 0,
        }
    }
}
