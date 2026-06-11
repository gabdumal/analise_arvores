use crate::{
    agents::{
        Agent, minimax_alpha_beta::MinimaxAlphaBeta,
        minimax_alpha_beta_with_transposition_table::MinimaxAlphaBetaWithTranspositionTable,
        monte_carlo::MonteCarlo,
    },
    experiment::{
        graphviz::exporter::GraphvizExporter,
        visualization::scenario::{VisualizationRequest, VisualizationTarget},
    },
};

pub fn run_visualization(request: VisualizationRequest) {
    match request.target {
        VisualizationTarget::MinimaxAlphaBeta => {
            run_minimax_alpha_beta(request.scenario.match_context, request.limit);
        }

        VisualizationTarget::MinimaxAlphaBetaWithTranspositionTable => {
            run_minimax_alpha_beta_with_transposition_table(
                request.scenario.match_context,
                request.limit,
            );
        }

        VisualizationTarget::MonteCarlo => {
            run_monte_carlo(request.scenario.match_context, request.limit);
        }
    }
}

fn run_minimax_alpha_beta(
    match_context: crate::game::match_context::MatchContext,
    depth_limit: usize,
) {
    let mut agent = MinimaxAlphaBeta::new(depth_limit, usize::MAX, true);

    let movement = agent.choose_movement(&match_context);

    println!("Scenario: Minimax Alpha-Beta");
    println!("Movement: {:?}", movement);
    println!("{:#?}", agent.metrics());

    agent
        .graph
        .export(format!("results/graphs/alpha_beta_depth_{}", depth_limit))
        .unwrap();
}

fn run_minimax_alpha_beta_with_transposition_table(
    match_context: crate::game::match_context::MatchContext,
    depth_limit: usize,
) {
    let mut agent = MinimaxAlphaBetaWithTranspositionTable::new(depth_limit, usize::MAX, true);

    let movement = agent.choose_movement(&match_context);

    println!("Scenario: Minimax Alpha-Beta with Transposition Table");
    println!("Movement: {:?}", movement);
    println!("{:#?}", agent.metrics());

    agent
        .graph
        .export(format!(
            "results/graphs/alpha_beta_tt_depth_{}",
            depth_limit
        ))
        .unwrap();
}

fn run_monte_carlo(match_context: crate::game::match_context::MatchContext, simulations: usize) {
    let mut agent = MonteCarlo::new(simulations, usize::MAX, true);

    let movement = agent.choose_movement(&match_context);

    println!("Scenario: Monte-Carlo");
    println!("Movement: {:?}", movement);
    println!("{:#?}", agent.metrics());

    agent
        .graph
        .export(format!(
            "results/graphs/monte_carlo_simulations_{}",
            simulations
        ))
        .unwrap();
}
