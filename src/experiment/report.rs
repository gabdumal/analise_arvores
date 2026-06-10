use crate::{
    agents::search_metrics::{DerivedMetrics, SearchMetrics},
    game::movement::Movement,
};

pub struct ExperimentReport {
    pub algorithm: String,
    pub parameter: usize,

    pub movement: Movement,

    pub metrics: SearchMetrics,
    pub derived: DerivedMetrics,
}
