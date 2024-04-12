/// Enum representing cell type
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CellType {
    Hidden,
    Visible,
    Flag,
}

/// Struct representing cell in board
#[derive(Debug, Clone)]
pub struct Cell {
    pub value: u8,
    pub cell_type: CellType,
    pub sel: bool,
    wrong: bool,
}

impl Cell {
    /// Creates new hidden [`Cell`] with given value
    pub fn new(value: u8) -> Self {
        Self {
            value,
            cell_type: CellType::Hidden,
            sel: false,
            wrong: false,
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

    pub fn is_wrong(&self) -> bool {
        self.wrong
    }

    pub fn set_wrong(&mut self) {
        self.wrong = true;
    }
}
