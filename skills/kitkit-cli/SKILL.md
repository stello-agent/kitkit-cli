---
name: kitkit-cli
description: "Use when an agent needs to work with KitKit through kitkit-cli: authenticate, choose spaces, understand the KitKit space/session/topology/shared-memory/insight/fork mental model, parse JSON output, or prepare for Kitty-like global conversation-tree work."
---

# KitKit CLI

## Mental Model

Use `kitkit-cli` as the command-line access layer for KitKit space-level work.

- A space owns one conversation tree and one shared-memory store.
- A session is a node in the conversation tree.
- `topology` shows the tree, including session ids, labels, status, turn counts, and children.
- `digest` reads one session's L2 view: metadata, memory summary, and current insight.
- `insight` writes one cross-session finding into one target session's insight slot.
- `shared-memory` stores durable space-level context.
- `fork` creates a child session from a source session.

Do not treat the CLI as ordinary chat. It is for global awareness, cross-node coordination, durable memory, and forks.

## Basic Workflow

Start with authentication and space selection:

```bash
kitkit-cli auth status
kitkit-cli spaces list
kitkit-cli spaces get <SPACE_ID>
kitkit-cli topology <SPACE_ID>
```

Use `spaces get` when the label alone is not enough and human-facing space metadata matters.

Use `--json` whenever output will be parsed, compared, summarized mechanically, or passed between tools:

```bash
kitkit-cli --json spaces list
kitkit-cli --json topology <SPACE_ID>
kitkit-cli --json digest <SPACE_ID> <SESSION_ID>
```

Use human-readable output for quick inspection.

## Shared Memory

Read shared memory early when acting with Kitty-like global context:

```bash
kitkit-cli shared-memory list <SPACE_ID>
```

Write shared memory only for durable facts such as user preferences, project background, long-lived goals, or stable constraints:

```bash
kitkit-cli shared-memory upsert <SPACE_ID> <SLUG> --body-file memory.md
cat memory.md | kitkit-cli shared-memory upsert <SPACE_ID> <SLUG> --stdin
```

Delete only when the entry is wrong or obsolete:

```bash
kitkit-cli shared-memory delete <SPACE_ID> <SLUG>
```

Shared memory is global to the space. Do not use it for temporary notes, one-session findings, or anything that belongs in an insight.

## Command Help

Treat live CLI help as the command authority:

```bash
kitkit-cli --help
kitkit-cli shared-memory --help
kitkit-cli insight put --help
kitkit-cli fork --help
```

Prefer fixing `kitkit-cli` help or behavior in the CLI repo if the tool surface is confusing. Do not paper over framework or CLI problems in a skill.
