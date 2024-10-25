use std::ops::{Index, IndexMut};

use rand::{thread_rng, Rng};
use termint::{
    buffer::Buffer,
    geometry::{Rect, Vec2},
    widgets::{Element, Grid, Widget},
};

use super::cell::{Cell, CellType};

/// Struct representing board
#[derive(Debug, Clone)]
pub struct Board {
    pub cells: Vec<Cell>,
    pub size: Vec2<usize>,
    pub mines: usize,
    generated: bool,
    pub cur: Vec2,
    rev: usize,
    flags: usize,
}

impl Board {
    /// Creates new [`Board`] with given size
    pub fn new(size: Vec2, mines: usize) -> Self {
        let mut cells = vec![Cell::new(0x00); size.x * size.y];
        if size.x > 0 && size.y > 0 {
            cells[0].sel();
        }
        Self {
            size,
            cells,
            mines,
            generated: false,
            cur: Vec2::new(0, 0),
            rev: 0,
            flags: 0,
        }
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
            self.cells[id].set(0xfe);
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
        for cell in &mut self.cells {
            if cell.is_mine() {
                cell.show();
            }
            if !cell.is_mine() && cell.is_flag() {
                cell.cell_type = CellType::WrongFlag;
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
        self.rev + self.flags == self.size.x * self.size.y
            && self.mines == self.flags
    }

    /// Resets the [`Board`]
    pub fn reset(&mut self) {
        self.cells = vec![Cell::new(0); self.size.x * self.size.y];
        self.cells[self.cur.x + self.cur.y * self.size.x].sel();
        self.generated = false;
        self.rev = 0;
        self.flags = 0;
    }

    /// Gets flags left
    pub fn flags_left(&self) -> isize {
        self.mines as isize - self.flags as isize
    }

    pub fn cur_up(&mut self) {
        self.cells[self.cur.x + self.cur.y * self.size.x].sel();
        self.cur.y = self.cur.y.checked_sub(1).unwrap_or(self.size.y - 1);
        self.cells[self.cur.x + self.cur.y * self.size.x].sel();
    }

    pub fn cur_down(&mut self) {
        self.cells[self.cur.x + self.cur.y * self.size.x].sel();
        self.cur.y += 1;
        if self.cur.y >= self.size.y {
            self.cur.y = 0;
        }
        self.cells[self.cur.x + self.cur.y * self.size.x].sel();
    }

    pub fn cur_left(&mut self) {
        self.cells[self.cur.x + self.cur.y * self.size.x].sel();
        self.cur.x = self.cur.x.checked_sub(1).unwrap_or(self.size.x - 1);
        self.cells[self.cur.x + self.cur.y * self.size.x].sel();
    }

    pub fn cur_right(&mut self) {
        self.cells[self.cur.x + self.cur.y * self.size.x].sel();
        self.cur.x += 1;
        if self.cur.x >= self.size.x {
            self.cur.x = 0;
        }
        self.cells[self.cur.x + self.cur.y * self.size.x].sel();
    }
}

impl Widget for Board {
    fn render(&self, buffer: &mut Buffer) {
        let mut grid = Grid::new(vec![6; self.size.x], vec![3; self.size.y]);
        for pos in Rect::new(0, 0, self.size.x, self.size.y) {
            grid.push(self[pos].clone(), pos.x, pos.y);
        }
        grid.render(buffer);
    }

    fn height(&self, _size: &Vec2) -> usize {
        self.size.y * 3
    }

    fn width(&self, _size: &Vec2) -> usize {
        self.size.x * 6
    }
}

// Private methods implementations
impl Board {
    /// Generates the [`Board`] - fills it with mines
    fn generate(&mut self) {
        self.generated = true;
        let mut rng = thread_rng();

        let mut cannot = self.get_neighbors(&self.cur);
        cannot.push(Vec2::new(self.cur.x, self.cur.y));

        for _ in 0..self.mines {
            let mut x = rng.gen_range(0..self.size.x);
            let mut y = rng.gen_range(0..self.size.y);

            let mut id = self.get_id(x, y);
            while self.cells[id].get() == 0xff
                || cannot.contains(&Vec2::new(x, y))
            {
                x = rng.gen_range(0..self.size.x);
                y = rng.gen_range(0..self.size.y);
                id = self.get_id(x, y);
            }

            self.cells[id].set(0xff);
            self.inc_neighbors(id);
        }
    }

    /// Increments value of cell neighbors
    fn inc_neighbors(&mut self, pos: usize) {
        let x = (pos % self.size.x) as isize;
        let y = (pos / self.size.y) as isize;

        self.inc_hor_neighbors(x, y - 1);
        self.inc_hor_neighbors(x, y);
        self.inc_hor_neighbors(x, y + 1);
    }

    /// Increments value of cell horizontal neighbors
    fn inc_hor_neighbors(&mut self, x: isize, y: isize) {
        if y >= 0 && y < self.size.y as isize {
            self.inc_cell(x - 1, y);
            self.inc_cell(x, y);
            self.inc_cell(x + 1, y);
        }
    }

    /// Increments cell value
    fn inc_cell(&mut self, x: isize, y: isize) {
        let width = self.size.x as isize;
        if x >= 0 && x < width {
            self.cells[(y * width + x) as usize].inc();
        }
    }

    /// Reveals cell and its neighbors, when the cell value is 0
    fn reveal_cell(&mut self, coords: &Vec2) {
        let cell = &mut self.cells[coords.x + coords.y * self.size.x];
        if cell.is_visible() || cell.is_flag() {
            return;
        }

        cell.show();
        self.rev += 1;
        if cell.get() == 0x00 {
            for n in self.get_neighbors(coords) {
                self.reveal_cell(&n);
            }
        }
    }

    /// Reveals neighbors of visible cell
    fn reveal_vis(&mut self) -> bool {
        let mut ret = true;
        for n in self.get_neighbors(&self.cur) {
            let cell = &mut self.cells[n.x + n.y * self.size.x];
            if cell.is_flag() {
                continue;
            }
            if cell.is_mine() {
                cell.set(0xfe);
                ret = false;
            } else {
                self.reveal_cell(&n);
            }
        }
        ret
    }

    /// Gets cell id from given coords
    pub fn get_id(&self, x: usize, y: usize) -> usize {
        self.size.x * y + x
    }

    fn get_neighbors(&self, coords: &Vec2) -> Vec<Vec2> {
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

    fn add_neighbor(&self, cells: &mut Vec<Vec2>, x: isize, y: isize) {
        if (0..self.size.x as isize).contains(&x)
            && (0..self.size.y as isize).contains(&y)
        {
            cells.push(Vec2::new(x as usize, y as usize));
        }
    }
}

impl Index<usize> for Board {
    type Output = Cell;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cells[index]
    }
}

impl Index<Vec2> for Board {
    type Output = Cell;

    fn index(&self, pos: Vec2) -> &Self::Output {
        &self.cells[pos.x + pos.y * self.size.y]
    }
}

impl IndexMut<usize> for Board {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.cells[index]
    }
}

impl IndexMut<Vec2> for Board {
    fn index_mut(&mut self, pos: Vec2) -> &mut Self::Output {
        &mut self.cells[pos.x + pos.y * self.size.y]
    }
}

impl From<Board> for Element {
    fn from(value: Board) -> Self {
        Element::new(value)
    }
}

impl From<Board> for Box<dyn Widget> {
    fn from(value: Board) -> Self {
        Box::new(value)
    }
}
