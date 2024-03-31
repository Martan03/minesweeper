/// Represents state the game is in
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GameState {
    Playing,
    GameOver,
    Win,
}

/// Represents which game screen is currently displayed
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GameScreen {
    Game,
    Help,
}
