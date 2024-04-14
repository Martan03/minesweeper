use termint::{
    enums::{bg::Bg, fg::Fg, modifier::Modifier},
    geometry::constrain::Constrain,
    widgets::{layout::Layout, widget::Widget},
};

use crate::tui::{raw_span::RawSpan, widgets::button::Button};

/// Enum representing cell type
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CellType {
    Hidden,
    Visible,
    Flag,
    WrongFlag,
}

/// Struct representing cell in board
#[derive(Debug, Clone)]
pub struct Cell {
    pub value: u8,
    pub cell_type: CellType,
    pub sel: bool,
}

impl Cell {
    /// Creates new hidden [`Cell`] with given value
    pub fn new(value: u8) -> Self {
        Self {
            value,
            cell_type: CellType::Hidden,
            sel: false,
        }
    }

    /// Sets [`Cell`] value to given value
    pub fn set(&mut self, value: u8) {
        self.value = value;
    }

    /// Incrementes [`Cell`] value by one
    pub fn inc(&mut self) {
        self.value = self.value.saturating_add(1);
    }

    /// Gets [`Cell`] value
    pub fn get(&self) -> u8 {
        self.value
    }

    pub fn get_element(&self, sel: bool) -> Box<dyn Widget> {
        match self.cell_type {
            CellType::Visible => self.get_visible(),
            _ => self.get_hidden(sel),
        }
    }

    /// Sets [`Cell`] as visible (if possible)
    pub fn show(&mut self) {
        if self.cell_type != CellType::Flag {
            self.cell_type = CellType::Visible;
        }
    }

    /// Toggles [`Cell`] as flag (if possible)
    pub fn flag(&mut self, flags: usize) -> usize {
        if self.cell_type == CellType::Flag {
            self.cell_type = CellType::Hidden;
            return flags - 1;
        } else if self.cell_type == CellType::Hidden {
            self.cell_type = CellType::Flag;
            return flags + 1;
        }
        flags
    }

    /// Toggles whether [`Cell`] is selected
    pub fn sel(&mut self) {
        self.sel = !self.sel;
    }

    /// Checks whether cell is mine
    pub fn is_mine(&self) -> bool {
        self.value == 0xff || self.value == 0xfe
    }

    /// Checks whether cell is revealed
    pub fn is_visible(&self) -> bool {
        self.cell_type == CellType::Visible
    }

    /// Checks whether cell is flag
    pub fn is_flag(&self) -> bool {
        self.cell_type == CellType::Flag
    }

    fn get_visible(&self) -> Box<dyn Widget> {
        let lb = 0x797979;
        let db = match self.sel {
            true if self.value == 0xfe => 0xd20000,
            true => 0xa0a0a0,
            false if self.value == 0xfe => 0xee0000,
            false => 0xbcbcbc,
        };

        let mut vis = Layout::vertical();
        vis.add_child(
            RawSpan::new(" ▆▆▆▆▆").fg(Fg::Hex(db)).bg(Bg::Hex(lb)),
            Constrain::Length(1),
        );
        vis.add_child(
            RawSpan::new(format!(" {}  {} ", Bg::Hex(db), self.get_value()))
                .bg(Bg::Hex(lb)),
            Constrain::Length(1),
        );
        vis.add_child(
            RawSpan::new(format!(" {}▂▂▂▂▂", Bg::Hex(db)))
                .bg(Bg::Hex(lb))
                .fg(Fg::Hex(lb)),
            Constrain::Length(1),
        );
        vis.into()
    }

    fn get_hidden(&self, sel: bool) -> Box<dyn Widget> {
        Button::new(match self.cell_type {
            CellType::Flag => RawSpan::new(" ▶ ").fg(Fg::Hex(0xff0000)),
            CellType::WrongFlag => RawSpan::new(" ▶ ")
                .modifier(Modifier::Strike)
                .fg(Fg::Hex(0xff0000)),
            _ => RawSpan::new("   "),
        })
        .selected(sel)
        .into()
    }

    fn get_value(&self) -> String {
        match self.value {
            0x01 => format!("{}{}1 ", Modifier::Bold, Fg::Hex(0x0000ff)),
            0x02 => format!("{}{}2 ", Modifier::Bold, Fg::Hex(0x007700)),
            0x03 => format!("{}{}3 ", Modifier::Bold, Fg::Hex(0xff0000)),
            0x04 => format!("{}{}4 ", Modifier::Bold, Fg::Hex(0x000077)),
            0x05 => format!("{}{}5 ", Modifier::Bold, Fg::Hex(0x770000)),
            0x06 => format!("{}{}6 ", Modifier::Bold, Fg::Hex(0x007777)),
            0x07 => format!("{}{}7 ", Modifier::Bold, Fg::Hex(0x000000)),
            0x08 => format!("{}{}8 ", Modifier::Bold, Fg::Hex(0x777777)),
            0xfe | 0xff => "💣".to_string(),
            _ => "  ".to_string(),
        }
    }
}
