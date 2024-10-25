use std::{
    io::{stdout, Write},
    time::Duration,
};

use crossterm::{
    event::{poll, read, Event, KeyEvent},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use termint::{
    enums::Modifier,
    geometry::{Constraint, TextAlign, Vec2},
    term::Term,
    widgets::{Layout, StrSpanExtension},
};

use crate::{board::board_struct::Board, error::Error, game_state::GameState};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Screen {
    #[default]
    Game,
    Help,
    DiffPicker,
}

#[derive(Debug)]
pub struct App {
    pub board: Board,
    pub state: GameState,
    pub screen: Screen,
    pub picker_state: usize,
    pub term: Term,
}

impl App {
    /// Creates new [`App`]
    pub fn new(diff: Option<(Vec2, usize)>) -> Self {
        if let Some((size, mines)) = diff {
            Self {
                board: Board::new(size, mines),
                state: GameState::Playing,
                screen: Default::default(),
                picker_state: 0,
                term: Term::new().small_screen(Self::small_screen()),
            }
        } else {
            Self::default()
        }
    }

    /// Runs the [`App`]
    pub fn run(&mut self) -> Result<(), Error> {
        // Saves screen, clears screen and hides cursor
        print!("\x1b[?1049h\x1b[2J\x1b[?25l");
        _ = stdout().flush();
        enable_raw_mode()?;

        let res = self.main_loop();

        disable_raw_mode()?;
        // Restores screen
        print!("\x1b[?1049l\x1b[?25h");
        _ = stdout().flush();

        match res {
            Err(Error::Exit) => Ok(()),
            _ => res,
        }
    }

    /// Main loop of the [`App`]
    fn main_loop(&mut self) -> Result<(), Error> {
        self.render()?;
        loop {
            if poll(Duration::from_millis(100))? {
                self.event()?;
            }
        }
    }

    /// Renders the [`App`]
    pub fn render(&mut self) -> Result<(), Error> {
        let screen = match self.screen {
            Screen::Game => self.render_game(),
            Screen::Help => self.render_help(),
            Screen::DiffPicker => self.render_dp(),
        };
        self.term.render(screen)?;
        Ok(())
    }

    /// Handles key listening of the [`App`]
    fn event(&mut self) -> Result<(), Error> {
        match read()? {
            Event::Key(e) => self.key_handler(e),
            Event::Resize(_, _) => self.render(),
            _ => Ok(()),
        }
    }

    /// Handles key events
    fn key_handler(&mut self, event: KeyEvent) -> Result<(), Error> {
        match self.screen {
            Screen::Game => self.listen_game(event),
            Screen::Help => self.listen_help(event),
            Screen::DiffPicker => self.listen_dp(event),
        }
    }

    /// Small screen to be displayed, when game can't fit
    fn small_screen() -> Layout {
        let mut layout = Layout::vertical().center();
        layout.push(
            "Terminal too small!"
                .modifier(Modifier::BOLD)
                .align(TextAlign::Center),
            Constraint::Min(0),
        );
        layout.push(
            "You have to increase terminal size".align(TextAlign::Center),
            Constraint::Min(0),
        );
        layout
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            board: Board::new(Vec2::new(0, 0), 0),
            state: GameState::Playing,
            screen: Screen::DiffPicker,
            picker_state: 0,
            term: Term::new().small_screen(Self::small_screen()),
        }
    }
}
