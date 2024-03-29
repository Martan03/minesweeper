use termint::{
    enums::{fg::Fg, wrap::Wrap},
    geometry::{constrain::Constrain, direction::Direction},
    widgets::{
        block::Block,
        span::{Span, StrSpanExtension},
    },
};

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
    pub fn flag(&mut self) {
        if self.cell_type == CellType::Flag {
            self.cell_type = CellType::Hidden;
        } else if self.cell_type != CellType::Visible {
            self.cell_type = CellType::Flag;
        }
    }

    /// Gets [`Cell`] termint element
    pub fn get_element(&self) -> Block {
        let mut block = Block::new().direction(Direction::Horizontal).center();
        match self.cell_type {
            CellType::Hidden => {}
            CellType::Visible => {
                block.add_child(self.get_element_vis(), Constrain::Min(0))
            }
            // 🚩
            CellType::Flag => block.add_child("F", Constrain::Min(0)),
        }
        block
    }

    /// Gets [`Cell`] as active termint element
    pub fn get_element_act(&self) -> Block {
        let mut block = Block::new()
            .direction(Direction::Horizontal)
            .border_color(Fg::Cyan)
            .center();
        match self.cell_type {
            CellType::Hidden => {}
            CellType::Visible => {
                block.add_child(self.get_element_vis(), Constrain::Min(0))
            }
            CellType::Flag => block.add_child("F", Constrain::Min(0)),
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
}

impl Cell {
    pub fn get_element_vis(&self) -> Span {
        match self.value {
            0x01 => "1".fg(Fg::RGB(4, 59, 239)),
            0x02 => "2".fg(Fg::RGB(32, 145, 4)),
            0x03 => "3".fg(Fg::RGB(252, 25, 29)),
            0x04 => "4".fg(Fg::RGB(0, 6, 124)),
            0x05 => "5".fg(Fg::RGB(140, 4, 6)),
            0x06 => "6".fg(Fg::RGB(13, 125, 153)),
            0x07 => "7".fg(Fg::RGB(0, 0, 0)),
            0x08 => "8".fg(Fg::RGB(180, 180, 180)),
            // 💣
            0xff => "F".fg(Fg::Red).wrap(Wrap::Letter),
            _ => "".to_span(),
        }
    }
}
