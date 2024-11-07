/// Represents state the game is in
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GameState {
    Playing,
    GameOver,
    Win,
}

/// Represents which game screen is currently displayed
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
pub enum Screen {
    Game,
    #[default]
    Picker,
    Help,
}
