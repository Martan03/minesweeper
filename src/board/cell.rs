use termint::{
    enums::fg::Fg,
    geometry::{constrain::Constrain, direction::Direction},
    widgets::{block::Block, border::BorderType},
};

use crate::tui::raw_span::RawSpan;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CellType {
    Hidden,
    Visible,
    Flag,
}

/// Struct representing cell in board
#[derive(Debug, Clone)]
pub struct Cell {
    value: u8,
    cell_type: CellType,
}

impl Cell {
    /// Creates new hidden [`Cell`] with given value
    pub fn new(value: u8) -> Self {
        Self {
            value,
            cell_type: CellType::Hidden,
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

    /// Sets [`Cell`] as visible (if possible)
    pub fn show(&mut self) {
        if self.cell_type != CellType::Flag {
            self.cell_type = CellType::Visible;
        }
    }

    /// Sets [`Cell`] as flag (if possible)
    pub fn flag(&mut self, flags: usize) -> usize {
        if self.cell_type == CellType::Flag {
            self.cell_type = CellType::Hidden;
            return flags - 1;
        } else if self.cell_type != CellType::Visible {
            self.cell_type = CellType::Flag;
            return flags + 1;
        }
        flags
    }

    /// Gets [`Cell`] termint element
    pub fn get_element(&self) -> Block {
        let mut block = Block::new()
            .direction(Direction::Horizontal)
            .center()
            .border_color(Fg::Gray);
        match self.cell_type {
            CellType::Hidden => {}
            CellType::Visible => {
                block = block.border_color(Fg::Default);
                block.add_child(self.get_element_vis(), Constrain::Min(0))
            }
            CellType::Flag => {
                block.add_child(RawSpan::new("🚩"), Constrain::Min(0))
            }
        }
        block
    }

    /// Gets [`Cell`] as active termint element
    pub fn get_element_act(&self) -> Block {
        let mut block = Block::new()
            .direction(Direction::Horizontal)
            .border_type(BorderType::Thicker)
            .border_color(Fg::Gray)
            .center();
        match self.cell_type {
            CellType::Hidden => {}
            CellType::Visible => {
                block = block.border_color(Fg::Default);
                block.add_child(self.get_element_vis(), Constrain::Min(0))
            }
            CellType::Flag => {
                block.add_child(RawSpan::new("🚩"), Constrain::Min(0))
            }
        }
        block
    }

    /// Checks whether cell is mine
    pub fn is_mine(&self) -> bool {
        self.value == 0xff
    }

    /// Checks whether cell is revealed
    pub fn is_visible(&self) -> bool {
        self.cell_type == CellType::Visible
    }

    /// Checks whether cell is flag
    pub fn is_flag(&self) -> bool {
        self.cell_type == CellType::Flag
    }
}

impl Cell {
    pub fn get_element_vis(&self) -> RawSpan {
        match self.value {
            0x01 => RawSpan::new("1").fg(Fg::RGB(4, 59, 239)),
            0x02 => RawSpan::new("2").fg(Fg::RGB(32, 145, 4)),
            0x03 => RawSpan::new("3").fg(Fg::RGB(252, 25, 29)),
            0x04 => RawSpan::new("4").fg(Fg::RGB(0, 6, 124)),
            0x05 => RawSpan::new("5").fg(Fg::RGB(140, 4, 6)),
            0x06 => RawSpan::new("6").fg(Fg::RGB(13, 125, 153)),
            0x07 => RawSpan::new("7").fg(Fg::RGB(0, 0, 0)),
            0x08 => RawSpan::new("8").fg(Fg::RGB(180, 180, 180)),
            0xff => RawSpan::new("💣"),
            _ => RawSpan::new(""),
        }
    }
}
