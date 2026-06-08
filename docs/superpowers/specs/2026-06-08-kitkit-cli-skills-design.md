# KitKit CLI Skills Design

## Goal

Create a composable KitKit skills pack under `skills/` so Codex, Claude Code, and similar agents can use `kitkit-cli` with Kitty-like behavior:

- Maintain space-level awareness across a KitKit conversation tree.
- Treat the Stello/KitKit session tree as a compact document library.
- Read targeted session digests instead of assuming full transcript access.
- Read and maintain shared memory cautiously.
- Push concise cross-branch insights to the right session.
- Create forks with the right context mode and overrides.

## Non-Goals

- Do not add `agents/openai.yaml`.
- Do not change the CLI command surface in this pass.
- Do not duplicate the full CLI help text inside skills.
- Do not create scripts, assets, or large references unless later usage shows repeated mechanical work.

## Skill Layout

Create these directories:

```text
skills/
├── kitkit-cli/
│   └── SKILL.md
├── kitkit-conversation-tree/
│   └── SKILL.md
├── kitkit-insight/
│   └── SKILL.md
├── kitkit-fork/
│   └── SKILL.md
└── kitkit-as-kitty/
    └── SKILL.md
```

Each `SKILL.md` uses only the required frontmatter fields: `name` and `description`.

## Responsibilities

### `kitkit-cli`

Provide the foundation skill. Explain the KitKit mental model and the CLI's role:

- A space owns one conversation tree and one shared-memory store.
- A session is a node in the tree.
- `topology` gives the tree and session ids.
- `digest` gives one session's L2 view.
- `insight` writes one cross-session finding into one target session.
- `shared-memory` stores durable space-level context.
- `fork` creates a child session.

Include the default workflow:

1. Check authentication with `kitkit-cli auth status`.
2. Select a space with `kitkit-cli spaces list`.
3. Inspect display metadata with `kitkit-cli spaces get <SPACE_ID>` when useful.
4. Load topology with `kitkit-cli topology <SPACE_ID>`.
5. Use `--json` when output will be parsed or compared.

### `kitkit-conversation-tree`

Teach agents to use the session tree as a document library:

- Start from `topology`, then read selected nodes with `digest`.
- Use node labels, statuses, turn counts, and tree position to decide what to inspect.
- Prefer targeted reads over trying to exhaustively load every session.
- Treat digest memory as a compact L2 summary, not as guaranteed full transcript.
- Re-read topology when the tree may have changed.

This skill should tell agents to use `$kitkit-cli` for authentication, space selection, and base command rules.

### `kitkit-insight`

Teach insight push behavior:

- Push insights for cross-branch findings, contradictions, reusable context, or warnings that matter to another active session.
- Read target digest before overwriting the target insight slot.
- Keep content concise and actionable.
- Choose the recipient session deliberately from topology.
- Do not use insight as normal chat, durable memory, or a scratchpad.
- Remember repeated writes overwrite the previous insight.

This skill should tell agents to use `$kitkit-conversation-tree` when they need to find the right target session.

### `kitkit-fork`

Teach fork behavior:

- Fork only after identifying the source session in topology and reading relevant digest context.
- Explain `--context none`, `--context inherit`, and `--context compress`.
- Explain when to use `--profile`, repeated `--skill`, prompt overrides, first `--prompt`, and `--topology-parent-id`.
- Recommend conservative defaults: use `compress` for summarized parent handoff, `inherit` when exact parent context matters, and `none` for deliberately clean branches.
- Re-read topology after fork creation if subsequent steps depend on the new tree.

This skill should tell agents to use `$kitkit-cli` for command basics and `$kitkit-conversation-tree` for source-node selection.

### `kitkit-as-kitty`

Provide the high-level Kitty workflow that composes the other skills:

1. Establish space context with `$kitkit-cli`.
2. Read shared memory early.
3. Treat shared memory as durable but possibly stale; check it against current topology and digests before relying on it.
4. Build a global map using `$kitkit-conversation-tree`.
5. Push insight with `$kitkit-insight` when another branch should know something now.
6. Fork with `$kitkit-fork` when exploration should become a new branch.
7. Update shared memory only for durable facts, user preferences, project background, or long-lived goals.

The skill should emphasize acting like Kitty: maintain global awareness, surface useful cross-session context, avoid stale assumptions, and make changes through the CLI rather than inventing local state.

## Validation

After implementation:

- Run `quick_validate.py` for each skill folder if available.
- Run `kitkit-cli --help` and relevant subcommand help to verify command names in the skills still match the CLI.
- Check every skill body is concise and does not duplicate large CLI help sections.
- Confirm no `agents/openai.yaml` files are present.

