use crate::{
    agents::{
        Agent, minimax_alpha_beta::MinimaxAlphaBeta, monte_carlo::node::MonteCarloNode,
        search_metrics::SearchMetrics,
    },
    game::{match_context::MatchContext, movement::Movement},
};
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use std::time::Instant;

mod node;
mod rollout;
mod selection;
mod uct;

pub struct MonteCarloTreeSearch {
    simulations: usize,
    metrics: SearchMetrics,
    arena: Vec<MonteCarloNode>,
    rng: ChaCha8Rng,
}

impl MonteCarloTreeSearch {
    pub fn new(simulations: usize) -> Self {
        Self {
            simulations,
            metrics: SearchMetrics::default(),
            arena: Vec::new(),
            rng: ChaCha8Rng::seed_from_u64(512),
        }
    }

    fn search(&mut self, match_context: &MatchContext) -> Movement {
        self.reset_metrics();

        let start = Instant::now();

        self.arena.clear();

        self.arena
            .push(MonteCarloNode::new_root(match_context.board().clone()));

        let root_player = match_context.board().current_player();

        for _ in 0..self.simulations {
            self.metrics.simulations += 1;

            //
            // 1. Selection
            //
            let mut node_index = 0;
            let mut depth = 0;

            while self.arena[node_index].untried_movements.is_empty()
                && !self.arena[node_index].children.is_empty()
            {
                node_index = selection::best_child(&self.arena, node_index);
                depth += 1;
            }

            self.metrics.max_depth_reached = self.metrics.max_depth_reached.max(depth);

            self.metrics.estimated_stack_memory_bytes = self
                .metrics
                .estimated_stack_memory_bytes
                .max(depth * std::mem::size_of::<usize>());

            //
            // 2. Expansion
            //
            if !self.arena[node_index].untried_movements.is_empty() {
                let movement = self.arena[node_index].untried_movements.pop().unwrap();

                let child_board = self.arena[node_index]
                    .board
                    .apply_movement(movement, None)
                    .unwrap();

                let child_index = self.arena.len();

                self.arena
                    .push(MonteCarloNode::new_child(child_board, node_index, movement));

                self.arena[node_index].children.push(child_index);

                node_index = child_index;

                //
                // Um nó novo foi criado
                //
                self.metrics.nodes_expanded += 1;
            }

            //
            // Frontier
            //
            let branching = self.arena[node_index].board.legal_movements().len();
            self.metrics.peak_frontier_size = self.metrics.peak_frontier_size.max(branching);

            //
            // 3. Simulation
            //
            let mut rollout_length = 0;
            let reward = rollout::rollout(
                self.arena[node_index].board.clone(),
                root_player,
                &mut rollout_length,
                &mut self.rng,
            );

            self.metrics.nodes_evaluated += 1;
            self.metrics.leaf_nodes += 1;
            self.metrics.total_rollout_length += rollout_length;

            //
            // 4. Backpropagation
            //
            let mut current = Some(node_index);

            while let Some(index) = current {
                self.arena[index].visits += 1;

                self.arena[index].wins += reward;

                current = self.arena[index].parent;
            }

            //
            // Memory metrics
            //
            self.metrics.peak_nodes_in_memory =
                self.metrics.peak_nodes_in_memory.max(self.arena.len());

            self.metrics.peak_structure_memory_bytes = self
                .metrics
                .peak_structure_memory_bytes
                .max(self.arena.len() * std::mem::size_of::<MonteCarloNode>());
        }

        self.metrics.elapsed_time_ns = start.elapsed().as_nanos();

        //
        // Most visited child
        //
        let root = &self.arena[0];

        let best_child = root
            .children
            .iter()
            .max_by_key(|&&child| self.arena[child].visits)
            .copied()
            .expect("Root has no children");

        self.arena[best_child].movement.unwrap()
    }
}

impl Agent for MonteCarloTreeSearch {
    fn name(&self) -> &'static str {
        "Monte Carlo Tree Search"
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
fn mcts_returns_legal_movement() {
    let match_context = MatchContext::new();

    let mut agent = MonteCarloTreeSearch::new(1_000);

    let movement = agent.search(&match_context);

    assert!(match_context.board().legal_movements().contains(&movement));
}

#[test]
fn mcts_expands_tree() {
    let match_context = MatchContext::new();

    let mut agent = MonteCarloTreeSearch::new(5_000);

    agent.search(&match_context);

    assert!(agent.metrics().nodes_expanded > 0);
    assert!(agent.metrics().peak_nodes_in_memory > 1);
}

#[test]
fn mcts_counts_simulations() {
    let match_context = MatchContext::new();

    let mut agent = MonteCarloTreeSearch::new(10_000);

    agent.search(&match_context);

    assert_eq!(agent.metrics().simulations, 10_000);
}

#[test]
fn mcts_updates_memory_metrics() {
    let match_context = MatchContext::new();

    let mut agent = MonteCarloTreeSearch::new(2_000);

    agent.search(&match_context);

    assert!(agent.metrics().peak_nodes_in_memory > 0);
    assert!(agent.metrics().peak_structure_memory_bytes > 0);
}

#[test]
fn mcts_finds_immediate_win() {
    let mut match_context = MatchContext::new();

    match_context.play(Movement::new(0)).unwrap();
    match_context.play(Movement::new(4)).unwrap();
    match_context.play(Movement::new(1)).unwrap();
    match_context.play(Movement::new(5)).unwrap();
    match_context.play(Movement::new(2)).unwrap();

    let mut agent = MonteCarloTreeSearch::new(10_000);

    let movement = agent.search(&match_context);

    assert_eq!(movement.column, 3);
}

#[test]
fn mcts_updates_depth_metrics() {
    let match_context = MatchContext::new();

    let mut agent = MonteCarloTreeSearch::new(10_000);

    agent.search(&match_context);

    assert!(agent.metrics().max_depth_reached > 0);
    assert!(agent.metrics().peak_frontier_size > 0);
}
