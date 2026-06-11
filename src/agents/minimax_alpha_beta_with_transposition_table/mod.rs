use crate::{
    agents::{
        Agent,
        minimax_alpha_beta::MinimaxAlphaBeta,
        minimax_alpha_beta_with_transposition_table::transposition_table::{
            NodeType, TranspositionTable, TranspositionTableEntry,
        },
        search_metrics::SearchMetrics,
    },
    experiment::graphviz::minimax_alpha_beta_with_transposition_table::MinimaxAlphaBetaWithTranspositionTableGraph,
    game::{
        board::Board, game_state::GameState, match_context::MatchContext, movement::Movement,
        player::Player,
    },
};
use std::time::Instant;

pub mod transposition_table;

pub struct MinimaxAlphaBetaWithTranspositionTable {
    depth_limit: usize,
    metrics: SearchMetrics,
    table: TranspositionTable,
    pub graph: MinimaxAlphaBetaWithTranspositionTableGraph,
}

impl MinimaxAlphaBetaWithTranspositionTable {
    pub fn new(depth_limit: usize, max_nodes: usize, generate_graph: bool) -> Self {
        Self {
            depth_limit,
            metrics: SearchMetrics::default(),
            table: TranspositionTable::new(),
            graph: MinimaxAlphaBetaWithTranspositionTableGraph::new(max_nodes, generate_graph),
        }
    }

    fn search(&mut self, match_context: &MatchContext) -> Movement {
        self.reset_metrics();
        self.table.clear();

        let start = Instant::now();

        let mut alpha = isize::MIN;
        let mut beta = isize::MAX;

        let board = match_context.board();
        let root = self.graph.create_node(board, 0, alpha, beta);
        self.metrics.nodes_expanded = 1;

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
                self.depth_limit - 1,
                alpha,
                beta,
                !maximizing,
                root,
                Some(movement),
                match_context,
            );

            if maximizing {
                if score > best_score {
                    best_score = score;
                    best_movement = movement;
                }

                alpha = alpha.max(best_score);
                if alpha >= beta {
                    break;
                }
            } else {
                if score < best_score {
                    best_score = score;
                    best_movement = movement;
                }

                beta = beta.min(best_score);
                if alpha >= beta {
                    break;
                }
            }
        }

        if let Some(root_id) = root {
            self.graph.nodes[root_id].value = Some(best_score);
            self.graph.nodes[root_id].alpha_out = alpha;
            self.graph.nodes[root_id].beta_out = beta;
        }

        self.metrics.peak_tt_entries = self.table.len();
        self.metrics.peak_structure_memory_bytes =
            self.table.len() * std::mem::size_of::<TranspositionTableEntry>();

        self.metrics.elapsed_time_ns = start.elapsed().as_nanos();

        best_movement
    }

    fn minimax(
        &mut self,
        board: &Board,
        remaining_depth: usize,
        mut alpha: isize,
        mut beta: isize,
        maximizing: bool,
        parent_id: Option<usize>,
        incoming_movement: Option<Movement>,
        match_context: &MatchContext,
    ) -> isize {
        let current_depth = self.depth_limit.saturating_sub(remaining_depth);

        let node_id = if self.graph.config.enabled {
            let node_id = self.graph.create_node(board, current_depth, alpha, beta);

            if let Some(child_id) = node_id
                && let Some(parent_id) = parent_id
                && let Some(movement) = incoming_movement
            {
                self.graph.connect(parent_id, child_id, movement);
            }

            node_id
        } else {
            None
        };

        //
        // VISITED NODE
        //
        self.metrics.nodes_expanded += 1;
        self.metrics.max_depth_reached = self.metrics.max_depth_reached.max(current_depth);

        //
        // FRONTIER
        //
        let legal_movements = board.legal_movements();
        self.metrics.peak_frontier_size =
            self.metrics.peak_frontier_size.max(legal_movements.len());

        //
        // STACK (aproximação)
        //
        self.metrics.peak_nodes_in_memory = self.metrics.peak_nodes_in_memory.max(current_depth);
        self.metrics.estimated_stack_memory_bytes = self
            .metrics
            .estimated_stack_memory_bytes
            .max(current_depth * std::mem::size_of::<Board>());

        //
        // TT LOOKUP
        //
        self.metrics.tt_lookups += 1;
        let hash = board.zobrist_hash();

        if let Some(entry) = self.table.get(hash) {
            self.metrics.tt_hits += 1;

            if let Some(node_id) = node_id {
                let node = &mut self.graph.nodes[node_id];

                node.tt_hit = true;
                node.tt_depth = Some(entry.depth);
                node.tt_value = Some(entry.value);
                node.tt_bound = Some(entry.node_type);
            }

            if entry.depth >= remaining_depth {
                match entry.node_type {
                    NodeType::Exact => {
                        if let Some(node_id) = node_id {
                            let node = &mut self.graph.nodes[node_id];

                            node.resolved_by_tt = true;
                            node.value = Some(entry.value);

                            node.alpha_out = alpha;
                            node.beta_out = beta;
                        }

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

                    if let Some(node_id) = node_id {
                        let node = &mut self.graph.nodes[node_id];

                        node.cutoff_occurred = true;
                        node.resolved_by_tt = true;
                        node.value = Some(entry.value);

                        node.alpha_out = alpha;
                        node.beta_out = beta;
                    }

                    return entry.value;
                }
            }
        } else {
            self.metrics.tt_misses += 1;
        }

        let original_alpha = alpha;
        let original_beta = beta;

        //
        // DEPTH CUTOFF
        //
        if remaining_depth == 0 {
            self.metrics.leaf_nodes += 1;

            let value = board.evaluate();
            self.metrics.nodes_evaluated += 1;

            self.table.insert(
                hash,
                TranspositionTableEntry {
                    value,
                    depth: remaining_depth,
                    node_type: NodeType::Exact,
                },
            );

            self.metrics.tt_stores += 1;

            if let Some(node_id) = node_id {
                let node = &mut self.graph.nodes[node_id];

                node.value = Some(value);
                node.alpha_out = alpha;
                node.beta_out = beta;
            }

            return value;
        }

        //
        // TERMINAL NODE
        //
        match board.game_state() {
            GameState::InProgress => {}

            _ => {
                self.metrics.leaf_nodes += 1;

                let value = board.evaluate();
                self.metrics.nodes_evaluated += 1;

                self.table.insert(
                    hash,
                    TranspositionTableEntry {
                        value,
                        depth: remaining_depth,
                        node_type: NodeType::Exact,
                    },
                );

                self.metrics.tt_stores += 1;

                if let Some(node_id) = node_id {
                    let node = &mut self.graph.nodes[node_id];

                    node.value = Some(value);
                    node.alpha_out = alpha;
                    node.beta_out = beta;
                }

                return value;
            }
        }

        //
        // SEARCH
        //
        let value = if maximizing {
            let mut value = isize::MIN;

            for movement in legal_movements {
                let child = board
                    .apply_movement(movement, Some(match_context.zobrist()))
                    .unwrap();

                value = value.max(self.minimax(
                    &child,
                    remaining_depth - 1,
                    alpha,
                    beta,
                    false,
                    node_id,
                    Some(movement),
                    match_context,
                ));

                alpha = alpha.max(value);

                if alpha >= beta {
                    self.metrics.alpha_cutoffs += 1;

                    if let Some(node_id) = node_id {
                        self.graph.nodes[node_id].cutoff_occurred = true;
                    }

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
                    remaining_depth - 1,
                    alpha,
                    beta,
                    true,
                    node_id,
                    Some(movement),
                    match_context,
                ));

                beta = beta.min(value);

                if alpha >= beta {
                    self.metrics.beta_cutoffs += 1;

                    if let Some(node_id) = node_id {
                        self.graph.nodes[node_id].cutoff_occurred = true;
                    }

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
                depth: remaining_depth,
                node_type,
            },
        );

        self.metrics.tt_stores += 1;

        self.metrics.peak_tt_entries = self.metrics.peak_tt_entries.max(self.table.len());

        self.metrics.peak_structure_memory_bytes =
            self.table.len() * std::mem::size_of::<TranspositionTableEntry>();

        if let Some(node_id) = node_id {
            let node = &mut self.graph.nodes[node_id];

            node.value = Some(value);
            node.alpha_out = alpha;
            node.beta_out = beta;
        }

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

    let mut agent = MinimaxAlphaBetaWithTranspositionTable::new(6, 0, false);

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

    let mut agent = MinimaxAlphaBetaWithTranspositionTable::new(6, 0, false);

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

    let mut agent = MinimaxAlphaBetaWithTranspositionTable::new(6, 0, false);

    let movement = agent.choose_movement(&match_context);

    assert_eq!(movement.column, 3);
}

#[test]
fn tt_should_store_entries() {
    let match_context = MatchContext::new();

    let mut agent = MinimaxAlphaBetaWithTranspositionTable::new(6, 0, false);

    agent.choose_movement(&match_context);

    assert!(agent.metrics().tt_stores > 0);
}

#[test]
fn tt_should_generate_hits() {
    let match_context = MatchContext::new();

    let mut agent = MinimaxAlphaBetaWithTranspositionTable::new(7, 0, false);

    agent.choose_movement(&match_context);

    assert!(agent.metrics().tt_hits > 0);
}

#[test]
fn tt_and_alpha_beta_should_choose_same_move() {
    let match_context = MatchContext::new();

    let mut ab = MinimaxAlphaBeta::new(7, 0, false);

    let mut tt = MinimaxAlphaBetaWithTranspositionTable::new(7, 0, false);

    let ab_move = ab.choose_movement(&match_context);

    let tt_move = tt.choose_movement(&match_context);

    assert_eq!(ab_move.column, tt_move.column);
}

#[test]
fn tt_should_expand_fewer_nodes() {
    let match_context = MatchContext::new();

    let mut ab = MinimaxAlphaBeta::new(8, 0, false);

    let mut tt = MinimaxAlphaBetaWithTranspositionTable::new(8, 0, false);

    ab.choose_movement(&match_context);
    tt.choose_movement(&match_context);

    assert!(tt.metrics().nodes_expanded < ab.metrics().nodes_expanded);
}

#[test]
fn tt_should_use_memory() {
    let match_context = MatchContext::new();

    let mut tt = MinimaxAlphaBetaWithTranspositionTable::new(7, 0, false);

    tt.choose_movement(&match_context);

    assert!(tt.metrics().estimated_stack_memory_bytes > 0);
}

#[test]
fn tt_should_choose_center_column() {
    let match_context = MatchContext::new();

    let mut tt = MinimaxAlphaBetaWithTranspositionTable::new(8, 0, false);

    let movement = tt.choose_movement(&match_context);

    assert_eq!(movement.column, 3);
}
