use crate::{
    agents::{
        Agent,
        minimax_alpha_beta::MinimaxAlphaBeta,
        search_metrics::SearchMetrics,
        transposition_table::{NodeType, TranspositionTable, TranspositionTableEntry},
    },
    game::{
        board::Board, game_state::GameState, match_context::MatchContext, movement::Movement,
        player::Player,
    },
};
use std::time::Instant;

pub struct MinimaxAlphaBetaWithTranspositionTable {
    depth_limit: usize,
    metrics: SearchMetrics,
    table: TranspositionTable,
}

impl MinimaxAlphaBetaWithTranspositionTable {
    pub fn new(depth_limit: usize) -> Self {
        Self {
            depth_limit,
            metrics: SearchMetrics::default(),
            table: TranspositionTable::new(),
        }
    }

    fn search(&mut self, match_context: &MatchContext) -> Movement {
        self.reset_metrics();
        self.table.clear();

        let start = Instant::now();

        let board = match_context.board();

        let maximizing = board.current_player() == Player::Red;

        let legal_movements = board.legal_movements();

        let mut best_movement = legal_movements[0];

        let mut best_score = if maximizing { isize::MIN } else { isize::MAX };

        for movement in legal_movements {
            let child = board
                .apply_movement(movement, Some(match_context.zobrist()))
                .unwrap();

            let score = self.minimax(
                &child,
                match_context,
                self.depth_limit - 1,
                isize::MIN,
                isize::MAX,
                !maximizing,
                1,
            );

            if maximizing {
                if score > best_score {
                    best_score = score;
                    best_movement = movement;
                }
            } else if score < best_score {
                best_score = score;
                best_movement = movement;
            }
        }

        self.metrics.elapsed_time_ns = start.elapsed().as_nanos();

        //
        // Métricas derivadas (MESMO PADRÃO do AB)
        //
        if self.metrics.max_depth_reached > 0 {
            self.metrics.effective_branching_factor = (self.metrics.nodes_expanded as f64)
                .powf(1.0 / self.metrics.max_depth_reached as f64);
        }

        if self.metrics.nodes_expanded > 0 {
            self.metrics.nanoseconds_per_node =
                self.metrics.elapsed_time_ns as f64 / self.metrics.nodes_expanded as f64;
        }

        self.metrics.peak_tt_entries = self.table.len();

        self.metrics.peak_structure_memory_bytes =
            self.table.len() * std::mem::size_of::<TranspositionTableEntry>();

        best_movement
    }

    fn minimax(
        &mut self,
        board: &Board,
        match_context: &MatchContext,
        depth: usize,
        mut alpha: isize,
        mut beta: isize,
        maximizing: bool,
        current_depth: usize,
    ) -> isize {
        self.metrics.nodes_expanded += 1;

        self.metrics.max_depth_reached = self.metrics.max_depth_reached.max(current_depth);

        //
        // FRONTIER
        //
        self.metrics.peak_frontier_size = self
            .metrics
            .peak_frontier_size
            .max(board.legal_movements().len());

        //
        // STACK (aproximação)
        //
        self.metrics.peak_nodes_in_memory = self.metrics.peak_nodes_in_memory.max(current_depth);

        self.metrics.estimated_stack_memory_bytes = self
            .metrics
            .estimated_stack_memory_bytes
            .max(current_depth * std::mem::size_of::<Board>());

        //
        // TT lookup count (IMPORTANTE)
        //
        self.metrics.tt_lookups += 1;

        let hash = board.zobrist_hash();

        //
        // TRANSPOSITION TABLE LOOKUP
        //
        if let Some(entry) = self.table.get(hash) {
            self.metrics.tt_hits += 1;

            if entry.depth >= depth {
                match entry.node_type {
                    NodeType::Exact => {
                        return entry.value;
                    }
                    NodeType::LowerBound => {
                        alpha = alpha.max(entry.value);
                    }
                    NodeType::UpperBound => {
                        beta = beta.min(entry.value);
                    }
                }

                if alpha >= beta {
                    self.metrics.tt_cutoffs += 1;
                    return entry.value;
                }
            }
        } else {
            self.metrics.tt_misses += 1;
        }

        let original_alpha = alpha;
        let original_beta = beta;

        //
        // TERMINAL CONDITIONS
        //
        if depth == 0 {
            self.metrics.leaf_nodes += 1;
            self.metrics.nodes_evaluated += 1;

            let value = board.evaluate();

            self.table.insert(
                hash,
                TranspositionTableEntry {
                    value,
                    depth,
                    node_type: NodeType::Exact,
                },
            );

            self.metrics.tt_stores += 1;

            return value;
        }

        match board.game_state() {
            GameState::InProgress => {}

            _ => {
                self.metrics.leaf_nodes += 1;
                self.metrics.nodes_evaluated += 1;

                let value = board.evaluate();

                self.table.insert(
                    hash,
                    TranspositionTableEntry {
                        value,
                        depth,
                        node_type: NodeType::Exact,
                    },
                );

                self.metrics.tt_stores += 1;

                return value;
            }
        }

        //
        // SEARCH
        //
        let legal_movements = board.legal_movements();

        let value = if maximizing {
            let mut value = isize::MIN;

            for movement in legal_movements {
                let child = board
                    .apply_movement(movement, Some(match_context.zobrist()))
                    .unwrap();

                value = value.max(self.minimax(
                    &child,
                    match_context,
                    depth - 1,
                    alpha,
                    beta,
                    false,
                    current_depth + 1,
                ));

                alpha = alpha.max(value);

                if alpha >= beta {
                    self.metrics.alpha_cutoffs += 1;
                    break;
                }
            }

            value
        } else {
            let mut value = isize::MAX;

            for movement in legal_movements {
                let child = board
                    .apply_movement(movement, Some(match_context.zobrist()))
                    .unwrap();

                value = value.min(self.minimax(
                    &child,
                    match_context,
                    depth - 1,
                    alpha,
                    beta,
                    true,
                    current_depth + 1,
                ));

                beta = beta.min(value);

                if alpha >= beta {
                    self.metrics.beta_cutoffs += 1;
                    break;
                }
            }

            value
        };

        //
        // STORE IN TT
        //
        let node_type = if value <= original_alpha {
            NodeType::UpperBound
        } else if value >= original_beta {
            NodeType::LowerBound
        } else {
            NodeType::Exact
        };

        self.table.insert(
            hash,
            TranspositionTableEntry {
                value,
                depth,
                node_type,
            },
        );

        self.metrics.tt_stores += 1;

        self.metrics.peak_tt_entries = self.table.len();

        self.metrics.peak_structure_memory_bytes =
            self.table.len() * std::mem::size_of::<TranspositionTableEntry>();

        value
    }
}

impl Agent for MinimaxAlphaBetaWithTranspositionTable {
    fn name(&self) -> &'static str {
        "Minimax Alpha-Beta + Transposition Table"
    }

    fn metrics(&self) -> &SearchMetrics {
        &self.metrics
    }

    fn reset_metrics(&mut self) {
        self.metrics = SearchMetrics::default();
    }

    fn choose_movement(&mut self, match_context: &MatchContext) -> Movement {
        self.search(match_context)
    }
}

#[test]
fn tt_should_choose_legal_movement() {
    let match_context = MatchContext::new();

    let mut agent = MinimaxAlphaBetaWithTranspositionTable::new(6);

    let movement = agent.choose_movement(&match_context);

    assert!(match_context.board().legal_movements().contains(&movement));
}

#[test]
fn tt_should_take_immediate_win() {
    let mut match_context = MatchContext::new();

    match_context.play(Movement::new(0)).unwrap();
    match_context.play(Movement::new(0)).unwrap();

    match_context.play(Movement::new(1)).unwrap();
    match_context.play(Movement::new(1)).unwrap();

    match_context.play(Movement::new(2)).unwrap();
    match_context.play(Movement::new(2)).unwrap();

    let mut agent = MinimaxAlphaBetaWithTranspositionTable::new(6);

    let movement = agent.choose_movement(&match_context);

    assert_eq!(movement.column, 3);
}

#[test]
fn tt_should_block_immediate_loss() {
    let mut match_context = MatchContext::new();

    match_context.play(Movement::new(4)).unwrap();
    match_context.play(Movement::new(0)).unwrap();
    match_context.play(Movement::new(4)).unwrap();
    match_context.play(Movement::new(1)).unwrap();
    match_context.play(Movement::new(5)).unwrap();
    match_context.play(Movement::new(2)).unwrap();

    let mut agent = MinimaxAlphaBetaWithTranspositionTable::new(6);

    let movement = agent.choose_movement(&match_context);

    assert_eq!(movement.column, 3);
}

#[test]
fn tt_should_store_entries() {
    let match_context = MatchContext::new();

    let mut agent = MinimaxAlphaBetaWithTranspositionTable::new(6);

    agent.choose_movement(&match_context);

    assert!(agent.metrics().tt_stores > 0);
}

#[test]
fn tt_should_generate_hits() {
    let match_context = MatchContext::new();

    let mut agent = MinimaxAlphaBetaWithTranspositionTable::new(7);

    agent.choose_movement(&match_context);

    assert!(agent.metrics().tt_hits > 0);
}

#[test]
fn tt_and_alpha_beta_should_choose_same_move() {
    let match_context = MatchContext::new();

    let mut ab = MinimaxAlphaBeta::new(7);

    let mut tt = MinimaxAlphaBetaWithTranspositionTable::new(7);

    let ab_move = ab.choose_movement(&match_context);

    let tt_move = tt.choose_movement(&match_context);

    assert_eq!(ab_move.column, tt_move.column);
}

#[test]
fn tt_should_expand_fewer_nodes() {
    let match_context = MatchContext::new();

    let mut ab = MinimaxAlphaBeta::new(8);

    let mut tt = MinimaxAlphaBetaWithTranspositionTable::new(8);

    ab.choose_movement(&match_context);
    tt.choose_movement(&match_context);

    assert!(tt.metrics().nodes_expanded < ab.metrics().nodes_expanded);
}

#[test]
fn tt_should_use_memory() {
    let match_context = MatchContext::new();

    let mut tt = MinimaxAlphaBetaWithTranspositionTable::new(7);

    tt.choose_movement(&match_context);

    assert!(tt.metrics().estimated_stack_memory_bytes > 0);
}

#[test]
fn tt_should_choose_center_column() {
    let match_context = MatchContext::new();

    let mut tt = MinimaxAlphaBetaWithTranspositionTable::new(8);

    let movement = tt.choose_movement(&match_context);

    assert_eq!(movement.column, 3);
}
