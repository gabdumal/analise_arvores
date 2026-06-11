use crate::game::{board::Board, movement::Movement};
#[derive(Clone)]
pub struct MonteCarloNode {
    pub parent: Option<usize>,
    pub children: Vec<usize>,

    pub board: Board,
    pub movement: Option<Movement>,

    pub visits: usize,
    pub wins: f64,

    pub untried_movements: Vec<Movement>,

    pub graph_id: Option<usize>,
}

impl MonteCarloNode {
    pub fn new_root(board: Board, graph_id: Option<usize>) -> Self {
        let untried_movements = board.legal_movements();

        Self {
            board,
            parent: None,
            children: Vec::new(),
            movement: None,
            visits: 0,
            wins: 0.0,
            untried_movements,
            graph_id,
        }
    }

    pub fn new_child(
        board: Board,
        parent: usize,
        movement: Movement,
        graph_id: Option<usize>,
    ) -> Self {
        let untried_movements = board.legal_movements();

        Self {
            board,
            parent: Some(parent),
            children: Vec::new(),
            movement: Some(movement),
            visits: 0,
            wins: 0.0,
            untried_movements,
            graph_id,
        }
    }
}
