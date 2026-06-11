use crate::{
    agents::{
        Agent, minimax_alpha_beta::MinimaxAlphaBeta,
        minimax_alpha_beta_with_transposition_table::MinimaxAlphaBetaWithTranspositionTable,
        monte_carlo::MonteCarloTreeSearch,
    },
    experiment::visualization::{
        runner::run_visualization,
        scenario::{VisualizationRequest, VisualizationScenario, VisualizationTarget},
    },
    game::match_context::MatchContext,
};

mod agents;
mod experiment;
mod game;

fn run_minimax_alpha_beta(depth_limit: usize) {
    let match_context = MatchContext::new();
    let mut agent = MinimaxAlphaBeta::new(depth_limit, 0, false);
    let movement = agent.choose_movement(&match_context);
    println!(
        "Depth limit: {}. Chosen column: {}.",
        depth_limit, movement.column
    );
    println!("{:#?}", agent.metrics());
}

fn experiment_with_minimax_alpha_beta() {
    println!("Minimax with Alpha-Beta pruning.");
    run_minimax_alpha_beta(6);
    run_minimax_alpha_beta(7);
    run_minimax_alpha_beta(8);
}

fn run_minimax_alpha_beta_with_transposition_table(depth_limit: usize) {
    let match_context = MatchContext::new();
    let mut tt = MinimaxAlphaBetaWithTranspositionTable::new(depth_limit, 0, false);
    let movement = tt.choose_movement(&match_context);
    println!(
        "Depth limit: {}. Chosen column: {}.",
        depth_limit, movement.column
    );
    println!("{:#?}", tt.metrics());
}

fn experiment_with_minimax_alpha_beta_with_transposition_table() {
    println!("Minimax with Alpha-Beta pruning and transposition table.");
    run_minimax_alpha_beta_with_transposition_table(6);
    run_minimax_alpha_beta_with_transposition_table(7);
    run_minimax_alpha_beta_with_transposition_table(8);
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

fn experiment_with_monte_carlo() {
    println!("Monte-Carlo.");
    run_monte_carlo(10_000);
    run_monte_carlo(20_000);
    run_monte_carlo(30_000);
}

fn visualize_minimax_alpha_beta() {
    let request = VisualizationRequest {
        target: VisualizationTarget::MinimaxAlphaBeta,
        scenario: VisualizationScenario {
            name: "opening".to_string(),
            match_context: MatchContext::new(),
        },
        limit: 5,
    };
    run_visualization(request);
}

fn visualize_minimax_alpha_beta_with_transposition_table() {
    let request = VisualizationRequest {
        target: VisualizationTarget::MinimaxAlphaBetaWithTranspositionTable,
        scenario: VisualizationScenario {
            name: "opening".to_string(),
            match_context: MatchContext::new(),
        },
        limit: 5,
    };
    run_visualization(request);
}

fn main() {
    visualize_minimax_alpha_beta_with_transposition_table();
}
