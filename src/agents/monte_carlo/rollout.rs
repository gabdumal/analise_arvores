use crate::game::{board::Board, game_state::GameState, player::Player};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

pub fn rollout(
    mut board: Board,
    root_player: Player,
    rollout_length: &mut usize,
    rng: &mut ChaCha8Rng,
) -> f64 {
    *rollout_length = 0;

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

        *rollout_length += 1;

        let legal = board.legal_movements();

        let movement = legal[rng.random_range(0..legal.len())];

        board = board.apply_movement(movement, None).unwrap();
    }
}
