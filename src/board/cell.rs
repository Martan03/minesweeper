use termint::{
    enums::fg::Fg,
    geometry::{constrain::Constrain, direction::Direction},
    widgets::block::Block,
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

    /// Sets [`Cell`] as visible
    pub fn show(&mut self) {
        self.cell_type = CellType::Visible;
    }

    /// Gets [`Cell`] termint element
    pub fn get_element(&self) -> Block {
        let mut block = Block::new().direction(Direction::Horizontal).center();
        match self.cell_type {
            CellType::Hidden => {}
            CellType::Visible => {
                block.add_child(format!("{}", self.value), Constrain::Min(0))
            }
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
                block.add_child(format!("{}", self.value), Constrain::Min(0))
            }
            CellType::Flag => block.add_child("F", Constrain::Min(0)),
        }
        block
    }
}
