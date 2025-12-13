use termint::{
    buffer::Buffer,
    enums::{Color, Modifier},
    geometry::{Rect, Vec2},
    style::Style,
    widgets::{cache::Cache, Element, Widget},
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

    /// Increments [`Cell`] value by one
    pub fn inc(&mut self) {
        self.value = self.value.saturating_add(1);
    }

    /// Gets [`Cell`] value
    pub fn get(&self) -> u8 {
        self.value
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

    /// Gets the corresponding cell element
    pub fn element(&self) -> Element {
        match self.cell_type {
            CellType::Visible => self.clone().into(),
            _ => self.get_hidden(self.sel).into(),
        }
    }
}

impl Widget for Cell {
    fn render(&self, buffer: &mut Buffer, rect: Rect, _cache: &mut Cache) {
        self.render_visible(buffer, rect);
    }

    fn height(&self, _size: &Vec2) -> usize {
        3
    }

    fn width(&self, _size: &Vec2) -> usize {
        7
    }

    fn children(&self) -> Vec<&Element> {
        vec![]
    }
}

impl Cell {
    fn render_visible(&self, buffer: &mut Buffer, rect: Rect) {
        let lb = Color::Hex(0x797979);
        let db = match self.sel {
            true if self.value == 0xfe => Color::Hex(0xd20000),
            true => Color::Hex(0xa0a0a0),
            false if self.value == 0xfe => Color::Hex(0xee0000),
            false => Color::Hex(0xbcbcbc),
        };

        let mut pos = *rect.pos();
        buffer.set_str_styled(" ▆▆▆▆▆", &pos, Style::new().bg(lb).fg(db));

        pos.y += 1;
        let (val, fg) = self.get_value();
        buffer.set_str_styled(
            format!("   {val} "),
            &pos,
            Style::new().bg(db).fg(fg),
        );
        buffer.set_bg(lb, &pos);

        pos.y += 1;
        buffer.set_str_styled(" ▂▂▂▂▂", &pos, Style::new().bg(db).fg(lb));
        buffer.set_bg(lb, &pos);
    }

    fn get_hidden(&self, sel: bool) -> Button {
        let text = match self.cell_type {
            CellType::Flag => RawSpan::new(" ▶ ").fg(Color::Hex(0xff0000)),
            CellType::WrongFlag => RawSpan::new(" ▶ ")
                .modifier(Modifier::STRIKED)
                .fg(Color::Hex(0xff0000)),
            _ => RawSpan::new("   "),
        };
        Button::new(text).selected(sel)
    }

    fn get_value(&self) -> (&str, Color) {
        match self.value {
            0x01 => ("1 ", Color::Hex(0x0000ff)),
            0x02 => ("2 ", Color::Hex(0x007700)),
            0x03 => ("3 ", Color::Hex(0xff0000)),
            0x04 => ("4 ", Color::Hex(0x000077)),
            0x05 => ("5 ", Color::Hex(0x770000)),
            0x06 => ("6 ", Color::Hex(0x007777)),
            0x07 => ("7 ", Color::Hex(0x000000)),
            0x08 => ("8 ", Color::Hex(0x777777)),
            0xfe | 0xff => ("💣", Color::Default),
            _ => ("  ", Color::Default),
        }
    }
}

impl From<Cell> for Element {
    fn from(value: Cell) -> Self {
        Element::new(value)
    }
}

impl From<Cell> for Box<dyn Widget> {
    fn from(value: Cell) -> Self {
        Box::new(value)
    }
}
