#[derive(Debug, Default, Clone)]
pub struct SearchMetrics {
    pub alpha_cutoffs: usize,
    pub beta_cutoffs: usize,

    pub nodes_expanded: usize,
    pub leaf_nodes: usize,
    pub max_depth_reached: usize,
    pub peak_nodes_in_memory: usize,
    pub peak_frontier_size: usize,

    pub estimated_stack_memory_bytes: usize,
    pub elapsed_time_ns: u128,

    pub tt_hits: usize,
    pub tt_misses: usize,
    pub tt_stores: usize,
}
