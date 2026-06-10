// export/visualization/runner.rs

use std::fmt::format;

use crate::{
    agents::{Agent, minimax_alpha_beta::MinimaxAlphaBeta},
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
            // run_minimax_alpha_beta_tt(request.scenario.match_context, request.limit);
        }

        VisualizationTarget::MonteCarlo => {
            // run_monte_carlo(request.scenario.match_context, request.limit);
        }
    }
}

fn run_minimax_alpha_beta(
    match_context: crate::game::match_context::MatchContext,
    depth_limit: usize,
) {
    let mut agent = MinimaxAlphaBeta::new(depth_limit, 128, true);

    let movement = agent.choose_movement(&match_context);

    println!("Scenario: Alpha-Beta");
    println!("Move: {:?}", movement);
    println!("{:#?}", agent.metrics());

    agent
        .graph
        .export(format!(
            "results/graphs/alpha_beta_depth_{}.dot",
            depth_limit
        ))
        .unwrap();
}

// fn run_minimax_alpha_beta_tt(
//     match_context: crate::game::match_context::MatchContext,
//     depth_limit: usize,
// ) {
//     let mut agent = MinimaxAlphaBetaWithTranspositionTable::new(depth_limit);

//     agent.enable_graph_capture();

//     let movement = agent.search(&match_context);

//     println!("Scenario: Alpha-Beta + TT");
//     println!("Move: {:?}", movement);
//     println!("{:#?}", agent.metrics());

//     let dot = agent.graph().to_dot();

//     std::fs::create_dir_all("results/graphs").unwrap();

//     std::fs::write(format!("results/graphs/tt_depth_{}.dot", depth_limit), dot).unwrap();
// }

// fn run_monte_carlo(match_context: crate::game::match_context::MatchContext, simulations: usize) {
//     let mut agent = MonteCarloTreeSearch::new(simulations);

//     agent.enable_graph_capture();

//     let movement = agent.search(&match_context);

//     println!("Scenario: MCTS");
//     println!("Move: {:?}", movement);
//     println!("{:#?}", agent.metrics());

//     let dot = agent.graph().to_dot();

//     std::fs::create_dir_all("results/graphs").unwrap();

//     std::fs::write(format!("results/graphs/mcts_{}.dot", simulations), dot).unwrap();
// }
