/// Represents state the game is in
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum GameState {
    Playing,
    GameOver,
    Win,
}
