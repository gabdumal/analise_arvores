use crate::{
    agents::minimax_alpha_beta_with_transposition_table::transposition_table::NodeType,
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
pub struct MinimaxAlphaBetaWithTranspositionTableGraphNode {
    pub id: usize,

    pub player: Player,
    pub board: String,
    pub value: Option<isize>,

    pub alpha_in: isize,
    pub beta_in: isize,
    pub alpha_out: isize,
    pub beta_out: isize,

    pub depth: usize,
    pub cutoff_occurred: bool,

    pub tt_hit: bool,
    pub tt_depth: Option<usize>,
    pub tt_value: Option<isize>,
    pub tt_bound: Option<NodeType>,
    pub resolved_by_tt: bool,
    pub tt_source_node: Option<usize>,
}

#[derive(Clone)]
pub struct MinimaxAlphaBetaWithTranspositionTableGraphEdge {
    pub from: usize,
    pub to: usize,
    pub movement: Movement,
}

pub struct MinimaxAlphaBetaWithTranspositionTableGraph {
    pub config: GraphvizConfig,
    pub nodes: Vec<MinimaxAlphaBetaWithTranspositionTableGraphNode>,
    pub edges: Vec<MinimaxAlphaBetaWithTranspositionTableGraphEdge>,
}

impl MinimaxAlphaBetaWithTranspositionTableGraph {
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

        self.nodes
            .push(MinimaxAlphaBetaWithTranspositionTableGraphNode {
                id,
                player: board.current_player(),
                board: to_ascii(board),
                value: None,
                alpha_in: alpha,
                beta_in: beta,
                alpha_out: alpha,
                beta_out: beta,
                depth,
                cutoff_occurred: false,
                tt_hit: false,
                tt_bound: None,
                tt_depth: None,
                tt_value: None,
                resolved_by_tt: false,
                tt_source_node: None,
            });

        Some(id)
    }

    pub fn connect(&mut self, parent: usize, child: usize, movement: Movement) {
        self.edges
            .push(MinimaxAlphaBetaWithTranspositionTableGraphEdge {
                from: parent,
                to: child,
                movement,
            });
    }
}

fn format_value(value: isize) -> String {
    if value == isize::MIN {
        "-∞".to_string()
    } else if value == isize::MAX {
        "∞".to_string()
    } else {
        value.to_string()
    }
}

impl GraphvizExporter for MinimaxAlphaBetaWithTranspositionTableGraph {
    fn export<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path_ref = path.as_ref();
        if let Some(parent) = path_ref.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let dot_path = path_ref.with_extension("dot");
        let svg_path = path_ref.with_extension("svg");
        let mut dot_file = File::create(&dot_path)?;

        writeln!(
            dot_file,
            "digraph MinimaxAlphaBetaWithTranspositionTable {{"
        )?;

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
            let cutoff_border = if node.cutoff_occurred { 8 } else { 0 };
            let outer_color = if node.resolved_by_tt {
                "#5cd2f6"
            } else {
                "#5D88F5"
            };

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

            let tt_bound = node
                .tt_bound
                .as_ref()
                .map(|bound| format!("{bound:?}"))
                .unwrap_or_else(|| "-".to_string());

            let tt_depth = node
                .tt_depth
                .map(|d| d.to_string())
                .unwrap_or_else(|| "-".to_string());

            let tt_value = node
                .tt_value
                .map(|v| format_value(v).to_string())
                .unwrap_or_else(|| "-".to_string());

            let tt_source_node = node
                .tt_source_node
                .map(|v| v.to_string())
                .unwrap_or_else(|| "-".to_string());

            let label_html = format!(
                r##"<
<TABLE STYLE="ROUNDED" BORDER="{}" COLOR="{}" CELLSPACING="8">
    <TR><TD STYLE="ROUNDED" COLOR="{}" BORDER="4" CELLPADDING="8">
        <TABLE STYLE="ROUNDED" COLOR="BLACK" BORDER="0" CELLBORDER="1" CELLSPACING="0" CELLPADDING="8">

            <TR><TD ALIGN="LEFT"><B>ID</B></TD><TD ALIGN="RIGHT"><B>{}</B></TD></TR>
            <TR><TD ALIGN="LEFT">Depth</TD><TD ALIGN="RIGHT">{}</TD></TR>

            <TR><TD ALIGN="LEFT">α in</TD><TD ALIGN="RIGHT">{}</TD></TR>
            <TR><TD ALIGN="LEFT">α out</TD><TD ALIGN="RIGHT">{}</TD></TR>

            <TR><TD ALIGN="LEFT">β in</TD><TD ALIGN="RIGHT">{}</TD></TR>
            <TR><TD ALIGN="LEFT">β out</TD><TD ALIGN="RIGHT">{}</TD></TR>

            <TR><TD ALIGN="LEFT">Value</TD><TD ALIGN="RIGHT"><B>{}</B></TD></TR>

            <TR><TD BORDER="0" COLSPAN="2">&nbsp;</TD></TR>

            <TR><TD ALIGN="LEFT"><B>TT Hit</B></TD><TD ALIGN="RIGHT">{}</TD></TR>
            <TR><TD ALIGN="LEFT">Source</TD><TD ALIGN="RIGHT">{}</TD></TR>
            <TR><TD ALIGN="LEFT">Value</TD><TD ALIGN="RIGHT">{}</TD></TR>
            <TR><TD ALIGN="LEFT">Bound</TD><TD ALIGN="RIGHT">{}</TD></TR>
            <TR><TD ALIGN="LEFT">Depth</TD><TD ALIGN="RIGHT">{}</TD></TR>
            <TR><TD ALIGN="LEFT">Resolved</TD><TD ALIGN="RIGHT">{}</TD></TR>

            <TR><TD BORDER="0" COLSPAN="2">&nbsp;</TD></TR>

            <TR><TD BORDER="0" COLSPAN="2">{}</TD></TR>

        </TABLE>
    </TD></TR>
</TABLE>
>"##,
                cutoff_border,
                outer_color,
                player_border_color,
                node.id,
                node.depth,
                format_value(node.alpha_in),
                format_value(node.alpha_out),
                format_value(node.beta_in),
                format_value(node.beta_out),
                value,
                if node.tt_hit { "Yes" } else { "No" },
                tt_source_node,
                tt_value,
                tt_bound,
                tt_depth,
                if node.resolved_by_tt { "Yes" } else { "No" },
                board
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
