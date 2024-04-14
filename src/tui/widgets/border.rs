use std::io::{stdout, Write};

use termint::{
    enums::{bg::Bg, cursor::Cursor, fg::Fg},
    geometry::coords::Coords,
    widgets::{layout::Layout, widget::Widget},
};

pub struct Border {
    board: Layout,
    top_bar: Option<Box<dyn Widget>>,
    bot_bar: Option<Box<dyn Widget>>,
    bg: bool,
}

impl Border {
    pub fn new(board: Layout, bg: bool) -> Self {
        Self {
            board,
            top_bar: None,
            bot_bar: None,
            bg,
        }
    }

    /// Sets top bar to given [`Widget`]
    pub fn top_bar<T>(mut self, bar: T) -> Self
    where
        T: Into<Box<dyn Widget>>,
    {
        self.top_bar = Some(bar.into());
        self
    }

    /// Sets bottom bar to given [`Widget`]
    pub fn bot_bar<T>(mut self, bar: T) -> Self
    where
        T: Into<Box<dyn Widget>>,
    {
        self.bot_bar = Some(bar.into());
        self
    }
}

impl Widget for Border {
    fn render(&self, pos: &Coords, size: &Coords) {
        print!("{}", self.get_string(pos, size));
        _ = stdout().flush();
    }

    fn get_string(&self, pos: &Coords, size: &Coords) -> String {
        let brframe1 = format!(
            "{}{}▄▟{} {} \x1b[0m",
            Fg::Hex(0xbcbcbc),
            Bg::Hex(0xffffff),
            Bg::Hex(0xbcbcbc),
            Bg::Hex(0x797979)
        );
        let brframe2 = format!(
            "{}{}▄▄▄{} \x1b[0m",
            Fg::Hex(0x797979),
            Bg::Hex(0xbcbcbc),
            Bg::Hex(0x797979)
        );

        let blframe1 =
            format!("{} {}  ", Bg::Hex(0xffffff), Bg::Hex(0xbcbcbc));
        let blframe2 = format!(
            "{}{}▄{}{}▄▄",
            Bg::Hex(0xffffff),
            Fg::Hex(0xbcbcbc),
            Bg::Hex(0xbcbcbc),
            Fg::Hex(0x797979)
        );

        let lframe = format!(
            "{} {} {}{}▌",
            Bg::Hex(0xffffff),
            Bg::Hex(0xbcbcbc),
            Fg::Hex(0xbcbcbc),
            Bg::Hex(0x797979)
        );

        let tlframe1 =
            format!("{}{} ▄▄▄", Bg::Hex(0xffffff), Fg::Hex(0xbcbcbc));
        let tlframe2 = format!(
            "{} {}{} ▗▄",
            Bg::Hex(0xffffff),
            Bg::Hex(0xbcbcbc),
            Fg::Hex(0x797979)
        );

        // Horizontal borders
        let hframe = "▄".repeat(size.x - 7);

        let tframe1 =
            format!("{}{}{}", Bg::Hex(0xffffff), Fg::Hex(0xbcbcbc), &hframe);
        let tframe2 =
            format!("{}{}{}", Bg::Hex(0xbcbcbc), Fg::Hex(0x797979), &hframe);

        let bframe1 =
            format!("{}{}{}", Fg::Hex(0xbcbcbc), Bg::Hex(0xffffff), &hframe);
        let bframe2 =
            format!("{}{}{}", Fg::Hex(0x797979), Bg::Hex(0xbcbcbc), &hframe);

        let trframe1 = format!(
            "{}{}▄▄{}{}▄\x1b[0m",
            Bg::Hex(0xffffff),
            Fg::Hex(0xbcbcbc),
            Bg::Hex(0xbcbcbc),
            Fg::Hex(0x797979)
        );
        let trframe2 =
            format!("{}  {} \x1b[0m", Bg::Hex(0xbcbcbc), Bg::Hex(0x797979));

        let rframe = format!(
            "{}{}▌\x1b[0m{}{}▌ {} \x1b[0m",
            Bg::Hex(0xffffff),
            Fg::Hex(0x797979),
            Fg::Hex(0xffffff),
            Bg::Hex(0xbcbcbc),
            Bg::Hex(0x797979)
        );

        let mut res = String::new();
        for y in 0..size.y - 4 {
            if self.bg {
                res.push_str(&format!(
                    "{}{lframe}{}{}{rframe}",
                    Cursor::Pos(pos.x, pos.y + 2 + y),
                    Bg::Hex(0xbcbcbc),
                    " ".repeat(size.x - 7),
                ));
            } else {
                res.push_str(&format!(
                    "{}{lframe}{}{rframe}",
                    Cursor::Pos(pos.x, pos.y + 2 + y),
                    Cursor::Pos(pos.x + size.x - 4, pos.y + 2 + y),
                ));
            }
        }

        res.push_str(&format!(
            "{}{}{}{}",
            Cursor::Pos(pos.x, pos.y),
            tlframe1,
            tframe1,
            trframe1
        ));
        if let Some(top_bar) = &self.top_bar {
            res.push_str(&format!(
                "{}{} {}{}{} {}",
                Cursor::Pos(pos.x, pos.y + 1),
                Bg::Hex(0xffffff),
                Bg::Hex(0xbcbcbc),
                " ".repeat(size.x - 2),
                Bg::Hex(0x797979),
                Bg::Hex(0xbcbcbc),
            ));
            res.push_str(&top_bar.get_string(
                &Coords::new(pos.x + 3, pos.y + 1),
                &Coords::new(size.x - 6, 1),
            ));
        }
        res.push_str(&format!(
            "{}{}{}{}",
            Cursor::Pos(pos.x, pos.y + 1 + self.top_bar.is_some() as usize),
            tlframe2,
            tframe2,
            trframe2
        ));

        res.push_str(&format!(
            "{}{blframe1}{bframe1}{brframe1}",
            Cursor::Pos(
                pos.x,
                pos.y + size.y - 2 - self.bot_bar.is_some() as usize
            )
        ));
        if let Some(bot_bar) = &self.bot_bar {
            res.push_str(&format!(
                "{}{} {}{}{} {}",
                Cursor::Pos(pos.x, pos.y + size.y - 2),
                Bg::Hex(0xffffff),
                Bg::Hex(0xbcbcbc),
                " ".repeat(size.x - 2),
                Bg::Hex(0x797979),
                Bg::Hex(0xbcbcbc),
            ));
            res.push_str(&bot_bar.get_string(
                &Coords::new(pos.x + 3, pos.y + size.y - 2),
                &Coords::new(size.x - 6, 1),
            ));
        }
        res.push_str(&format!(
            "{}{blframe2}{bframe2}{brframe2}",
            Cursor::Pos(pos.x, pos.y + size.y - 1)
        ));

        res.push_str(&self.board.get_string(
            &Coords::new(
                pos.x + 3,
                pos.y + 2 + self.top_bar.is_some() as usize,
            ),
            &Coords::new(
                size.x.saturating_sub(7),
                size.y.saturating_sub(
                    4 + self.top_bar.is_some() as usize
                        + self.bot_bar.is_some() as usize,
                ),
            ),
        ));
        res
    }

    fn height(&self, size: &Coords) -> usize {
        self.board.height(size)
            + 4
            + self.top_bar.is_some() as usize
            + self.bot_bar.is_some() as usize
    }

    fn width(&self, size: &Coords) -> usize {
        self.board.width(size) + 7
    }
}

impl From<Border> for Box<dyn Widget> {
    fn from(value: Border) -> Self {
        Box::new(value)
    }
}
