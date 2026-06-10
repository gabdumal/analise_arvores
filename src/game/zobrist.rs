use crate::game::{
    board::{Board, COLUMNS, ROWS},
    movement::Movement,
};
use rand::{RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;
use std::collections::HashSet;

pub struct ZobristTable {
    pub pieces: [[[u64; 2]; ROWS]; COLUMNS],
    pub side_to_move: u64,
}

impl ZobristTable {
    pub fn new() -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(42);

        let mut pieces = [[[0u64; 2]; ROWS]; COLUMNS];

        for column in 0..COLUMNS {
            for row in 0..ROWS {
                for player in 0..2 {
                    pieces[column][row][player] = rng.random();
                }
            }
        }

        Self {
            pieces,
            side_to_move: rng.random(),
        }
    }
}

#[test]
fn same_position_should_have_same_hash() {
    let zobrist = ZobristTable::new();

    let board1 = Board::new()
        .apply_movement(Movement::new(3), Some(&zobrist))
        .unwrap();

    let board2 = Board::new()
        .apply_movement(Movement::new(3), Some(&zobrist))
        .unwrap();

    assert_eq!(board1.zobrist_hash(), board2.zobrist_hash());
}

#[test]
fn different_positions_should_have_different_hashes() {
    let zobrist = ZobristTable::new();

    let board1 = Board::new()
        .apply_movement(Movement::new(3), Some(&zobrist))
        .unwrap();

    let board2 = Board::new()
        .apply_movement(Movement::new(4), Some(&zobrist))
        .unwrap();

    assert_ne!(board1.zobrist_hash(), board2.zobrist_hash());
}

#[test]
fn side_to_move_should_change_hash() {
    let zobrist = ZobristTable::new();

    let board1 = Board::new();

    let board2 = board1
        .apply_movement(Movement::new(0), Some(&zobrist))
        .unwrap();

    assert_ne!(board1.zobrist_hash(), board2.zobrist_hash());
}

#[test]
fn should_not_collide_in_small_sample() {
    let zobrist = ZobristTable::new();

    let mut hashes = HashSet::new();

    for column in 0..7 {
        let board = Board::new()
            .apply_movement(Movement::new(column), Some(&zobrist))
            .unwrap();

        assert!(hashes.insert(board.zobrist_hash()));
    }
}
