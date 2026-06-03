mod api_client;
mod auth;
mod auth_store;
mod cli;
mod input;
mod output;
mod session;
mod shared_memory;
mod space;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Command};
use output::Output;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    let json = cli.json;
    let output = run(cli).await?;
    output.print_output(json)
}

#[inline]
async fn run(cli: Cli) -> Result<Output> {
    match cli.command {
        Command::Auth { command } => auth::run(command, &cli.base_url).await,
        Command::Spaces { command } => space::run_spaces(command, &cli.base_url).await,
        Command::Topology { space_id } => space::run_topology(space_id, &cli.base_url).await,
        Command::Digest {
            space_id,
            session_id,
        } => session::run_digest(space_id, session_id, &cli.base_url).await,
        Command::Insight { command } => session::run_insight(command, &cli.base_url).await,
        Command::SharedMemory { command } => shared_memory::run(command, &cli.base_url).await,
        Command::Fork(args) => space::run_fork(args, &cli.base_url).await,
    }
}
