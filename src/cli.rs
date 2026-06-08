use clap::{Args, Parser, Subcommand, ValueEnum};
use kitkit_sdk::DEFAULT_BASE_URL;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "kitkit-cli")]
#[command(version)]
#[command(
    about = "kitkit-cli is a command-line wrapper around KitKit's REST APIs for agents that need Kitty-like, space-level awareness.\n\nIt intentionally focuses on global conversation-tree operations instead of ordinary session chat: list spaces, inspect a space topology, read a single session L2 digest, push insights, edit shared memory, and create forks."
)]
#[command(
    after_help = "Workflow:\n  1. Run `kitkit-cli auth login` once to cache tokens in the platform config directory.\n  2. Use `kitkit-cli spaces list` to choose a space.\n  3. Use `kitkit-cli topology <SPACE_ID>` to find session ids.\n  4. Use `digest`, `insight put`, `shared-memory`, and `fork` for Kitty/main-agent work.\n\nUse --json on any command when another program or agent should parse the output."
)]
pub struct Cli {
    #[arg(
        long,
        global = true,
        help = "Print machine-readable JSON instead of human-readable output.\n\nJSON output is produced from the strongly typed SDK response structs and is intended for Codex, Claude Code, scripts, and other agent tooling."
    )]
    pub json: bool,

    #[arg(
        long,
        global = true,
        env = "KITKIT_BASE_URL",
        default_value = DEFAULT_BASE_URL,
        value_name = "URL",
        help = "KitKit REST API base URL.\n\nDefaults to the production API. Set KITKIT_BASE_URL or pass --base-url when targeting staging, local development, or a self-hosted deployment."
    )]
    pub base_url: String,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    #[command(
        about = "Authenticate and manage cached KitKit tokens.\n\nLogin is interactive by default. Tokens are cached in the platform config directory so later commands can call authenticated APIs without receiving credentials on every invocation.",
        after_help = "Subcommands:\n  login   authenticate against the selected KitKit API base URL\n  status  verify the cached token and show the cached account\n  logout  remove the local token cache\n\nExamples:\n  kitkit-cli auth login\n  kitkit-cli auth status\n  kitkit-cli --base-url http://localhost:3000 auth login"
    )]
    Auth {
        #[command(subcommand)]
        command: AuthCommand,
    },
    #[command(
        about = "List spaces or show one space's display metadata.\n\nIn KitKit, a space is the boundary for one conversation tree, shared memory store, and Kitty/main-agent context. The list endpoint intentionally returns only stable identity fields: id and label. Use `spaces get` when you need the space description and timestamps.",
        after_help = "Subcommands:\n  list  choose a space by id and label\n  get   inspect the display metadata for one space\n\nExamples:\n  kitkit-cli spaces list\n  kitkit-cli spaces get <SPACE_ID>\n  kitkit-cli --json spaces list"
    )]
    Spaces {
        #[command(subcommand)]
        command: SpacesCommand,
    },
    #[command(
        about = "Print the session topology tree for a space.\n\nTopology is the first thing a Kitty-like agent should inspect. It gives each session id, label, status, turn count, and child relationship so follow-up commands can target a specific node.",
        after_help = "Example:\n  kitkit-cli topology <SPACE_ID>\n  kitkit-cli --json topology <SPACE_ID>"
    )]
    Topology {
        #[arg(
            value_name = "SPACE_ID",
            help = "Space UUID whose conversation tree should be loaded"
        )]
        space_id: String,
    },
    #[command(
        about = "Read one session's L2 digest and current insight.\n\nA digest is the compact L2 view for a single session: session metadata, memory summary, and the current insight slot. This command does not bulk-read the whole tree and does not return L3 conversation records.",
        after_help = "Example:\n  kitkit-cli digest <SPACE_ID> <SESSION_ID>\n  kitkit-cli --json digest <SPACE_ID> <SESSION_ID>"
    )]
    Digest {
        #[arg(
            value_name = "SPACE_ID",
            help = "Space UUID that owns the target session"
        )]
        space_id: String,
        #[arg(
            value_name = "SESSION_ID",
            help = "Session UUID from the topology tree"
        )]
        session_id: String,
    },
    #[command(
        about = "Push cross-session insights into target sessions.\n\nInsight is a single slot attached to a session. It is meant for cross-branch findings: contradictions, useful context, or recommendations discovered by a Kitty/main-style agent. Repeated writes overwrite the previous insight.",
        after_help = "Subcommands:\n  put  write the target session's insight slot\n\nTypical flow:\n  1. Run `topology <SPACE_ID>` to identify the target session.\n  2. Run `digest <SPACE_ID> <SESSION_ID>` if you need the current L2 context.\n  3. Run `insight put` to push a concise finding into that session."
    )]
    Insight {
        #[command(subcommand)]
        command: InsightCommand,
    },
    #[command(
        about = "Read or edit space-level shared memory.\n\nShared memory is global to the space, not to one session. Kitty receives these entries in its system context every turn, so use it for durable facts such as user preferences, project background, and long-lived goals.",
        after_help = "Subcommands:\n  list    inspect all durable shared-memory entries\n  upsert  create or replace one entry by slug\n  delete  remove one entry by slug\n\nExamples:\n  kitkit-cli shared-memory list <SPACE_ID>\n  kitkit-cli shared-memory upsert <SPACE_ID> project-background --body-file background.md\n  kitkit-cli shared-memory delete <SPACE_ID> project-background"
    )]
    SharedMemory {
        #[command(subcommand)]
        command: SharedMemoryCommand,
    },
    #[command(
        about = "Create a child fork session from an existing source session.\n\nForks are topology nodes. The source session provides the parent conversation context, while the optional topology parent can place the new node elsewhere in the visible tree. The API supports no context, inherited context, or compressed context, plus per-session prompt and skill overrides.",
        after_help = "Examples:\n  kitkit-cli fork <SPACE_ID> <SESSION_ID> --label \"Research branch\" --context none\n  kitkit-cli fork <SPACE_ID> <SESSION_ID> --label \"Compressed branch\" --context compress --prompt \"Start from the key tradeoffs\"\n  kitkit-cli --json fork <SPACE_ID> <SESSION_ID> --label \"Profile branch\" --profile product --profile-var audience=developer"
    )]
    Fork(ForkArgs),
}

#[derive(Debug, Subcommand)]
pub enum AuthCommand {
    #[command(
        about = "Log in and cache access/refresh tokens.\n\nWithout --email, the CLI prompts for the account email. Without --password-stdin, it prompts for the password without echoing it. The cached token is scoped to the current --base-url.",
        after_help = "Examples:\n  kitkit-cli auth login\n  kitkit-cli auth login --email user@example.com\n  printf '%s' \"KITKIT_PASSWORD\" | kitkit-cli auth login --email \"KITKIT_EMAIL\" --password-stdin\n\nAfter login, run `kitkit-cli auth status` to confirm the cached account."
    )]
    Login(LoginArgs),
    #[command(
        about = "Show cached authentication state.\n\nThis command validates the cached access token, refreshes it when possible, and prints the account associated with the cache.",
        after_help = "Examples:\n  kitkit-cli auth status\n  kitkit-cli --json auth status\n\nUse this before scripting API calls when you need to know which KitKit account and base URL are active."
    )]
    Status,
    #[command(
        about = "Remove cached authentication tokens from the platform config directory.\n\nThis only clears the local cache; it does not delete the KitKit account and does not alter any space data.",
        after_help = "Examples:\n  kitkit-cli auth logout\n  kitkit-cli --json auth logout\n\nRun `kitkit-cli auth login` again to create a new local token cache."
    )]
    Logout,
}

#[derive(Debug, Args)]
pub struct LoginArgs {
    #[arg(
        long,
        value_name = "EMAIL",
        help = "KitKit account email; prompts interactively when omitted"
    )]
    pub email: Option<String>,

    #[arg(
        long,
        help = "Read the password from stdin instead of prompting.\n\nUseful for non-interactive scripts, for example: `printf '%s' \"KITKIT_PASSWORD\" | kitkit-cli auth login --email \"KITKIT_EMAIL\" --password-stdin`."
    )]
    pub password_stdin: bool,
}

#[derive(Debug, Subcommand)]
pub enum SpacesCommand {
    #[command(
        about = "List spaces visible to the authenticated user.\n\nThe REST response intentionally contains only id and label so the output is small and stable for agents. Pick a space id from this output before calling topology, digest, insight, shared-memory, or fork.",
        after_help = "Examples:\n  kitkit-cli spaces list\n  kitkit-cli --json spaces list\n\nHuman output is a small table. JSON output has the shape `{ \"data\": [{ \"id\": \"...\", \"label\": \"...\" }] }`."
    )]
    List,
    #[command(
        about = "Show one space's display metadata.\n\nThis returns display fields such as label, description, pinned timestamp, and creation/update timestamps. Runtime configuration, model profile, capability, and prompt configuration are intentionally not part of this CLI surface.",
        after_help = "Examples:\n  kitkit-cli spaces get <SPACE_ID>\n  kitkit-cli --json spaces get <SPACE_ID>\n\nUse this command when an agent needs human-facing space context before inspecting the topology."
    )]
    Get {
        #[arg(value_name = "SPACE_ID", help = "Space UUID returned by `spaces list`")]
        space_id: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum InsightCommand {
    #[command(
        about = "Write an insight into one active session.\n\nThe target session must belong to the given space and must be active. Each session has one insight slot; writing a new insight replaces the previous one. Content must be non-empty and is limited by the server to 4000 characters.",
        after_help = "Examples:\n  kitkit-cli insight put <SPACE_ID> <SESSION_ID> --content \"Branch A contradicts branch B on pricing.\"\n  cat insight.md | kitkit-cli insight put <SPACE_ID> <SESSION_ID> --stdin\n  kitkit-cli --json insight put <SPACE_ID> <SESSION_ID> --content-file insight.md"
    )]
    Put(InsightPutArgs),
}

#[derive(Debug, Args)]
pub struct InsightPutArgs {
    #[arg(
        value_name = "SPACE_ID",
        help = "Space UUID that owns the target session"
    )]
    pub space_id: String,
    #[arg(
        value_name = "SESSION_ID",
        help = "Active session UUID that should receive the insight"
    )]
    pub session_id: String,

    #[arg(
        long,
        value_name = "TEXT",
        conflicts_with_all = ["content_file", "stdin"],
        help = "Insight text to write directly on the command line"
    )]
    pub content: Option<String>,

    #[arg(
        long = "content-file",
        value_name = "PATH",
        conflicts_with = "stdin",
        help = "Read insight text from a UTF-8 file"
    )]
    pub content_file: Option<PathBuf>,

    #[arg(
        long,
        help = "Read insight text from stdin.\n\nExactly one of --content, --content-file, or --stdin is required."
    )]
    pub stdin: bool,
}

#[derive(Debug, Subcommand)]
pub enum SharedMemoryCommand {
    #[command(
        about = "List all shared memory entries for a space.\n\nShared memory is global space context. It is appropriate for durable facts that should be visible to Kitty/main-style agents across sessions. Unlike digest, it is not scoped to one session.",
        after_help = "Examples:\n  kitkit-cli shared-memory list <SPACE_ID>\n  kitkit-cli --json shared-memory list <SPACE_ID>\n\nUse this before writing a new entry if you need to avoid overwriting an existing slug."
    )]
    List {
        #[arg(
            value_name = "SPACE_ID",
            help = "Space UUID whose shared memory should be listed"
        )]
        space_id: String,
    },
    #[command(
        about = "Create or replace one shared memory entry.\n\nThe slug is the stable entry key. Upsert overwrites the body for an existing slug or creates it when missing. The server requires a non-empty body and limits slugs to 128 characters.",
        after_help = "Examples:\n  kitkit-cli shared-memory upsert <SPACE_ID> user-profile --body \"Prefers concise technical answers.\"\n  kitkit-cli shared-memory upsert <SPACE_ID> project-background --body-file background.md\n  cat memory.md | kitkit-cli shared-memory upsert <SPACE_ID> project-background --stdin"
    )]
    Upsert(SharedMemoryUpsertArgs),
    #[command(
        about = "Delete one shared memory entry by slug.\n\nDeleting a missing slug is treated as a no-op by the server. This changes space-level memory only; it does not delete sessions, digests, insights, or conversation records.",
        after_help = "Examples:\n  kitkit-cli shared-memory delete <SPACE_ID> project-background\n  kitkit-cli --json shared-memory delete <SPACE_ID> project-background\n\nRun `kitkit-cli shared-memory list <SPACE_ID>` afterwards if you need to confirm the remaining entries."
    )]
    Delete {
        #[arg(
            value_name = "SPACE_ID",
            help = "Space UUID whose shared memory should be edited"
        )]
        space_id: String,
        #[arg(value_name = "SLUG", help = "Shared memory entry key to delete")]
        slug: String,
    },
}

#[derive(Debug, Args)]
pub struct SharedMemoryUpsertArgs {
    #[arg(
        value_name = "SPACE_ID",
        help = "Space UUID whose shared memory should be edited"
    )]
    pub space_id: String,
    #[arg(
        value_name = "SLUG",
        help = "Shared memory entry key; max 128 characters on the server"
    )]
    pub slug: String,

    #[arg(
        long,
        value_name = "TEXT",
        conflicts_with_all = ["body_file", "stdin"],
        help = "Entry body to write directly on the command line"
    )]
    pub body: Option<String>,

    #[arg(
        long = "body-file",
        value_name = "PATH",
        conflicts_with = "stdin",
        help = "Read the entry body from a UTF-8 file"
    )]
    pub body_file: Option<PathBuf>,

    #[arg(
        long,
        help = "Read the entry body from stdin.\n\nExactly one of --body, --body-file, or --stdin is required."
    )]
    pub stdin: bool,
}

#[derive(Debug, Args)]
pub struct ForkArgs {
    #[arg(
        value_name = "SPACE_ID",
        help = "Space UUID that owns the source session"
    )]
    pub space_id: String,
    #[arg(
        value_name = "SOURCE_SESSION_ID",
        help = "Session UUID to fork from; find this in `topology`"
    )]
    pub source_session_id: String,

    #[arg(long, value_name = "LABEL", help = "Label for the new child session")]
    pub label: String,

    #[arg(
        long,
        value_enum,
        value_name = "MODE",
        help = "Parent context mode for the fork.\n\nnone: create the child without parent conversation context.\ninherit: carry parent context directly.\ncompress: create the child from compressed parent context and allow KitKit to generate the initial compressed handoff."
    )]
    pub context: Option<ForkContextArg>,

    #[arg(
        long,
        value_name = "TEXT",
        help = "Override the new session system prompt"
    )]
    pub system_prompt: Option<String>,

    #[arg(
        long,
        value_name = "TEXT",
        help = "Override the prompt used when consolidating this session into L2 memory"
    )]
    pub consolidate_prompt: Option<String>,

    #[arg(
        long,
        value_name = "TEXT",
        help = "Override the prompt used for regular session compression"
    )]
    pub compress_prompt: Option<String>,

    #[arg(
        long,
        value_name = "TEXT",
        help = "Override the prompt used when this session is later fork-compressed"
    )]
    pub fork_compress_prompt: Option<String>,

    #[arg(
        long = "skill",
        value_name = "NAME",
        help = "Enable a named space skill for the new session; repeat for multiple skills"
    )]
    pub skills: Vec<String>,

    #[arg(
        long,
        value_name = "TEXT",
        help = "Optional first user prompt to send into the new fork"
    )]
    pub prompt: Option<String>,

    #[arg(
        long,
        value_name = "SESSION_ID",
        help = "Place the new topology node under this parent instead of the source session"
    )]
    pub topology_parent_id: Option<String>,

    #[arg(
        long,
        value_name = "NAME",
        help = "Apply a fork profile from the space capability configuration"
    )]
    pub profile: Option<String>,

    #[arg(
        long = "profile-var",
        value_name = "KEY=VALUE",
        value_parser = parse_key_value,
        help = "Reserved profile variable passed to the fork API; repeat for multiple variables"
    )]
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
