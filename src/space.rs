use crate::api_client::authenticated_client;
use crate::cli::{ForkArgs, ForkContextArg, SpacesCommand};
use crate::output::{
    Output, Printer, ReadablePrinter, empty_text, kv_table, status_text, table_with_header,
};
use anyhow::Result;
use kitkit_sdk::spaces::{
    self as spaces_api, ForkContext, ForkSessionRequest, ForkSessionResponse, ListSpacesResponse,
    SessionTreeNode, Space,
};
use std::collections::HashMap;

pub async fn run_spaces(command: SpacesCommand, base_url: &str) -> Result<Output> {
    let client = authenticated_client(base_url).await?;
    match command {
        SpacesCommand::List => Ok(Output::SpacesList(spaces_api::list(&client).await?)),
        SpacesCommand::Get { space_id } => {
            Ok(Output::Space(spaces_api::get(&client, &space_id).await?))
        }
    }
}

pub async fn run_topology(space_id: String, base_url: &str) -> Result<Output> {
    let client = authenticated_client(base_url).await?;
    Ok(Output::Topology(
        spaces_api::topology(&client, &space_id).await?,
    ))
}

pub async fn run_fork(args: ForkArgs, base_url: &str) -> Result<Output> {
    let client = authenticated_client(base_url).await?;
    let profile_vars = if args.profile_vars.is_empty() {
        None
    } else {
        Some(args.profile_vars.into_iter().collect::<HashMap<_, _>>())
    };
    let request = ForkSessionRequest {
        label: args.label,
        system_prompt: args.system_prompt,
        consolidate_prompt: args.consolidate_prompt,
        compress_prompt: args.compress_prompt,
        fork_compress_prompt: args.fork_compress_prompt,
        skills: (!args.skills.is_empty()).then_some(args.skills),
        prompt: args.prompt,
        context: args.context.map(fork_context),
        topology_parent_id: args.topology_parent_id,
        profile: args.profile,
        profile_vars,
    };
    Ok(Output::Fork(
        spaces_api::fork_session(&client, &args.space_id, &args.source_session_id, &request)
            .await?,
    ))
}

impl Printer for ReadablePrinter<ListSpacesResponse> {
    fn print_output(self) -> Result<()> {
        let mut table = table_with_header(["id", "label"]);
        for space in &self.inner.data {
            table.add_row([space.id.as_str(), space.label.as_str()]);
        }
        println!("{table}");
        Ok(())
    }
}

impl Printer for ReadablePrinter<Space> {
    fn print_output(self) -> Result<()> {
        let out = &self.inner;
        let mut table = kv_table();
        table.add_row(["id", out.id.as_str()]);
        table.add_row(["label", out.label.as_str()]);
        table.add_row(["description", empty_text(&out.description)]);
        table.add_row(["pinned_at", out.pinned_at.as_deref().unwrap_or("")]);
        table.add_row(["created_at", out.created_at.as_str()]);
        table.add_row(["updated_at", out.updated_at.as_str()]);
        println!("{table}");
        Ok(())
    }
}

impl Printer for ReadablePrinter<SessionTreeNode> {
    fn print_output(self) -> Result<()> {
        println!("{}", render_topology_tree(&self.inner));
        Ok(())
    }
}

impl Printer for ReadablePrinter<ForkSessionResponse> {
    fn print_output(self) -> Result<()> {
        let out = &self.inner;
        let mut table = kv_table();
        table.add_row(["id", out.id.as_str()]);
        table.add_row(["label", out.label.as_str()]);
        table.add_row(["parent_id", out.parent_id.as_deref().unwrap_or("")]);
        table.add_row(["depth", &out.depth.to_string()]);
        table.add_row(["index", &out.index.to_string()]);
        println!("{table}");
        Ok(())
    }
}

fn render_topology_tree(root: &SessionTreeNode) -> String {
    let mut lines = vec![format_topology_node(root)];
    for (index, child) in root.children.iter().enumerate() {
        add_topology_tree_lines(&mut lines, child, "", index + 1 == root.children.len());
    }
    lines.join("\n")
}

fn add_topology_tree_lines(
    lines: &mut Vec<String>,
    node: &SessionTreeNode,
    prefix: &str,
    is_last: bool,
) {
    let connector = if is_last { "└── " } else { "├── " };
    lines.push(format!("{prefix}{connector}{}", format_topology_node(node)));

    let child_prefix = if is_last {
        format!("{prefix}    ")
    } else {
        format!("{prefix}│   ")
    };
    for (index, child) in node.children.iter().enumerate() {
        add_topology_tree_lines(
            lines,
            child,
            &child_prefix,
            index + 1 == node.children.len(),
        );
    }
}

fn format_topology_node(node: &SessionTreeNode) -> String {
    format!(
        "{} ({}) [{}, {}]",
        node.label,
        node.id,
        status_text(node.status),
        turn_count_text(node.turn_count)
    )
}

fn turn_count_text(turn_count: u64) -> String {
    let unit = if turn_count == 1 { "turn" } else { "turns" };
    format!("{turn_count} {unit}")
}

fn fork_context(context: ForkContextArg) -> ForkContext {
    match context {
        ForkContextArg::None => ForkContext::None,
        ForkContextArg::Inherit => ForkContext::Inherit,
        ForkContextArg::Compress => ForkContext::Compress,
    }
}
