use crate::{
    experiment::graphviz::{GraphvizConfig, exporter::GraphvizExporter, to_ascii},
    game::{board::Board, movement::Movement},
};
use std::{
    fs::File,
    io::{Result, Write},
    path::Path,
    process::Command,
};

#[derive(Clone)]
pub struct MinimaxAlphaBetaGraphNode {
    pub id: usize,

    pub board: String,
    pub value: Option<isize>,

    pub alpha: isize,
    pub beta: isize,

    pub depth: usize,

    pub pruned: bool,
}

#[derive(Clone)]
pub struct MinimaxAlphaBetaGraphEdge {
    pub from: usize,
    pub to: usize,

    pub movement: Movement,
}

pub struct MinimaxAlphaBetaGraph {
    pub config: GraphvizConfig,
    pub nodes: Vec<MinimaxAlphaBetaGraphNode>,
    pub edges: Vec<MinimaxAlphaBetaGraphEdge>,
}

impl MinimaxAlphaBetaGraph {
    pub fn new(max_nodes: usize, generate_graph: bool) -> Self {
        Self {
            config: GraphvizConfig {
                max_nodes,
                enabled: generate_graph,
            },
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn clear(&mut self) {
        self.nodes.clear();
    }

    pub fn create_node(
        &mut self,
        board: &Board,
        depth: usize,
        alpha: isize,
        beta: isize,
    ) -> Option<usize> {
        if self.nodes.len() >= self.config.max_nodes {
            return None;
        }

        let id = self.nodes.len();

        self.nodes.push(MinimaxAlphaBetaGraphNode {
            id,
            board: to_ascii(board),
            value: None,
            depth,
            alpha,
            beta,
            pruned: false,
        });

        Some(id)
    }

    pub fn connect(&mut self, parent: usize, child: usize, movement: Movement) {
        self.edges.push(MinimaxAlphaBetaGraphEdge {
            from: parent,
            to: child,
            movement,
        });
    }
}

impl GraphvizExporter for MinimaxAlphaBetaGraph {
    fn export<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path_ref = path.as_ref();

        if let Some(parent) = path_ref.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let dot_path = path_ref.with_extension("dot");
        let svg_path = path_ref.with_extension("svg");

        let mut dot_file = File::create(&dot_path)?;

        writeln!(dot_file, "digraph MinimaxAlphaBeta {{")?;
        writeln!(dot_file, "rankdir=TB;")?;
        writeln!(dot_file, "node [fontname=\"Courier New\"];")?;

        for node in &self.nodes {
            let color = if node.pruned { "#FFEFDF" } else { "white" };

            let board = node
                .board
                .replace('\\', "\\\\")
                .replace('"', "\\\"")
                .replace('\n', "\\l");

            let value = node
                .value
                .map(|v| v.to_string())
                .unwrap_or_else(|| "-".to_string());

            let alpha = if node.alpha == isize::MIN {
                "-∞"
            } else {
                &node.alpha.to_string()
            };
            let beta = if node.beta == isize::MAX {
                "∞"
            } else {
                &node.beta.to_string()
            };

            writeln!(
                dot_file,
                r#"
{} [
shape=box
style=filled
fillcolor="{}"
label="id: {}\ldepth: {}\lα: {}\lβ: {}\lvalue: {}\l\l{}\l"
];
"#,
                node.id, color, node.id, node.depth, alpha, beta, value, board,
            )?;
        }

        for edge in &self.edges {
            writeln!(
                dot_file,
                r#"{} -> {} [label="Col: {}"];"#,
                edge.from, edge.to, edge.movement.column,
            )?;
        }

        writeln!(dot_file, "}}")?;

        drop(dot_file);

        let status = Command::new("dot")
            .arg("-Tsvg")
            .arg(&dot_path)
            .arg("-o")
            .arg(&svg_path)
            .status()?;

        if !status.success() {
            return Err(std::io::Error::other("Graphviz failed to generate SVG."));
        }

        Ok(())
    }
}
