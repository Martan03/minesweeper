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
        self.visible = true;
    }

    /// Gets [`Cell`] visibility
    pub fn visible(&self) -> bool {
        self.visible
    }
}
