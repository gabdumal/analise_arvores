use crate::experiment::graphviz::exporter::GraphvizExporter;
use std::{
    fs::File,
    io::{Result, Write},
    path::Path,
};

#[derive(Clone)]
pub struct MinimaxAlphaBetaWithTranspositionTableGraphNode {
    pub id: usize,

    pub hash: usize,

    pub value: Option<isize>,

    pub depth: usize,

    pub children: Vec<usize>,
}

pub struct TranspositionEdge {
    pub from: usize,
    pub to: usize,
}

pub struct MinimaxAlphaBetaWithTranspositionTableGraph {
    pub nodes: Vec<MinimaxAlphaBetaWithTranspositionTableGraphNode>,

    pub transpositions: Vec<TranspositionEdge>,
}

impl GraphvizExporter for MinimaxAlphaBetaWithTranspositionTableGraph {
    fn export<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let mut file = File::create(path)?;

        writeln!(file, "digraph AlphaBetaTT {{")?;
        writeln!(file, "rankdir=TB;")?;

        for node in &self.nodes {
            writeln!(
                file,
                r#"
{} [
shape=box
label="id={}
hash={}
value={:?}"
];
"#,
                node.id, node.id, node.hash, node.value
            )?;

            for child in &node.children {
                writeln!(file, "{} -> {};", node.id, child)?;
            }
        }

        for edge in &self.transpositions {
            writeln!(
                file,
                "{} -> {} [style=dashed color=blue];",
                edge.from, edge.to
            )?;
        }

        writeln!(file, "}}")?;

        Ok(())
    }
}
