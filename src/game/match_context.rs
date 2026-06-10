use crate::game::{board::Board, game_state::GameState, movement::Movement, zobrist::ZobristTable};

#[derive(Debug)]
pub enum MatchError {
    InvalidMovement(Movement),
}

pub struct MatchContext {
    board: Board,
    zobrist: ZobristTable,
    movement_history: Vec<Movement>,
}

impl MatchContext {
    pub fn new() -> Self {
        Self {
            board: Board::new(),
            zobrist: ZobristTable::new(),
            movement_history: Vec::new(),
        }
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    pub fn zobrist(&self) -> &ZobristTable {
        &self.zobrist
    }

    pub fn movement_history(&self) -> &[Movement] {
        &self.movement_history
    }

    pub fn movement_count(&self) -> usize {
        self.movement_history.len()
    }

    pub fn last_movement(&self) -> Option<Movement> {
        self.movement_history.last().copied()
    }

    pub fn play(&mut self, movement: Movement) -> Result<(), MatchError> {
        let next_board = self
            .board
            .apply_movement(movement, Some(&self.zobrist))
            .ok_or(MatchError::InvalidMovement(movement))?;

        self.board = next_board;
        self.movement_history.push(movement);

        Ok(())
    }

    pub fn game_state(&self) -> GameState {
        self.board.game_state()
    }

    pub fn is_finished(&self) -> bool {
        !matches!(self.game_state(), GameState::InProgress)
    }
}
