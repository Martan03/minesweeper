use std::io::{stdout, Write};

use termint::{
    enums::{bg::Bg, cursor::Cursor, fg::Fg},
    geometry::coords::Coords,
    widgets::{layout::Layout, widget::Widget},
};

pub struct Border {
    board: Layout,
    left: String,
    win: bool,
}

impl Border {
    pub fn new(board: Layout, left: String, win: bool) -> Self {
        Self { board, left, win }
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
        res.push_str(&format!(
            "{}{}{}{}",
            Cursor::Pos(pos.x, pos.y),
            tlframe1,
            tframe1,
            trframe1
        ));
        let state = if self.win { "Victory" } else { "" };
        res.push_str(&format!(
            "{}{} {}  {}{}{}{}  {} ",
            Cursor::Pos(pos.x, pos.y + 1),
            Bg::Hex(0xffffff),
            Bg::Hex(0xbcbcbc),
            Fg::Hex(0x303030),
            self.left,
            " ".repeat(size.x - self.left.len() - state.len() - 6),
            state,
            Bg::Hex(0x797979),
        ));
        res.push_str(&format!(
            "{}{}{}{}",
            Cursor::Pos(pos.x, pos.y + 2),
            tlframe2,
            tframe2,
            trframe2
        ));

        for y in 0..size.y - 5 {
            res.push_str(&format!(
                "{}{lframe}{}{rframe}",
                Cursor::Pos(pos.x, pos.y + 3 + y),
                Cursor::Pos(pos.x + size.x - 4, pos.y + 3 + y),
            ));
        }

        res.push_str(&format!(
            "{}{blframe1}{bframe1}{brframe1}",
            Cursor::Pos(pos.x, pos.y + size.y - 3)
        ));
        res.push_str(&format!(
            "{}{} {}  {}🛈 Press i for help{}{} ",
            Cursor::Pos(pos.x, pos.y + size.y - 2),
            Bg::Hex(0xffffff),
            Bg::Hex(0xbcbcbc),
            Fg::Hex(0x303030),
            " ".repeat(size.x - 22),
            Bg::Hex(0x797979),
        ));
        res.push_str(&format!(
            "{}{blframe2}{bframe2}{brframe2}",
            Cursor::Pos(pos.x, pos.y + size.y - 1)
        ));

        res.push_str(
            &self
                .board
                .get_string(&Coords::new(pos.x + 3, pos.y + 3), size),
        );
        res
    }

    fn height(&self, size: &Coords) -> usize {
        self.board.height(size) + 6
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
