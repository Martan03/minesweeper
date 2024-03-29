use rand::{thread_rng, Rng};
use termint::{geometry::constrain::Constrain, widgets::layout::Layout};

use super::cell::Cell;

/// Struct representing board
#[derive(Debug, Clone)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    cells: Vec<Cell>,
    mines: usize,
    generated: bool,
    cur: (usize, usize),
}

impl Board {
    /// Creates new [`Board`] with given size
    pub fn new(width: usize, height: usize, mines: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![Cell::new(0x00); width * height],
            mines,
            generated: false,
            cur: (0, 0),
        }
    }

    /// Gets [`Board`] as termint Layout element
    pub fn get_element(&self) -> Layout {
        let mut layout = Layout::horizontal().center();
        layout.add_child(
            self.get_cells_layout(),
            Constrain::Length(5 * self.width),
        );
        layout
    }

    /// Changes the size of the [`Board`]
    pub fn resize(&mut self, width: usize, height: usize) {
        self.width = width;
        self.height = height;
        self.cells = vec![Cell::new(0x00); width * height];
    }

    /// Reveals current cell and its neighbors when 0
    pub fn reveal(&mut self) -> bool {
        if !self.generated {
            self.generated = true;
            self.generate();
        }

        if self.cells[self.cur_id()].is_flag() {
            return true;
        }

        if self.cells[self.cur_id()].is_mine() {
            return false;
        }

        self.reveal_cell(self.cur.0 as isize, self.cur.1 as isize);

        true
    }

    /// Reveals all mines
    pub fn reveal_mines(&mut self) {
        for i in 0..self.cells.len() {
            if self.cells[i].is_mine() {
                self.cells[i].show();
            }
        }
    }

    /// Flags current [`Cell`]
    pub fn flag(&mut self) {
        self.cells[self.cur.0 + self.cur.1 * self.width].flag();
    }

    pub fn cur_up(&mut self) {
        self.cur.1 = self.cur.1.checked_sub(1).unwrap_or(self.height - 1);
    }

    pub fn cur_down(&mut self) {
        self.cur.1 += 1;
        if self.cur.1 >= self.height {
            self.cur.1 = 0;
        }
    }

    pub fn cur_left(&mut self) {
        self.cur.0 = self.cur.0.checked_sub(1).unwrap_or(self.width - 1);
    }

    pub fn cur_right(&mut self) {
        self.cur.0 += 1;
        if self.cur.0 >= self.height {
            self.cur.0 = 0;
        }
    }

    /// Resets the [`Board`]
    pub fn reset(&mut self) {
        self.cells = vec![Cell::new(0); self.width * self.height];
        self.generated = false;
    }
}

// Private methods implementations
impl Board {
    fn get_cells_layout(&self) -> Layout {
        let mut layout = Layout::vertical();
        for y in 0..self.height {
            let mut row = Layout::horizontal();
            for x in 0..self.width {
                let cell = if self.cur.0 == x && self.cur.1 == y {
                    self.cells[x + y * self.width].get_element_act()
                } else {
                    self.cells[x + y * self.width].get_element()
                };
                row.add_child(cell, Constrain::Length(5));
            }
            layout.add_child(row, Constrain::Length(3));
        }

        layout
    }

    /// Generates the [`Board`] - fills it with mines
    fn generate(&mut self) {
        let mut rng = thread_rng();

        let cur_id = self.cur.0 + self.cur.1 * self.width;
        let cannot = self.get_neighbors(cur_id as isize);

        let max = self.width * self.height;
        for _ in 0..self.mines {
            let mut rnd = rng.gen_range(0..max);
            while self.cells[rnd].get() == 0xff
                || cannot.contains(&(rnd as isize))
                || rnd == cur_id
            {
                rnd = rng.gen_range(0..max);
            }

            self.cells[rnd].set(0xff);
            self.inc_neighbors(rnd);
        }
    }

    /// Increments value of cell neighbors
    fn inc_neighbors(&mut self, pos: usize) {
        let x = (pos % self.width) as isize;
        let y = (pos / self.width) as isize;

        self.inc_hor_neighbors(x, y - 1);
        self.inc_hor_neighbors(x, y);
        self.inc_hor_neighbors(x, y + 1);
    }

    /// Increments value of cell horizontal neighbors
    fn inc_hor_neighbors(&mut self, x: isize, y: isize) {
        if y >= 0 && y < self.height as isize {
            self.inc_cell(x - 1, y);
            self.inc_cell(x, y);
            self.inc_cell(x + 1, y);
        }
    }

    /// Increments cell value
    fn inc_cell(&mut self, x: isize, y: isize) {
        let width = self.width as isize;
        if x >= 0 && x < width {
            self.cells[(y * width + x) as usize].inc();
        }
    }

    /// Reveals cell and its neighbors, when the cell value is 0
    fn reveal_cell(&mut self, x: isize, y: isize) {
        if !(0..self.width as isize).contains(&x)
            || !(0..self.height as isize).contains(&y)
        {
            return;
        }

        let cell = &mut self.cells[x as usize + y as usize * self.width];
        if cell.is_visible() {
            return;
        }

        cell.show();
        if cell.get() == 0x00 {
            self.reveal_cell(x - 1, y - 1);
            self.reveal_cell(x, y - 1);
            self.reveal_cell(x + 1, y - 1);
            self.reveal_cell(x - 1, y);
            self.reveal_cell(x + 1, y);
            self.reveal_cell(x - 1, y + 1);
            self.reveal_cell(x, y + 1);
            self.reveal_cell(x + 1, y + 1);
        }
    }

    /// Gets current id
    fn cur_id(&self) -> usize {
        self.width * self.cur.1 + self.cur.0
    }

    /// Gets all neighbors of given value
    fn get_neighbors(&self, val: isize) -> [isize; 8] {
        [
            val + 1,
            val - 1,
            val - self.width as isize,
            val - self.width as isize - 1,
            val - self.width as isize + 1,
            val + self.width as isize - 1,
            val + self.width as isize,
            val + self.width as isize + 1,
        ]
    }
}
