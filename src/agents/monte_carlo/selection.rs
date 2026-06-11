use crate::agents::monte_carlo::{node::MonteCarloNode, uct::uct_score};

pub fn best_child(arena: &[MonteCarloNode], node_index: usize) -> (usize, Vec<(usize, f64)>) {
    let node = &arena[node_index];

    let parent_visits = node.visits;

    let mut scores = Vec::with_capacity(node.children.len());

    let mut best_child = node.children[0];
    let mut best_score = f64::NEG_INFINITY;

    for &child_index in &node.children {
        let score = uct_score(
            parent_visits,
            arena[child_index].visits,
            arena[child_index].wins,
        );

        scores.push((child_index, score));

        if score > best_score {
            best_score = score;
            best_child = child_index;
        }
    }

    (best_child, scores)
}
