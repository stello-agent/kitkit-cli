use clap::{Args, Parser, Subcommand, ValueEnum};
use kitkit_sdk::DEFAULT_BASE_URL;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "kitkit-cli")]
#[command(about = "CLI for KitKit conversation-tree and memory APIs")]
pub struct Cli {
    #[arg(
        long,
        global = true,
        help = "Print JSON instead of human-readable tables"
    )]
    pub json: bool,

    #[arg(
        long,
        global = true,
        env = "KITKIT_BASE_URL",
        default_value = DEFAULT_BASE_URL,
        help = "KitKit API base URL"
    )]
    pub base_url: String,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Auth {
        #[command(subcommand)]
        command: AuthCommand,
    },
    Spaces {
        #[command(subcommand)]
        command: SpacesCommand,
    },
    Topology {
        space_id: String,
    },
    Digest {
        space_id: String,
        session_id: String,
    },
    Insight {
        #[command(subcommand)]
        command: InsightCommand,
    },
    SharedMemory {
        #[command(subcommand)]
        command: SharedMemoryCommand,
    },
    Fork(ForkArgs),
}

#[derive(Debug, Subcommand)]
pub enum AuthCommand {
    Login(LoginArgs),
    Status,
    Logout,
}

#[derive(Debug, Args)]
pub struct LoginArgs {
    #[arg(long)]
    pub email: Option<String>,

    #[arg(long, help = "Read the password from stdin instead of prompting")]
    pub password_stdin: bool,
}

#[derive(Debug, Subcommand)]
pub enum SpacesCommand {
    List,
    Get { space_id: String },
}

#[derive(Debug, Subcommand)]
pub enum InsightCommand {
    Put(InsightPutArgs),
}

#[derive(Debug, Args)]
pub struct InsightPutArgs {
    pub space_id: String,
    pub session_id: String,

    #[arg(long, conflicts_with_all = ["content_file", "stdin"])]
    pub content: Option<String>,

    #[arg(long = "content-file", conflicts_with = "stdin")]
    pub content_file: Option<PathBuf>,

    #[arg(long, help = "Read insight content from stdin")]
    pub stdin: bool,
}

#[derive(Debug, Subcommand)]
pub enum SharedMemoryCommand {
    List { space_id: String },
    Upsert(SharedMemoryUpsertArgs),
    Delete { space_id: String, slug: String },
}

#[derive(Debug, Args)]
pub struct SharedMemoryUpsertArgs {
    pub space_id: String,
    pub slug: String,

    #[arg(long, conflicts_with_all = ["body_file", "stdin"])]
    pub body: Option<String>,

    #[arg(long = "body-file", conflicts_with = "stdin")]
    pub body_file: Option<PathBuf>,

    #[arg(long, help = "Read shared-memory body from stdin")]
    pub stdin: bool,
}

#[derive(Debug, Args)]
pub struct ForkArgs {
    pub space_id: String,
    pub source_session_id: String,

    #[arg(long)]
    pub label: String,

    #[arg(long, value_enum)]
    pub context: Option<ForkContextArg>,

    #[arg(long)]
    pub system_prompt: Option<String>,

    #[arg(long)]
    pub consolidate_prompt: Option<String>,

    #[arg(long)]
    pub compress_prompt: Option<String>,

    #[arg(long)]
    pub fork_compress_prompt: Option<String>,

    #[arg(long = "skill")]
    pub skills: Vec<String>,

    #[arg(long)]
    pub prompt: Option<String>,

    #[arg(long)]
    pub topology_parent_id: Option<String>,

    #[arg(long)]
    pub profile: Option<String>,

    #[arg(long = "profile-var", value_parser = parse_key_value)]
    pub profile_vars: Vec<(String, String)>,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ForkContextArg {
    None,
    Inherit,
    Compress,
}

fn parse_key_value(value: &str) -> Result<(String, String), String> {
    let Some((key, value)) = value.split_once('=') else {
        return Err(format!("expected KEY=VALUE, got {value}"));
    };
    if key.is_empty() {
        return Err(format!("expected non-empty KEY in {value}"));
    }
    Ok((key.to_string(), value.to_string()))
}
