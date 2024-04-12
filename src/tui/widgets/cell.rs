use termint::{
    enums::{bg::Bg, cursor::Cursor, fg::Fg, modifier::Modifier},
    geometry::coords::Coords,
    widgets::widget::Widget,
};

use crate::board::cell::{Cell, CellType};

impl Widget for Cell {
    fn render(&self, _pos: &Coords, _size: &Coords) {
        todo!()
    }

    fn get_string(&self, pos: &Coords, _size: &Coords) -> String {
        if self.cell_type == CellType::Visible {
            self.get_visible(pos)
        } else {
            self.get_hidden(pos)
        }
    }

    fn height(&self, _size: &Coords) -> usize {
        3
    }

    fn width(&self, _size: &Coords) -> usize {
        6
    }
}

impl Cell {
    fn get_visible(&self, pos: &Coords) -> String {
        let (db, lb) = if self.sel {
            (0xa0a0a0, 0x797979)
        } else {
            (0xbcbcbc, 0x797979)
        };

        let top = format!("{}{} ▆▆▆▆▆\x1b[0m", Fg::Hex(db), Bg::Hex(lb),);
        let mid1 = format!("{} {}  ", Bg::Hex(lb), Bg::Hex(db));
        let bot = format!(
            "{} {}{}▂▂▂▂▂\x1b[0m",
            Bg::Hex(lb),
            Fg::Hex(lb),
            Bg::Hex(db)
        );

        let val = match self.value {
            0x01 => format!("{}{}1 ", Modifier::Bold, Fg::Hex(0x0000ff)),
            0x02 => format!("{}{}2 ", Modifier::Bold, Fg::Hex(0x007700)),
            0x03 => format!("{}{}3 ", Modifier::Bold, Fg::Hex(0xff0000)),
            0x04 => format!("{}{}4 ", Modifier::Bold, Fg::Hex(0x000077)),
            0x05 => format!("{}{}5 ", Modifier::Bold, Fg::Hex(0x770000)),
            0x06 => format!("{}{}6 ", Modifier::Bold, Fg::Hex(0x007777)),
            0x07 => format!("{}{}7 ", Modifier::Bold, Fg::Hex(0x000000)),
            0x08 => format!("{}{}8 ", Modifier::Bold, Fg::Hex(0x777777)),
            0xff => "💣".to_string(),
            _ => "  ".to_string(),
        };

        format!(
            "{}{top}{}{mid1}{val} \x1b[0m{}{bot}",
            Cursor::Pos(pos.x, pos.y),
            Cursor::Pos(pos.x, pos.y + 1),
            Cursor::Pos(pos.x, pos.y + 2)
        )
    }

    fn get_hidden(&self, pos: &Coords) -> String {
        let (lb, db, w) = if self.sel {
            (0x999999, 0x696969, 0xcccccc)
        } else {
            (0xbcbcbc, 0x797979, 0xffffff)
        };

        let top = format!(
            "{}{}▌{}▗▄▄▄{}▛\x1b[0m",
            Fg::Hex(db),
            Bg::Hex(w),
            Fg::Hex(lb),
            Bg::Hex(db),
        );

        let mid1 = format!(
            "{}{}▌{}{}▌ ",
            Fg::Hex(db),
            Bg::Hex(w),
            Fg::Hex(w),
            Bg::Hex(lb)
        );
        let mid2 = format!(" {}{}▌\x1b[0m", Fg::Hex(lb), Bg::Hex(db));

        let bot = format!(
            "{}{}▌{}{}▘{}▄▄▄{}▟\x1b[0m",
            Fg::Hex(db),
            Bg::Hex(w),
            Fg::Hex(w),
            Bg::Hex(lb),
            Fg::Hex(db),
            Bg::Hex(lb),
        );

        let val = if self.cell_type == CellType::Flag {
            format!("{}▶", Fg::Hex(0xff0000))
        } else {
            " ".to_string()
        };
        format!(
            "{}{top}{}{mid1}{val}{mid2}{}{bot}",
            Cursor::Pos(pos.x, pos.y),
            Cursor::Pos(pos.x, pos.y + 1),
            Cursor::Pos(pos.x, pos.y + 2),
        )
    }
}

impl From<Cell> for Box<dyn Widget> {
    fn from(value: Cell) -> Self {
        Box::new(value)
    }
}
