use crate::game::player::Player;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    InProgress,
    Win(Player),
    Draw,
}
