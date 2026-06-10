use crate::game::{
    board::{Board, COLUMNS, ROWS},
    player::Player,
};

fn horizontal(board: &Board, player: Player) -> bool {
    for row in 0..ROWS {
        for column in 0..(COLUMNS - 3) {
            let mut count = 0;

            for k in 0..4 {
                if board.grid()[row][column + k] == Some(player) {
                    count += 1;
                }
            }

            if count == 4 {
                return true;
            }
        }
    }

    false
}

fn vertical(board: &Board, player: Player) -> bool {
    for column in 0..COLUMNS {
        for row in 0..(ROWS - 3) {
            let mut count = 0;

            for k in 0..4 {
                if board.grid()[row + k][column] == Some(player) {
                    count += 1;
                }
            }

            if count == 4 {
                return true;
            }
        }
    }

    false
}

fn diagonal_up(board: &Board, player: Player) -> bool {
    for row in 3..ROWS {
        for column in 0..(COLUMNS - 3) {
            let mut count = 0;

            for k in 0..4 {
                if board.grid()[row - k][column + k] == Some(player) {
                    count += 1;
                }
            }

            if count == 4 {
                return true;
            }
        }
    }

    false
}

fn diagonal_down(board: &Board, player: Player) -> bool {
    for row in 0..(ROWS - 3) {
        for column in 0..(COLUMNS - 3) {
            let mut count = 0;

            for k in 0..4 {
                if board.grid()[row + k][column + k] == Some(player) {
                    count += 1;
                }
            }

            if count == 4 {
                return true;
            }
        }
    }

    false
}

pub fn winner(board: &Board) -> Option<Player> {
    // TODO: this could be optimized, since the winner on ConnectFour can only be the one that plays last.

    if horizontal(board, Player::Red)
        || vertical(board, Player::Red)
        || diagonal_up(board, Player::Red)
        || diagonal_down(board, Player::Red)
    {
        return Some(Player::Red);
    }

    if horizontal(board, Player::Yellow)
        || vertical(board, Player::Yellow)
        || diagonal_up(board, Player::Yellow)
        || diagonal_down(board, Player::Yellow)
    {
        return Some(Player::Yellow);
    }

    None
}
