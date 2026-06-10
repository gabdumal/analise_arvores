use crate::game::{
    board::{Board, COLUMNS},
    player::Player,
};

pub mod exporter;
pub mod minimax_alpha_beta;
pub mod minimax_alpha_beta_with_transposition_table;
pub mod monte_carlo;

#[derive(Default)]
pub struct GraphvizConfig {
    pub enabled: bool,
    pub max_nodes: usize,
}

pub fn to_ascii(board: &Board) -> String {
    let grid = board.grid();

    let mut text = String::new();

    for row in grid {
        for column in 0..COLUMNS {
            let cell = match row[column] {
                Some(Player::Red) => "🔴",
                Some(Player::Yellow) => "🟡",
                None => "⚪",
            };

            text.push_str(cell);

            if column + 1 < COLUMNS {
                text.push(' ');
            }
        }

        text.push('\n');
    }

    text
}
