use crate::experiment::graphviz::exporter::GraphvizExporter;
use std::{
    fs::File,
    io::{Result, Write},
    path::Path,
};

#[derive(Clone)]
pub struct MonteCarloGraphNode {
    pub id: usize,

    pub visits: usize,

    pub wins: f64,

    pub movement: Option<usize>,

    pub children: Vec<usize>,
}

pub struct MonteCarloGraph {
    pub nodes: Vec<MonteCarloGraphNode>,
}

impl GraphvizExporter for MonteCarloGraph {
    fn export<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut file = File::create(path)?;

        writeln!(file, "digraph MCTS {{")?;
        writeln!(file, "rankdir=TB;")?;

        for node in &self.nodes {
            writeln!(
                file,
                r#"
{} [
shape=ellipse
label="id={}
move={:?}
visits={}
wins={:.2}"
];
"#,
                node.id, node.id, node.movement, node.visits, node.wins
            )?;

            for child in &node.children {
                let width = ((self.nodes[*child].visits as f64).ln() + 1.0).max(1.0);

                writeln!(file, "{} -> {} [penwidth={}];", node.id, child, width)?;
            }
        }

        writeln!(file, "}}")?;

        Ok(())
    }
}
