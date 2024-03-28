use rand::{thread_rng, Rng};

use super::cell::Cell;

/// Struct representing board
#[derive(Debug, Clone)]
pub struct Board {
    width: usize,
    height: usize,
    cells: Vec<Cell>,
}

impl Board {
    /// Creates new [`Board`] with given size
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![Cell::new(0x00); width * height],
        }
    }

    /// Gets [`Cell`] on given position on the [`Board`]
    pub fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        self.cells.get(y * self.width + x)
    }

    /// Changes the size of the [`Board`]
    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.cells = vec![Cell::new(0x00); width * height];
    }

    /// Generates the [`Board`] - fills it with mines
    pub fn generate(&mut self, mines: usize) {
        let mut rng = thread_rng();

        let max = self.width * self.height;
        for _ in 0..mines {
            let mut rnd = rng.gen_range(0..max);
            while self.cells[rnd].get() != 0xff {
                rnd = rng.gen_range(0..max);
            }

            self.cells[rnd].set(0xff);
        }
    }
}

// Private methods implementations
impl Board {
    fn inc_near(&mut self, pos: usize) {
        if pos > self.width {
            let x = pos - self.width;
        }
    }
}
