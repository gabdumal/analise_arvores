use super::{node::MonteCarloNode, uct::uct_score};

pub fn best_child(arena: &[MonteCarloNode], node_index: usize) -> usize {
    let node = &arena[node_index];

    let parent_visits = node.visits;

    *node
        .children
        .iter()
        .max_by(|&&a, &&b| {
            let score_a = uct_score(parent_visits, arena[a].visits, arena[a].wins);

            let score_b = uct_score(parent_visits, arena[b].visits, arena[b].wins);

            score_a.partial_cmp(&score_b).unwrap()
        })
        .unwrap()
}
