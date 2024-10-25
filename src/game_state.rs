/// Represents state the game is in
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GameState {
    Playing,
    GameOver,
    Win,
}

impl GameState {
    pub fn is_playing(&self) -> bool {
        *self == Self::Playing
    }
}
