use crate::game::match_context::MatchContext;

pub struct VisualizationScenario {
    pub name: String,
    pub match_context: MatchContext,
}

pub enum VisualizationTarget {
    MinimaxAlphaBeta,
    MinimaxAlphaBetaWithTranspositionTable,
    MonteCarlo,
}

pub struct VisualizationRequest {
    pub target: VisualizationTarget,
    pub scenario: VisualizationScenario,
    pub limit: usize,
}
