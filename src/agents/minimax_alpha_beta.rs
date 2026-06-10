use std::time::Instant;

use crate::{
    agents::{Agent, search_metrics::SearchMetrics},
    game::{
        board::Board, game_state::GameState, match_context::MatchContext, movement::Movement,
        player::Player,
    },
};

pub struct MinimaxAlphaBeta {
    depth_limit: usize,
    metrics: SearchMetrics,
}

impl MinimaxAlphaBeta {
    pub fn new(depth_limit: usize) -> Self {
        Self {
            depth_limit,
            metrics: SearchMetrics::default(),
        }
    }

    fn search(&mut self, match_context: &MatchContext) -> Movement {
        self.reset_metrics();

        let start = Instant::now();

        let board = match_context.board();

        let maximizing = board.current_player() == Player::Red;

        let legal_movements = board.legal_movements();

        let mut best_movement = legal_movements[0];

        let mut best_score = if maximizing { isize::MIN } else { isize::MAX };

        for movement in legal_movements {
            let child = board.apply_movement(movement, None).unwrap();

            let score = self.minimax(
                &child,
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
        // Derived metrics
        //
        if self.metrics.max_depth_reached > 0 {
            self.metrics.effective_branching_factor = (self.metrics.nodes_expanded as f64)
                .powf(1.0 / self.metrics.max_depth_reached as f64);
        }

        if self.metrics.nodes_expanded > 0 {
            self.metrics.nanoseconds_per_node =
                self.metrics.elapsed_time_ns as f64 / self.metrics.nodes_expanded as f64;
        }

        best_movement
    }

    fn minimax(
        &mut self,
        board: &Board,
        depth: usize,
        mut alpha: isize,
        mut beta: isize,
        maximizing: bool,
        current_depth: usize,
    ) -> isize {
        self.metrics.nodes_expanded += 1;

        self.metrics.max_depth_reached = self.metrics.max_depth_reached.max(current_depth);

        //
        // Alpha-Beta puro:
        // memória ≈ profundidade da pilha
        //
        self.metrics.peak_nodes_in_memory = self.metrics.peak_nodes_in_memory.max(current_depth);

        let legal_movements = board.legal_movements();

        self.metrics.peak_frontier_size =
            self.metrics.peak_frontier_size.max(legal_movements.len());

        self.metrics.estimated_stack_memory_bytes = self
            .metrics
            .estimated_stack_memory_bytes
            .max(current_depth * std::mem::size_of::<Board>());

        //
        // Não existe estrutura auxiliar
        //
        self.metrics.peak_structure_memory_bytes = 0;

        //
        // Depth cutoff
        //
        if depth == 0 {
            self.metrics.leaf_nodes += 1;

            //
            // Heurística realmente executada
            //
            self.metrics.nodes_evaluated += 1;

            return board.evaluate();
        }

        //
        // Terminal node
        //
        match board.game_state() {
            GameState::InProgress => {}
            _ => {
                self.metrics.leaf_nodes += 1;
                self.metrics.nodes_evaluated += 1;
                return board.evaluate();
            }
        }

        if maximizing {
            let mut value = isize::MIN;

            for movement in legal_movements {
                let child = board.apply_movement(movement, None).unwrap();

                value = value.max(self.minimax(
                    &child,
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
                let child = board.apply_movement(movement, None).unwrap();

                value = value.min(self.minimax(
                    &child,
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
        }
    }
}

impl Agent for MinimaxAlphaBeta {
    fn name(&self) -> &'static str {
        "Minimax Alpha-Beta"
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
fn minimax_should_choose_legal_movement() {
    let match_context = MatchContext::new();
    let mut agent = MinimaxAlphaBeta::new(4);

    let movement = agent.choose_movement(&match_context);
    assert!(match_context.board().legal_movements().contains(&movement));
}

#[test]
fn minimax_should_take_immediate_win() {
    let mut match_context: MatchContext = MatchContext::new();

    match_context.play(Movement::new(0)).unwrap();
    match_context.play(Movement::new(0)).unwrap();

    match_context.play(Movement::new(1)).unwrap();
    match_context.play(Movement::new(1)).unwrap();

    match_context.play(Movement::new(2)).unwrap();
    match_context.play(Movement::new(2)).unwrap();

    let mut agent = MinimaxAlphaBeta::new(4);
    let movement = agent.choose_movement(&match_context);

    assert_eq!(movement.column, 3);
}

#[test]
fn minimax_should_block_immediate_loss() {
    let mut match_context = MatchContext::new();

    match_context.play(Movement::new(4)).unwrap();
    match_context.play(Movement::new(0)).unwrap();
    match_context.play(Movement::new(4)).unwrap();
    match_context.play(Movement::new(1)).unwrap();
    match_context.play(Movement::new(5)).unwrap();
    match_context.play(Movement::new(2)).unwrap();

    let mut agent = MinimaxAlphaBeta::new(4);
    let movement = agent.choose_movement(&match_context);

    assert_eq!(movement.column, 3);
}

#[test]
fn minimax_should_collect_metrics() {
    let match_context = MatchContext::new();

    let mut agent = MinimaxAlphaBeta::new(4);
    agent.choose_movement(&match_context);

    let metrics = agent.metrics();
    assert!(metrics.nodes_expanded > 0);
    assert!(metrics.leaf_nodes > 0);
    assert!(metrics.max_depth_reached > 0);
}

#[test]
fn minimax_should_perform_pruning() {
    let match_context = MatchContext::new();

    let mut agent = MinimaxAlphaBeta::new(6);
    agent.choose_movement(&match_context);

    let metrics = agent.metrics();
    assert!(metrics.alpha_cutoffs + metrics.beta_cutoffs > 0);
}
