use std::io::{stdout, Write};

use termint::{
    enums::{bg::Bg, cursor::Cursor, fg::Fg},
    geometry::coords::Coords,
    widgets::widget::Widget,
};

pub struct Button {
    content: Box<dyn Widget>,
    selected: bool,
}

impl Button {
    /// Creates new minesweeper style [`Button`]
    pub fn new<T>(content: T) -> Self
    where
        T: Into<Box<dyn Widget>>,
    {
        Self {
            content: content.into(),
            selected: false,
        }
    }

    /// Selects [`Button`]
    pub fn select(mut self) -> Self {
        self.selected = true;
        self
    }

    /// Sets whether [`Button`] is selected or not
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl Widget for Button {
    fn render(&self, pos: &Coords, size: &Coords) {
        print!("{}", self.get_string(pos, size));
        _ = stdout().flush();
    }

    fn get_string(&self, pos: &Coords, size: &Coords) -> String {
        let mut res = String::new();
        let (lb, db, w) = if self.selected {
            (0x999999, 0x696969, 0xcccccc)
        } else {
            (0xbcbcbc, 0x797979, 0xffffff)
        };

        let con_size =
            Coords::new(size.x.saturating_sub(3), size.y.saturating_sub(2));
        let hline = "▄".repeat(con_size.x);

        res.push_str(&format!(
            "{}{}{}▌{}▗{}{}▛\x1b[0m",
            Cursor::Pos(pos.x, pos.y),
            Fg::Hex(db),
            Bg::Hex(w),
            Fg::Hex(lb),
            hline,
            Bg::Hex(db),
        ));

        res.push_str(&format!(
            "{}{}{}▌{}{}▌",
            Cursor::Pos(pos.x, pos.y + 1),
            Fg::Hex(db),
            Bg::Hex(w),
            Fg::Hex(w),
            Bg::Hex(lb)
        ));
        res.push_str(&format!(
            "{}{}{}{}▌{}",
            " ".repeat(con_size.x),
            Bg::Hex(lb),
            Fg::Hex(lb),
            Bg::Hex(db),
            Bg::Hex(lb),
        ));
        res.push_str(
            &self
                .content
                .get_string(&Coords::new(pos.x + 2, pos.y + 1), &con_size),
        );

        res.push_str(&format!(
            "\x1b[0m{}{}{}▌{}{}▘{}{}{}▟\x1b[0m",
            Cursor::Pos(pos.x, pos.y + 2),
            Fg::Hex(db),
            Bg::Hex(w),
            Fg::Hex(w),
            Bg::Hex(lb),
            Fg::Hex(db),
            hline,
            Bg::Hex(lb),
        ));
        res
    }

    fn height(&self, _size: &Coords) -> usize {
        3
    }

    fn width(&self, size: &Coords) -> usize {
        self.content.width(size) + 3
    }
}

impl From<Button> for Box<dyn Widget> {
    fn from(value: Button) -> Self {
        Box::new(value)
    }
}
