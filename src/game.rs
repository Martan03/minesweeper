use crate::board::board::Board;

/// Struct containing game info and implementing the game loop
pub struct Game {
    board: Board,
}

impl Game {
    /// Creates new [`Game`] struct
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            board: Board::new(width, height),
        }
    }
}
