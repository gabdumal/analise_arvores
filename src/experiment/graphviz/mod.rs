pub mod exporter;
pub mod minimax_alpha_beta;
pub mod minimax_alpha_beta_with_transposition_table;
pub mod monte_carlo;

#[derive(Default)]
pub struct GraphvizConfig {
    pub enabled: bool,
    pub max_nodes: usize,
}
