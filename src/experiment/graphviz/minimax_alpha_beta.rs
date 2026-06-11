use crate::{
    experiment::graphviz::{GraphvizConfig, exporter::GraphvizExporter, to_ascii},
    game::{board::Board, movement::Movement, player::Player},
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

    pub player: Player,
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
            player: board.current_player(),
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

        writeln!(
            dot_file,
            r#"
fontname="Atkinson Hyperlegible Mono";
rankdir=TB;
splines=polyline;
concentrate=true;
nodesep=1;
ranksep=4;
            "#
        )?;

        writeln!(
            dot_file,
            r#"
node [
    fontname="Atkinson Hyperlegible Mono"
    fontsize="32"
    shape=none
    border=0
];
"#
        )?;

        writeln!(
            dot_file,
            r#"
edge [
    fontname="Atkinson Hyperlegible Mono"
    fontsize="64"
    penwidth=2
    labeldistance=1
    labelangle=0
];
"#
        )?;

        for node in &self.nodes {
            let pruned_border = if node.pruned { 8 } else { 0 };
            let player_border_color = if node.player == Player::Red {
                "#F5685D"
            } else {
                "#FFD65B"
            };

            let board = node.board.replace('\n', "<BR/>");

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

            let label_html = format!(
                r##"<
<TABLE STYLE="ROUNDED" BORDER="{}" COLOR="#5D88F5" CELLSPACING="8">
    <TR><TD STYLE="ROUNDED" COLOR="{}" BORDER="4" CELLPADDING="8">
        <TABLE STYLE="ROUNDED" COLOR="BLACK" BORDER="0" CELLBORDER="1" CELLSPACING="0" CELLPADDING="8">
            <TR><TD ALIGN="LEFT"><B>ID</B></TD><TD ALIGN="RIGHT"><B>{}</B></TD></TR>
            <TR><TD ALIGN="LEFT">Depth</TD><TD ALIGN="RIGHT">{}</TD></TR>
            <TR><TD ALIGN="LEFT">α</TD><TD ALIGN="RIGHT">{}</TD></TR>
            <TR><TD ALIGN="LEFT">β</TD><TD ALIGN="RIGHT">{}</TD></TR>
            <TR><TD ALIGN="LEFT">Value</TD><TD ALIGN="RIGHT"><B>{}</B></TD></TR>
            <TR><TD BORDER="0" COLSPAN="2">&nbsp;</TD></TR>
            <TR><TD BORDER="0" COLSPAN="2">{}</TD></TR>
        </TABLE>
    </TD></TR>
</TABLE>
>"##,
                pruned_border, player_border_color, node.id, node.depth, alpha, beta, value, board
            );
            writeln!(
                dot_file,
                r#"
{} [
label={}
];
                "#,
                node.id, label_html,
            )?;
        }

        for edge in &self.edges {
            writeln!(
                dot_file,
                r##"
{}:s -> {}:n [headlabel=<
<TABLE BORDER="0">
    <TR><TD STYLE="ROUNDED" BORDER="1" CELLPADDING="8" BGCOLOR="#edf2fc">{}</TD></TR>
    <TR><TD BORDER="0">&nbsp;</TD></TR>
</TABLE>
>];
                "##,
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
