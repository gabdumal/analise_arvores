#[derive(Debug, Default, Clone)]
pub struct SearchMetrics {
    pub nodes_expanded: usize,
    pub nodes_evaluated: usize,
    pub leaf_nodes: usize,

    pub alpha_cutoffs: usize,
    pub beta_cutoffs: usize,

    pub tt_hits: usize,
    pub tt_misses: usize,
    pub tt_lookups: usize,
    pub tt_cutoffs: usize,
    pub tt_stores: usize,

    pub max_depth_reached: usize,

    pub peak_nodes_in_memory: usize,
    pub peak_frontier_size: usize,

    pub peak_tt_entries: usize,

    pub estimated_stack_memory_bytes: usize,
    pub peak_structure_memory_bytes: usize,

    pub simulations: usize,
    pub total_rollout_length: usize,

    pub elapsed_time_ns: u128,
}

#[derive(Debug, Default, Clone)]
pub struct DerivedMetrics {
    pub nanoseconds_per_node: f64,
    pub memory_per_node_bytes: f64,

    pub effective_branching_factor: f64,

    pub tt_hit_rate: f64,
    pub cutoff_rate: f64,

    pub average_rollout_length: f64,
}
