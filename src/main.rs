use crate::{
    agents::{
        Agent, minimax_alpha_beta::MinimaxAlphaBeta,
        minimax_alpha_beta_with_transposition_table::MinimaxAlphaBetaWithTranspositionTable,
    },
    game::match_context::MatchContext,
};

mod agents;
mod game;

fn alpha_beta() {
    let match_context = MatchContext::new();

    {
        let mut agent = MinimaxAlphaBeta::new(6);
        let movement = agent.choose_movement(&match_context);
        println!("Depth limit: 6. Chosen column: {}.", movement.column);
        println!("{:#?}", agent.metrics());
    }

    {
        let mut agent = MinimaxAlphaBeta::new(7);
        let movement = agent.choose_movement(&match_context);
        println!("Depth limit: 7. Chosen column: {}.", movement.column);
        println!("{:#?}", agent.metrics());
    }

    {
        let mut agent = MinimaxAlphaBeta::new(8);
        let movement = agent.choose_movement(&match_context);
        println!("Depth limit: 8. Chosen column: {}.", movement.column);
        println!("{:#?}", agent.metrics());
    }
}

fn alpha_beta_with_transposition_table() {
    let game = MatchContext::new();

    let mut ab = MinimaxAlphaBeta::new(8);

    let mut tt = MinimaxAlphaBetaWithTranspositionTable::new(8);

    let ab_move = ab.choose_movement(&game);

    println!("AB: {:?}\n{:#?}", ab_move, ab.metrics());

    let tt_move = tt.choose_movement(&game);

    println!("TT: {:?}\n{:#?}", tt_move, tt.metrics());
}

fn main() {
    alpha_beta();
    alpha_beta_with_transposition_table();
}
