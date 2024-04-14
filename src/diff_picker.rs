use std::time::Duration;

use crossterm::{
    event::{poll, read, Event, KeyCode, KeyEvent},
    terminal::disable_raw_mode,
};
use termint::{
    enums::fg::Fg,
    geometry::{constrain::Constrain, text_align::TextAlign},
    term::Term,
    widgets::{layout::Layout, span::StrSpanExtension},
};

use crate::{
    args::Difficulty,
    error::Error,
    tui::widgets::{border::Border, button::Button},
};

pub fn diff_picker() -> Result<Difficulty, Error> {
    let mut cur = 0;
    dp_render(cur);
    loop {
        if poll(Duration::from_millis(100))? {
            if let Some(sel) = dp_listener(&mut cur)? {
                match sel {
                    0 => return Ok(Difficulty::Easy),
                    1 => return Ok(Difficulty::Medium),
                    _ => return Ok(Difficulty::Hard),
                }
            };
        }
    }
}

fn dp_render(cur: usize) {
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

    let term = Term::new();
    _ = term.render(main);
}

fn dp_listener(cur: &mut usize) -> Result<Option<usize>, Error> {
    let Event::Key(KeyEvent { code, .. }) = read()? else {
        return Ok(None);
    };

    match code {
        KeyCode::Up => *cur = cur.saturating_sub(1),
        KeyCode::Down => *cur += (*cur < 2) as usize,
        KeyCode::Enter => return Ok(Some(*cur)),
        KeyCode::Esc | KeyCode::Char('q') => {
            disable_raw_mode()?;
            return Err(Error::ExitErr);
        }
        _ => return Ok(None),
    };

    dp_render(*cur);
    Ok(None)
}

fn get_button(text: &str, sel: bool) -> Button {
    Button::new(text.fg(Fg::Hex(0x303030)).align(TextAlign::Center))
        .selected(sel)
}
