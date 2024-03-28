/// Struct representing cell in board
#[derive(Debug, Clone)]
pub struct Cell {
    value: u8,
    visible: bool,
}

impl Cell {
    /// Creates new hidden [`Cell`] with given value
    pub fn new(value: u8) -> Self {
        Self {
            value,
            visible: false,
        }
    }

    /// Sets [`Cell`] as visible
    pub fn show(&mut self) {
        self.visible = true;
    }

    /// Gets [`Cell`] visibility
    pub fn visible(&self) -> bool {
        self.visible
    }
}
