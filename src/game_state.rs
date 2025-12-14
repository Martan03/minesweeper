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

/// Represents which game screen is currently displayed
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum Screen {
    Game,
    #[default]
    DiffPicker,
    Help,
}
