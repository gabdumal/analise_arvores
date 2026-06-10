use crate::{
    agents::{Agent, minimax_alpha_beta::MinimaxAlphaBeta},
    game::match_context::MatchContext,
};

mod agents;
mod game;

fn main() {
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
