use crate::game::{
    board::{Board, COLUMNS, ROWS},
    player::Player,
    rules::winner,
};

const WIN_SCORE: isize = 1_000_000;

const THREE_SCORE: isize = 100;
const TWO_SCORE: isize = 10;

const BLOCK_THREE_SCORE: isize = 80;

const CENTER_SCORE: isize = 6;

pub fn evaluate(board: &Board) -> isize {
    match winner(board) {
        Some(Player::Red) => return WIN_SCORE,
        Some(Player::Yellow) => return -WIN_SCORE,
        None => {}
    }

    let mut score = 0;

    score += evaluate_center(board);

    score += evaluate_horizontal(board);
    score += evaluate_vertical(board);

    score += evaluate_diagonal_down(board);
    score += evaluate_diagonal_up(board);

    score
}

fn evaluate_center(board: &Board) -> isize {
    let grid = board.grid();

    let center_column = COLUMNS / 2;

    let mut score = 0;

    for row in grid {
        match row[center_column] {
            Some(Player::Red) => {
                score += CENTER_SCORE;
            }
            Some(Player::Yellow) => {
                score -= CENTER_SCORE;
            }
            None => {}
        }
    }

    score
}

fn evaluate_window(window: [Option<Player>; 4]) -> isize {
    let mut red = 0;
    let mut yellow = 0;
    let mut empty = 0;

    for cell in window {
        match cell {
            Some(Player::Red) => red += 1,
            Some(Player::Yellow) => yellow += 1,
            None => empty += 1,
        }
    }

    if red == 4 {
        return WIN_SCORE;
    }

    if yellow == 4 {
        return -WIN_SCORE;
    }

    let mut score = 0;

    if red == 3 && empty == 1 {
        score += THREE_SCORE;
    }

    if red == 2 && empty == 2 {
        score += TWO_SCORE;
    }

    if yellow == 3 && empty == 1 {
        score -= BLOCK_THREE_SCORE;
    }

    if yellow == 2 && empty == 2 {
        score -= TWO_SCORE;
    }

    score
}

fn evaluate_horizontal(board: &Board) -> isize {
    let grid = board.grid();

    let mut score = 0;

    for row in grid {
        for column in 0..(COLUMNS - 3) {
            score += evaluate_window([
                row[column],
                row[column + 1],
                row[column + 2],
                row[column + 3],
            ]);
        }
    }

    score
}

fn evaluate_vertical(board: &Board) -> isize {
    let grid = board.grid();

    let mut score = 0;

    for row in 0..(ROWS - 3) {
        for column in 0..COLUMNS {
            score += evaluate_window([
                grid[row][column],
                grid[row + 1][column],
                grid[row + 2][column],
                grid[row + 3][column],
            ]);
        }
    }

    score
}

fn evaluate_diagonal_down(board: &Board) -> isize {
    let grid = board.grid();

    let mut score = 0;

    for row in 0..(ROWS - 3) {
        for column in 0..(COLUMNS - 3) {
            score += evaluate_window([
                grid[row][column],
                grid[row + 1][column + 1],
                grid[row + 2][column + 2],
                grid[row + 3][column + 3],
            ]);
        }
    }

    score
}

fn evaluate_diagonal_up(board: &Board) -> isize {
    let grid = board.grid();

    let mut score = 0;

    for row in 3..ROWS {
        for column in 0..(COLUMNS - 3) {
            score += evaluate_window([
                grid[row][column],
                grid[row - 1][column + 1],
                grid[row - 2][column + 2],
                grid[row - 3][column + 3],
            ]);
        }
    }

    score
}
