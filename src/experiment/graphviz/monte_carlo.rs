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
pub struct MonteCarloGraphNode {
    pub id: usize,

    pub player: Player,
    pub board: String,

    //
    // Tree
    //
    pub depth: usize,

    //
    // MCTS statistics
    //
    pub visits: usize,
    pub total_reward: f64,

    //
    // Selection
    //
    pub uct_value: Option<f64>,

    //
    // Diagnostics
    //
    pub rollout_result: Option<f64>,
}

#[derive(Clone)]
pub struct MonteCarloGraphEdge {
    pub from: usize,
    pub to: usize,
    pub movement: Movement,
}

pub struct MonteCarloGraph {
    pub config: GraphvizConfig,
    pub nodes: Vec<MonteCarloGraphNode>,
    pub edges: Vec<MonteCarloGraphEdge>,
}

impl MonteCarloGraph {
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
        self.edges.clear();
    }

    pub fn create_node(&mut self, board: &Board, depth: usize) -> Option<usize> {
        if self.nodes.len() >= self.config.max_nodes {
            return None;
        }

        let id = self.nodes.len();

        self.nodes.push(MonteCarloGraphNode {
            id,

            player: board.current_player(),
            board: to_ascii(board),

            depth,

            visits: 0,
            total_reward: 0.0,

            uct_value: None,

            rollout_result: None,
        });

        Some(id)
    }

    pub fn connect(&mut self, parent: usize, child: usize, movement: Movement) {
        self.edges.push(MonteCarloGraphEdge {
            from: parent,
            to: child,
            movement,
        });
    }
}

impl GraphvizExporter for MonteCarloGraph {
    fn export<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path_ref = path.as_ref();

        if let Some(parent) = path_ref.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let dot_path = path_ref.with_extension("dot");
        let svg_path = path_ref.with_extension("svg");

        let mut dot_file = File::create(&dot_path)?;

        writeln!(dot_file, "digraph MonteCarlo {{")?;

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
            let player_border_color = if node.player == Player::Red {
                "#F5685D"
            } else {
                "#FFD65B"
            };

            let board = node.board.replace('\n', "<BR/>");

            let uct_value = node
                .uct_value
                .map(|v| format!("{:.2}", v))
                .unwrap_or_else(|| "-".to_string());

            let rollout_result = node
                .rollout_result
                .map(|v| format!("{:.2}", v))
                .unwrap_or_else(|| "-".to_string());

            let label_html = format!(
                r##"<
<TABLE STYLE="ROUNDED" BORDER="0" COLOR="#5D88F5" CELLSPACING="8">
    <TR>
        <TD STYLE="ROUNDED" COLOR="{}" BORDER="4" CELLPADDING="8">
            <TABLE STYLE="ROUNDED"
                   COLOR="BLACK"
                   BORDER="0"
                   CELLBORDER="1"
                   CELLSPACING="0"
                   CELLPADDING="8">

                <TR>
                    <TD ALIGN="LEFT"><B>ID</B></TD>
                    <TD ALIGN="RIGHT"><B>{}</B></TD>
                </TR>

                <TR>
                    <TD ALIGN="LEFT">Depth</TD>
                    <TD ALIGN="RIGHT">{}</TD>
                </TR>

                <TR>
                    <TD BORDER="0" COLSPAN="2">&nbsp;</TD>
                </TR>

                <TR>
                    <TD ALIGN="LEFT"><B>Visits</B></TD>
                    <TD ALIGN="RIGHT"><B>{}</B></TD>
                </TR>

                <TR>
                    <TD ALIGN="LEFT">Reward</TD>
                    <TD ALIGN="RIGHT">{:.2}</TD>
                </TR>

                <TR>
                    <TD ALIGN="LEFT">UCT</TD>
                    <TD ALIGN="RIGHT">{}</TD>
                </TR>

                <TR>
                    <TD ALIGN="LEFT">Rollout</TD>
                    <TD ALIGN="RIGHT">{}</TD>
                </TR>

                <TR>
                    <TD BORDER="0" COLSPAN="2">&nbsp;</TD>
                </TR>

                <TR>
                    <TD BORDER="0" COLSPAN="2">{}</TD>
                </TR>

            </TABLE>
        </TD>
    </TR>
</TABLE>
>"##,
                player_border_color,
                node.id,
                node.depth,
                node.visits,
                node.total_reward,
                uct_value,
                rollout_result,
                board,
            );

            writeln!(
                dot_file,
                r#"
{} [
label={}
];
"#,
                node.id, label_html
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
