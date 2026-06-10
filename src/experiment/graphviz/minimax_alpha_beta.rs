use crate::experiment::graphviz::{GraphvizConfig, exporter::GraphvizExporter};
use std::{
    fs::File,
    io::{Result, Write},
    path::Path,
};

#[derive(Clone)]
pub struct MinimaxAlphaBetaGraphNode {
    pub id: usize,

    pub value: Option<isize>,

    pub alpha: isize,
    pub beta: isize,

    pub depth: usize,

    pub pruned: bool,

    pub children: Vec<usize>,
}

pub struct MinimaxAlphaBetaGraph {
    pub config: GraphvizConfig,
    pub nodes: Vec<MinimaxAlphaBetaGraphNode>,
}

impl MinimaxAlphaBetaGraph {
    pub fn new(max_nodes: usize, generate_graph: bool) -> Self {
        Self {
            config: GraphvizConfig {
                max_nodes,
                enabled: generate_graph,
            },
            nodes: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
    }

    pub fn create_node(&mut self, depth: usize, alpha: isize, beta: isize) -> Option<usize> {
        if self.nodes.len() >= self.config.max_nodes {
            return None;
        }

        let id = self.nodes.len();

        self.nodes.push(MinimaxAlphaBetaGraphNode {
            id,
            depth,
            alpha,
            beta,
            value: None,
            pruned: false,
            children: Vec::new(),
        });

        Some(id)
    }

    pub fn connect(&mut self, parent: usize, child: usize) {
        self.nodes[parent].children.push(child);
    }
}

impl GraphvizExporter for MinimaxAlphaBetaGraph {
    fn export<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path_ref = path.as_ref();

        if let Some(parent) = path_ref.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut file = File::create(path_ref)?;

        writeln!(file, "digraph MinimaxAlphaBeta {{")?;

        writeln!(file, "rankdir=TB;")?;

        for node in &self.nodes {
            let color = if node.pruned { "bisque" } else { "white" };

            writeln!(
                file,
                r#"
{} [
shape=box
style=filled
fillcolor="{}"
label="id: {}
depth: {}
α: {}
β: {}
value: {}"
];
"#,
                node.id,
                color,
                node.id,
                node.depth,
                node.alpha,
                node.beta,
                if let Some(node_value) = node.value {
                    node_value.to_string()
                } else {
                    String::from("-")
                },
            )?;

            for child in &node.children {
                writeln!(file, "{} -> {};", node.id, child)?;
            }
        }

        writeln!(file, "}}")?;

        Ok(())
    }
}
