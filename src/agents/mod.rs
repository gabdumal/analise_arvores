use crate::{
    agents::search_metrics::SearchMetrics,
    game::{match_context::MatchContext, movement::Movement},
};

pub mod minimax_alpha_beta;
mod search_metrics;

pub trait Agent {
    fn name(&self) -> &'static str;

    fn metrics(&self) -> &SearchMetrics;
    fn reset_metrics(&mut self);

    fn choose_movement(&mut self, game: &MatchContext) -> Movement;
}
