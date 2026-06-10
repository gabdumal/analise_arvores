use crate::game::{board::Board, game_state::GameState};
use rand::prelude::*;

pub fn rollout(mut board: Board) -> f64 {
    let mut rng = rand::rng();
    let root_player = board.current_player();

    loop {
        match board.game_state() {
            GameState::Win(player) => {
                return if player == root_player { 1.0 } else { 0.0 };
            }

            GameState::Draw => {
                return 0.5;
            }

            GameState::InProgress => {}
        }

        let legal = board.legal_movements();

        let movement = legal[rng.random_range(0..legal.len())];

        board = board.apply_movement(movement, None).unwrap();
    }
}
