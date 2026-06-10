pub fn uct_score(parent_visits: usize, child_visits: usize, child_wins: f64) -> f64 {
    if child_visits == 0 {
        return f64::INFINITY;
    }

    let exploitation = child_wins / child_visits as f64;

    let exploration = (2.0 * (parent_visits as f64).ln() / child_visits as f64).sqrt();

    exploitation + exploration
}
