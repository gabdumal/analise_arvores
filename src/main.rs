use crate::{
    agents::{
        Agent, minimax_alpha_beta::MinimaxAlphaBeta,
        minimax_alpha_beta_with_transposition_table::MinimaxAlphaBetaWithTranspositionTable,
        monte_carlo::MonteCarloTreeSearch,
    },
    game::match_context::MatchContext,
};

mod agents;
mod game;

fn run_alpha_beta(depth_limit: usize) {
    let match_context = MatchContext::new();
    let mut agent = MinimaxAlphaBeta::new(depth_limit);
    let movement = agent.choose_movement(&match_context);
    println!(
        "Depth limit: {}. Chosen column: {}.",
        depth_limit, movement.column
    );
    println!("{:#?}", agent.metrics());
}

fn alpha_beta() {
    println!("Minimax with Alpha-Beta pruning.");
    run_alpha_beta(6);
    run_alpha_beta(7);
    run_alpha_beta(8);
}

fn run_alpha_beta_with_transposition_table(depth_limit: usize) {
    let match_context = MatchContext::new();
    let mut tt = MinimaxAlphaBetaWithTranspositionTable::new(depth_limit);
    let movement = tt.choose_movement(&match_context);
    println!(
        "Depth limit: {}. Chosen column: {}.",
        depth_limit, movement.column
    );
    println!("{:#?}", tt.metrics());
}

fn alpha_beta_with_transposition_table() {
    println!("Minimax with Alpha-Beta pruning and transposition table.");
    run_alpha_beta_with_transposition_table(6);
    run_alpha_beta_with_transposition_table(7);
    run_alpha_beta_with_transposition_table(8);
}

fn run_monte_carlo(simulations: usize) {
    let match_context = MatchContext::new();
    let mut agent = MonteCarloTreeSearch::new(simulations);
    let movement = agent.choose_movement(&match_context);
    println!(
        "Simulations: {}. Chosen column: {}",
        simulations, movement.column
    );
    println!("{:#?}", agent.metrics());
}

fn monte_carlo() {
    println!("Monte-Carlo.");
    run_monte_carlo(50_000);
    run_monte_carlo(200_000);
    run_monte_carlo(400_000);
}

fn main() {
    alpha_beta();
    println!();
    alpha_beta_with_transposition_table();
    println!();
    monte_carlo();
    println!();
}
