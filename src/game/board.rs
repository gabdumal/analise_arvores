use crate::game::{
    movement::Movement, player::Player, rules::winner, state::GameState, zobrist::ZobristTable,
};
use std::array::repeat;

pub const ROWS: usize = 6;
pub const COLUMNS: usize = 7;
pub const BOARD_SIZE: usize = ROWS * COLUMNS;

type Grid = [[Option<Player>; COLUMNS]; ROWS];

#[derive(Clone)]
pub struct Board {
    grid: Grid,
    current_player: Player,
    movements_played: usize,
    zobrist_hash: u64,
}

impl Board {
    fn empty() -> Self {
        Self {
            grid: [[None; COLUMNS]; ROWS],
            current_player: Player::Red,
            movements_played: 0,
            zobrist_hash: 0,
        }
    }

    pub fn new() -> Self {
        Self::empty()
    }

    pub fn grid(&self) -> Grid {
        self.grid
    }

    pub fn current_player(&self) -> Player {
        self.current_player
    }

    pub fn movements_played(&self) -> usize {
        self.movements_played
    }

    pub fn zobrist_hash(&self) -> u64 {
        self.zobrist_hash
    }

    pub fn is_column_full(&self, column: usize) -> bool {
        self.grid[0][column].is_some()
    }

    pub fn legal_movements(&self) -> Vec<Movement> {
        (0..COLUMNS)
            .filter(|&c| !self.is_column_full(c))
            .map(Movement::new)
            .collect()
    }

    fn next_free_row(&self, column: usize) -> Option<usize> {
        (0..ROWS)
            .rev()
            .find(|&row| self.grid[row][column].is_none())
    }

    pub fn apply_movement(&self, movement: Movement, zobrist: &ZobristTable) -> Option<Self> {
        let row = self.next_free_row(movement.column)?;

        let mut next_board = self.clone();

        next_board.grid[row][movement.column] = Some(self.current_player);

        let player_index = match self.current_player {
            Player::Red => 0,
            Player::Yellow => 1,
        };

        next_board.zobrist_hash ^= zobrist.pieces[movement.column][row][player_index];

        next_board.zobrist_hash ^= zobrist.side_to_move;

        next_board.current_player = self.current_player.opponent();

        next_board.movements_played += 1;

        Some(next_board)
    }

    pub fn game_state(&self) -> GameState {
        match winner(self) {
            Some(winner) => GameState::Win(winner),
            None => {
                if self.movements_played == BOARD_SIZE {
                    GameState::Draw
                } else {
                    GameState::InProgress
                }
            }
        }
    }

    pub fn evaluate(&self) -> isize {
        match winner(self) {
            Some(Player::Red) => 1_000_000,
            Some(Player::Yellow) => -1_000_000,
            None => 0,
        }
    }
}

#[test]
fn should_create_empty_board() {
    let board = Board::new();
    assert_eq!(board.grid(), repeat(repeat(None)));
    assert_eq!(board.movements_played(), 0);
    assert_eq!(board.current_player(), Player::Red);
    assert_eq!(board.game_state(), GameState::InProgress);
}

#[test]
fn should_generate_seven_legal_movements() {
    let board = Board::new();
    let movements = board.legal_movements();
    assert_eq!(movements.len(), 7);
}

#[test]
fn should_drop_piece_at_bottom() {
    let zobrist = ZobristTable::new();
    let board = Board::new();

    let board = board.apply_movement(Movement::new(3), &zobrist).unwrap();
    assert_eq!(board.grid()[5][3], Some(Player::Red));
}

#[test]
fn should_stack_pieces() {
    let zobrist = ZobristTable::new();
    let board = Board::new();

    let board = board.apply_movement(Movement::new(3), &zobrist).unwrap();
    let board = board.apply_movement(Movement::new(3), &zobrist).unwrap();

    assert_eq!(board.grid()[5][3], Some(Player::Red));
    assert_eq!(board.grid()[4][3], Some(Player::Yellow));
}

#[test]
fn should_not_allow_full_column() {
    let zobrist = ZobristTable::new();
    let mut board = Board::new();

    for _ in 0..6 {
        board = board.apply_movement(Movement::new(0), &zobrist).unwrap();
    }
    assert!(board.apply_movement(Movement::new(0), &zobrist).is_none());
}

#[test]
fn should_detect_horizontal_win() {
    let zobrist = ZobristTable::new();
    let mut board = Board::new();

    board = board.apply_movement(Movement::new(0), &zobrist).unwrap();
    board = board.apply_movement(Movement::new(0), &zobrist).unwrap();

    board = board.apply_movement(Movement::new(1), &zobrist).unwrap();
    board = board.apply_movement(Movement::new(1), &zobrist).unwrap();

    board = board.apply_movement(Movement::new(2), &zobrist).unwrap();
    board = board.apply_movement(Movement::new(2), &zobrist).unwrap();

    board = board.apply_movement(Movement::new(3), &zobrist).unwrap();

    assert_eq!(board.game_state(), GameState::Win(Player::Red));
}

#[test]
fn should_detect_vertical_win() {
    let zobrist = ZobristTable::new();
    let mut board = Board::new();

    board = board.apply_movement(Movement::new(0), &zobrist).unwrap();
    board = board.apply_movement(Movement::new(1), &zobrist).unwrap();

    board = board.apply_movement(Movement::new(0), &zobrist).unwrap();
    board = board.apply_movement(Movement::new(1), &zobrist).unwrap();

    board = board.apply_movement(Movement::new(0), &zobrist).unwrap();
    board = board.apply_movement(Movement::new(1), &zobrist).unwrap();

    board = board.apply_movement(Movement::new(0), &zobrist).unwrap();

    assert_eq!(board.game_state(), GameState::Win(Player::Red));
}

#[test]
fn should_detect_diagonal_up_win() {
    let zobrist = ZobristTable::new();
    let mut board = Board::new();

    board = board.apply_movement(Movement::new(0), &zobrist).unwrap();
    board = board.apply_movement(Movement::new(6), &zobrist).unwrap();

    board = board.apply_movement(Movement::new(1), &zobrist).unwrap();
    board = board.apply_movement(Movement::new(6), &zobrist).unwrap();
    board = board.apply_movement(Movement::new(1), &zobrist).unwrap();
    board = board.apply_movement(Movement::new(6), &zobrist).unwrap();

    board = board.apply_movement(Movement::new(2), &zobrist).unwrap();
    board = board.apply_movement(Movement::new(5), &zobrist).unwrap();
    board = board.apply_movement(Movement::new(2), &zobrist).unwrap();
    board = board.apply_movement(Movement::new(5), &zobrist).unwrap();
    board = board.apply_movement(Movement::new(2), &zobrist).unwrap();
    board = board.apply_movement(Movement::new(5), &zobrist).unwrap();

    board = board.apply_movement(Movement::new(4), &zobrist).unwrap();
    board = board.apply_movement(Movement::new(3), &zobrist).unwrap();
    board = board.apply_movement(Movement::new(4), &zobrist).unwrap();
    board = board.apply_movement(Movement::new(3), &zobrist).unwrap();
    board = board.apply_movement(Movement::new(4), &zobrist).unwrap();
    board = board.apply_movement(Movement::new(3), &zobrist).unwrap();
    board = board.apply_movement(Movement::new(3), &zobrist).unwrap();

    assert_eq!(board.game_state(), GameState::Win(Player::Red));
}

#[test]
fn should_detect_diagonal_down_win() {
    let zobrist = ZobristTable::new();
    let mut board = Board::new();

    board = board.apply_movement(Movement::new(5), &zobrist).unwrap();
    board = board.apply_movement(Movement::new(5), &zobrist).unwrap();

    board = board.apply_movement(Movement::new(3), &zobrist).unwrap();
    board = board.apply_movement(Movement::new(4), &zobrist).unwrap();
    board = board.apply_movement(Movement::new(4), &zobrist).unwrap();
    board = board.apply_movement(Movement::new(3), &zobrist).unwrap();

    board = board.apply_movement(Movement::new(3), &zobrist).unwrap();
    board = board.apply_movement(Movement::new(2), &zobrist).unwrap();
    board = board.apply_movement(Movement::new(2), &zobrist).unwrap();
    board = board.apply_movement(Movement::new(2), &zobrist).unwrap();
    board = board.apply_movement(Movement::new(2), &zobrist).unwrap();

    assert_eq!(board.game_state(), GameState::Win(Player::Red));
}
