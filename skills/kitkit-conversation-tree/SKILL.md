---
name: kitkit-conversation-tree
description: "Use when an agent needs to treat a KitKit or Stello conversation tree as a document library: inspect topology, choose relevant session nodes, read targeted digests, compare branches, or build space-level context without loading full transcripts."
---

# KitKit Conversation Tree

## Start From Topology

Use `$kitkit-cli` first if authentication, space selection, or command basics are not already established.

Load the tree before choosing nodes:

```bash
kitkit-cli topology <SPACE_ID>
kitkit-cli --json topology <SPACE_ID>
```

Use topology fields as navigation signals:

- `id`: target for `digest`, `insight`, or `fork`.
- `label`: human-facing clue about the branch purpose.
- `status`: active sessions can receive insight; archived sessions are historical context.
- `turn_count`: rough activity and depth signal.
- `children`: branch structure and local neighborhood.

Re-read topology when another actor may have created forks, archived sessions, or changed labels.

## Read Digests Like Documents

Read selected nodes with `digest`:

```bash
kitkit-cli digest <SPACE_ID> <SESSION_ID>
kitkit-cli --json digest <SPACE_ID> <SESSION_ID>
```

Treat the digest as a compact L2 document for one session. It may include memory and the current insight, but it is not guaranteed to contain every L3 conversation record.

Prefer targeted reads:

1. Read the root or nearest common ancestor when you need project background.
2. Read labels and sibling branches before comparing decisions.
3. Read the target node before pushing insight or forking from it.
4. Read recently active or high-turn nodes when looking for current state.

Avoid exhaustively reading every node unless the tree is small or the task explicitly requires a full audit.

## Build A Global Map

When summarizing a space, keep a compact map in your own working context:

- Space id and label.
- Root session id.
- Important branches and their ids.
- Which digests you actually read.
- Open contradictions, stale assumptions, or reusable findings.
- Sessions that should receive insight.

Be explicit about evidence. Say which session digests support a conclusion and which areas were not inspected.

## Branch Comparison

For branch comparison:

1. Load topology.
2. Identify the branch nodes and their nearest shared ancestor.
3. Read the ancestor digest.
4. Read each branch digest.
5. Compare goals, assumptions, decisions, blockers, and stale facts.
6. Use `$kitkit-insight` when a finding should be pushed into an active session.
