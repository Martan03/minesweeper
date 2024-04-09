use rand::{thread_rng, Rng};
use termint::{
    geometry::{constrain::Constrain, coords::Coords},
    widgets::layout::Layout,
};

use super::cell::Cell;

/// Struct representing board
#[derive(Debug, Clone)]
pub struct Board {
    pub width: usize,
    pub height: usize,
    cells: Vec<Cell>,
    mines: usize,
    generated: bool,
    cur: Coords,
    rev: usize,
    flags: usize,
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
            cur: Coords::new(0, 0),
            rev: 0,
            flags: 0,
        }
    }

    /// Gets [`Board`] as termint Layout element
    pub fn get_element(&self, over: bool) -> Layout {
        let mut layout = Layout::vertical();
        for y in 0..self.height {
            let mut row = Layout::horizontal();
            for x in 0..self.width {
                let cell = self.cells[self.get_id(x, y)]
                    .get_element(self.cur.x == x && self.cur.y == y, over);
                row.add_child(cell, Constrain::Length(5));
            }
            layout.add_child(row, Constrain::Length(3));
        }
        layout
    }

    /// Reveals current [`Cell`] and its neighbors when 0
    pub fn reveal(&mut self) -> bool {
        if !self.generated {
            self.generate();
        }

        let id = self.get_id(self.cur.x, self.cur.y);
        if self.cells[id].is_flag() {
            return true;
        }
        if self.cells[id].is_mine() {
            return false;
        }

        if self.cells[id].is_visible() {
            return self.reveal_vis();
        } else {
            self.reveal_cell(&self.cur.clone());
        }

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
        let id = self.get_id(self.cur.x, self.cur.y);
        self.flags = self.cells[id].flag(self.flags);
    }

    /// Returns true when game is won, else false
    pub fn win(&self) -> bool {
        return self.rev + self.flags == self.width * self.height
            && self.mines == self.flags;
    }

    /// Resets the [`Board`]
    pub fn reset(&mut self) {
        self.cells = vec![Cell::new(0); self.width * self.height];
        self.generated = false;
        self.rev = 0;
        self.flags = 0;
    }

    /// Gets flags left
    pub fn flags_left(&self) -> isize {
        self.mines as isize - self.flags as isize
    }

    pub fn cur_up(&mut self) {
        self.cur.y = self.cur.y.checked_sub(1).unwrap_or(self.height - 1);
    }

    pub fn cur_down(&mut self) {
        self.cur.y += 1;
        if self.cur.y >= self.height {
            self.cur.y = 0;
        }
    }

    pub fn cur_left(&mut self) {
        self.cur.x = self.cur.x.checked_sub(1).unwrap_or(self.width - 1);
    }

    pub fn cur_right(&mut self) {
        self.cur.x += 1;
        if self.cur.x >= self.width {
            self.cur.x = 0;
        }
    }
}

// Private methods implementations
impl Board {
    /// Generates the [`Board`] - fills it with mines
    fn generate(&mut self) {
        self.generated = true;
        let mut rng = thread_rng();

        let cannot = self.get_neighbors(&self.cur);

        for _ in 0..self.mines {
            let mut x = rng.gen_range(0..self.width);
            let mut y = rng.gen_range(0..self.height);

            let mut id = self.get_id(x, y);
            while self.cells[id].get() == 0xff
                || cannot.contains(&Coords::new(x, y))
                || x == self.cur.x
                || y == self.cur.y
            {
                x = rng.gen_range(0..self.width);
                y = rng.gen_range(0..self.height);
                id = self.get_id(x, y);
            }

            self.cells[id].set(0xff);
            self.inc_neighbors(id);
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
    fn reveal_cell(&mut self, coords: &Coords) {
        let cell = &mut self.cells[coords.x + coords.y * self.width];
        if cell.is_visible() {
            return;
        }

        cell.show();
        self.rev += 1;
        if cell.get() == 0x00 {
            for n in self.get_neighbors(&coords) {
                self.reveal_cell(&n);
            }
        }
    }

    /// Reveals neighbors of visible cell
    fn reveal_vis(&mut self) -> bool {
        let mut ret = true;
        for n in self.get_neighbors(&self.cur) {
            let cell = &mut self.cells[n.x + n.y * self.width];
            if cell.is_flag() {
                continue;
            }
            if cell.is_mine() {
                ret = false;
            } else {
                self.reveal_cell(&n);
            }
        }
        ret
    }

    /// Gets cell id from given coords
    fn get_id(&self, x: usize, y: usize) -> usize {
        self.width * y + x
    }

    fn get_neighbors(&self, coords: &Coords) -> Vec<Coords> {
        let mut cells = Vec::new();
        let x = coords.x as isize;
        let y = coords.y as isize;

        self.add_neighbor(&mut cells, x - 1, y - 1);
        self.add_neighbor(&mut cells, x, y - 1);
        self.add_neighbor(&mut cells, x + 1, y - 1);
        self.add_neighbor(&mut cells, x - 1, y);
        self.add_neighbor(&mut cells, x + 1, y);
        self.add_neighbor(&mut cells, x - 1, y + 1);
        self.add_neighbor(&mut cells, x, y + 1);
        self.add_neighbor(&mut cells, x + 1, y + 1);
        cells
    }

    fn add_neighbor(&self, cells: &mut Vec<Coords>, x: isize, y: isize) {
        if (0..self.width as isize).contains(&x)
            && (0..self.height as isize).contains(&y)
        {
            cells.push(Coords::new(x as usize, y as usize));
        }
    }
}
