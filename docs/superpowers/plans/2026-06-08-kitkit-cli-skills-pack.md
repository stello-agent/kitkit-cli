# KitKit CLI Skills Pack Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Create a composable five-skill pack that teaches agents to use `kitkit-cli` with Kitty-like global awareness.

**Architecture:** Keep each skill as a standalone `skills/<name>/SKILL.md` file with only required frontmatter. Use `kitkit-cli` as the base mental-model skill, then layer conversation-tree, insight, fork, and high-level Kitty workflow skills on top. Do not create `agents/openai.yaml`, scripts, references, or assets.

**Tech Stack:** Markdown skill files, `kitkit-cli`, `python3` for skill validation, Git.

---

## File Structure

- Create `skills/kitkit-cli/SKILL.md`: foundational KitKit mental model and base CLI workflow.
- Create `skills/kitkit-conversation-tree/SKILL.md`: read topology and targeted digests as a document library.
- Create `skills/kitkit-insight/SKILL.md`: push concise cross-session insights safely.
- Create `skills/kitkit-fork/SKILL.md`: create fork sessions with the correct context mode and overrides.
- Create `skills/kitkit-as-kitty/SKILL.md`: high-level Kitty behavior that composes the other skills.
- Modify `README.md`: replace the stale skills TODO with a concise skills section.

## Commands

Run commands from `/home/i/Code/kitkit-cli`.

Use these validation helpers:

```bash
python3 /home/i/.codex/skills/.system/skill-creator/scripts/quick_validate.py skills/kitkit-cli
python3 /home/i/.codex/skills/.system/skill-creator/scripts/quick_validate.py skills/kitkit-conversation-tree
python3 /home/i/.codex/skills/.system/skill-creator/scripts/quick_validate.py skills/kitkit-insight
python3 /home/i/.codex/skills/.system/skill-creator/scripts/quick_validate.py skills/kitkit-fork
python3 /home/i/.codex/skills/.system/skill-creator/scripts/quick_validate.py skills/kitkit-as-kitty
cargo run -- --help
cargo run -- shared-memory --help
cargo run -- fork --help
find skills -path '*/agents/openai.yaml' -print
```

Expected validation:

- Every `quick_validate.py` command prints `Skill is valid!`.
- CLI help commands exit 0 and include the command names used by the skills.
- `find skills -path '*/agents/openai.yaml' -print` prints nothing.

Use this co-author trailer on implementation commits:

```text
Co-authored-by: Codex <codex@openai.com>
```

---

### Task 1: Scaffold Skill Directories

**Files:**
- Create: `skills/kitkit-cli/SKILL.md`
- Create: `skills/kitkit-conversation-tree/SKILL.md`
- Create: `skills/kitkit-insight/SKILL.md`
- Create: `skills/kitkit-fork/SKILL.md`
- Create: `skills/kitkit-as-kitty/SKILL.md`

- [ ] **Step 1: Confirm the skills directory is empty or only contains expected local work**

Run:

```bash
find skills -maxdepth 3 -type f -print
```

Expected: no existing skill files that would be overwritten.

- [ ] **Step 2: Initialize the five skills with the skill-creator template**

Run:

```bash
python3 /home/i/.codex/skills/.system/skill-creator/scripts/init_skill.py kitkit-cli --path skills
python3 /home/i/.codex/skills/.system/skill-creator/scripts/init_skill.py kitkit-conversation-tree --path skills
python3 /home/i/.codex/skills/.system/skill-creator/scripts/init_skill.py kitkit-insight --path skills
python3 /home/i/.codex/skills/.system/skill-creator/scripts/init_skill.py kitkit-fork --path skills
python3 /home/i/.codex/skills/.system/skill-creator/scripts/init_skill.py kitkit-as-kitty --path skills
```

Expected: each command creates a skill directory and a `SKILL.md`.

- [ ] **Step 3: Remove generated OpenAI UI metadata**

Run:

```bash
rm -rf skills/kitkit-cli/agents \
  skills/kitkit-conversation-tree/agents \
  skills/kitkit-insight/agents \
  skills/kitkit-fork/agents \
  skills/kitkit-as-kitty/agents
```

Expected: no `agents/openai.yaml` files remain.

- [ ] **Step 4: Verify no generated metadata remains**

Run:

```bash
find skills -path '*/agents/openai.yaml' -print
```

Expected: no output.

- [ ] **Step 5: Leave the scaffold uncommitted**

Run:

```bash
git status --short
```

Expected: the five scaffolded `SKILL.md` files are untracked or modified. Do not commit generated template skill content; replace each file in the following tasks before committing.

---

### Task 2: Write the Base `kitkit-cli` Skill

**Files:**
- Modify: `skills/kitkit-cli/SKILL.md`

- [ ] **Step 1: Replace `skills/kitkit-cli/SKILL.md` with this content**

```markdown
---
name: kitkit-cli
description: Use when an agent needs to work with KitKit through kitkit-cli: authenticate, choose spaces, understand the KitKit space/session/topology/shared-memory/insight/fork mental model, parse JSON output, or prepare for Kitty-like global conversation-tree work.
---

# KitKit CLI

## Mental Model

Use `kitkit-cli` as the command-line access layer for KitKit space-level work.

- A space owns one conversation tree and one shared-memory store.
- A session is a node in the conversation tree.
- `topology` shows the tree, including session ids, labels, status, turn counts, and children.
- `digest` reads one session's L2 view: metadata, memory summary, and current insight.
- `insight` writes one cross-session finding into one target session's insight slot.
- `shared-memory` stores durable space-level context visible to Kitty-like agents.
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
```

- [ ] **Step 2: Validate the skill**

Run:

```bash
python3 /home/i/.codex/skills/.system/skill-creator/scripts/quick_validate.py skills/kitkit-cli
```

Expected: `Skill is valid!`

- [ ] **Step 3: Commit the base skill**

Run:

```bash
git add skills/kitkit-cli/SKILL.md
git commit -m "docs: add kitkit cli skill" -m "Co-authored-by: Codex <codex@openai.com>"
```

Expected: commit succeeds.

---

### Task 3: Write the Conversation Tree Skill

**Files:**
- Modify: `skills/kitkit-conversation-tree/SKILL.md`

- [ ] **Step 1: Replace `skills/kitkit-conversation-tree/SKILL.md` with this content**

```markdown
---
name: kitkit-conversation-tree
description: Use when an agent needs to treat a KitKit or Stello conversation tree as a document library: inspect topology, choose relevant session nodes, read targeted digests, compare branches, or build space-level context without loading full transcripts.
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
```

- [ ] **Step 2: Validate the skill**

Run:

```bash
python3 /home/i/.codex/skills/.system/skill-creator/scripts/quick_validate.py skills/kitkit-conversation-tree
```

Expected: `Skill is valid!`

- [ ] **Step 3: Commit the conversation tree skill**

Run:

```bash
git add skills/kitkit-conversation-tree/SKILL.md
git commit -m "docs: add conversation tree skill" -m "Co-authored-by: Codex <codex@openai.com>"
```

Expected: commit succeeds.

---

### Task 4: Write the Insight Skill

**Files:**
- Modify: `skills/kitkit-insight/SKILL.md`

- [ ] **Step 1: Replace `skills/kitkit-insight/SKILL.md` with this content**

```markdown
---
name: kitkit-insight
description: Use when an agent needs to push, update, or reason about KitKit insights: cross-session findings, contradictions, warnings, reusable context, target-session selection, overwrite safety, and concise insight content.
---

# KitKit Insight

## Purpose

Use insight to push one concise cross-session finding into one target session.

Good insight candidates:

- A contradiction between branches.
- Context from one branch that changes another branch's next action.
- A warning about stale assumptions or invalidated work.
- A reusable conclusion that should be visible to a currently active session.
- A recommendation that helps the target session continue.

Do not use insight as normal chat, durable memory, a scratchpad, or a full summary. Use shared memory for stable space-level facts and `digest` for reading session state.

## Safety Workflow

Use `$kitkit-conversation-tree` if the target session is not already known.

Before writing:

1. Load or refresh topology.
2. Choose the target active session deliberately.
3. Read the target digest.
4. Check the current insight because writing replaces it.
5. Write only if the new insight is more useful than the current slot.

Commands:

```bash
kitkit-cli digest <SPACE_ID> <SESSION_ID>
kitkit-cli insight put <SPACE_ID> <SESSION_ID> --content-file insight.md
cat insight.md | kitkit-cli insight put <SPACE_ID> <SESSION_ID> --stdin
```

Use `--json` when confirming machine-readable results:

```bash
kitkit-cli --json insight put <SPACE_ID> <SESSION_ID> --content-file insight.md
```

## Writing Style

Write insight as a short handoff to the target session.

Include:

- The finding.
- Why it matters now.
- The source branch or digest evidence when useful.
- The suggested next action if there is one.

Avoid:

- Long transcripts.
- Multiple unrelated findings.
- Vague status updates.
- Private reasoning that the target session cannot act on.

Keep under the server's 4000-character limit. Prefer much shorter text unless the finding genuinely needs detail.

## Recipient Choice

Push insight to the session that can act on it.

- If one branch contains a contradiction that affects another branch, target the affected active branch.
- If a finding affects all future work, consider whether shared memory is more appropriate.
- If the target is archived, do not push insight there; use it as historical context instead.
- If several active sessions need the same finding, write each insight intentionally and keep each one tailored.
```

- [ ] **Step 2: Validate the skill**

Run:

```bash
python3 /home/i/.codex/skills/.system/skill-creator/scripts/quick_validate.py skills/kitkit-insight
```

Expected: `Skill is valid!`

- [ ] **Step 3: Commit the insight skill**

Run:

```bash
git add skills/kitkit-insight/SKILL.md
git commit -m "docs: add kitkit insight skill" -m "Co-authored-by: Codex <codex@openai.com>"
```

Expected: commit succeeds.

---

### Task 5: Write the Fork Skill

**Files:**
- Modify: `skills/kitkit-fork/SKILL.md`

- [ ] **Step 1: Replace `skills/kitkit-fork/SKILL.md` with this content**

```markdown
---
name: kitkit-fork
description: Use when an agent needs to create or plan KitKit fork sessions with kitkit-cli: choose source sessions, select none/inherit/compress context, apply fork profiles, attach skills, pass prompts, or place topology nodes.
---

# KitKit Fork

## Purpose

Use `fork` to create a child session from an existing source session. A fork is a new topology node for separate exploration, specialization, or handoff.

Use `$kitkit-cli` for command basics and `$kitkit-conversation-tree` to select the source node.

## Pre-Fork Checklist

Before creating a fork:

1. Load topology.
2. Identify the source session id.
3. Read the source digest.
4. Decide whether the child should inherit, compress, or omit parent context.
5. Decide whether the visible topology parent should differ from the source session.
6. Decide whether the fork needs a profile, prompt overrides, enabled skills, or an initial prompt.

Commands:

```bash
kitkit-cli topology <SPACE_ID>
kitkit-cli digest <SPACE_ID> <SOURCE_SESSION_ID>
```

## Context Modes

Choose the context mode deliberately:

- `--context compress`: use when the child should receive a summarized parent handoff. This is the conservative default for most branch work.
- `--context inherit`: use when exact parent context matters and extra context volume is acceptable.
- `--context none`: use when the child should start cleanly without parent conversation context.

Examples:

```bash
kitkit-cli fork <SPACE_ID> <SOURCE_SESSION_ID> --label "Research branch" --context compress
kitkit-cli fork <SPACE_ID> <SOURCE_SESSION_ID> --label "Clean repro" --context none
kitkit-cli fork <SPACE_ID> <SOURCE_SESSION_ID> --label "Continue exact context" --context inherit
```

## Profiles, Skills, And Prompts

Use a fork profile when the space capability configuration already defines a reusable role:

```bash
kitkit-cli fork <SPACE_ID> <SOURCE_SESSION_ID> --label "Product review" --profile product
kitkit-cli fork <SPACE_ID> <SOURCE_SESSION_ID> --label "Audience branch" --profile product --profile-var audience=developer
```

Enable named space skills when the new session needs specific capabilities:

```bash
kitkit-cli fork <SPACE_ID> <SOURCE_SESSION_ID> --label "Docs pass" --skill writing --skill review
```

Use prompt overrides only when the fork needs a different system or compression behavior:

```bash
kitkit-cli fork <SPACE_ID> <SOURCE_SESSION_ID> --label "Custom role" --system-prompt "Act as a concise reviewer."
```

Use `--prompt` for the first user prompt sent into the new fork:

```bash
kitkit-cli fork <SPACE_ID> <SOURCE_SESSION_ID> --label "Investigate bug" --context compress --prompt "Find the smallest likely cause."
```

## Topology Placement

By default, the new node is placed under the source session. Use `--topology-parent-id` only when the visible tree should place the child elsewhere:

```bash
kitkit-cli fork <SPACE_ID> <SOURCE_SESSION_ID> --label "Grouped follow-up" --topology-parent-id <PARENT_SESSION_ID>
```

After creating a fork, re-read topology when later steps depend on the new node id or tree position:

```bash
kitkit-cli topology <SPACE_ID>
```
```

- [ ] **Step 2: Validate the skill**

Run:

```bash
python3 /home/i/.codex/skills/.system/skill-creator/scripts/quick_validate.py skills/kitkit-fork
```

Expected: `Skill is valid!`

- [ ] **Step 3: Commit the fork skill**

Run:

```bash
git add skills/kitkit-fork/SKILL.md
git commit -m "docs: add kitkit fork skill" -m "Co-authored-by: Codex <codex@openai.com>"
```

Expected: commit succeeds.

---

### Task 6: Write the High-Level Kitty Skill

**Files:**
- Modify: `skills/kitkit-as-kitty/SKILL.md`

- [ ] **Step 1: Replace `skills/kitkit-as-kitty/SKILL.md` with this content**

```markdown
---
name: kitkit-as-kitty
description: Use when an agent should behave like KitKit Kitty: maintain global space awareness, read shared memory early, treat it as possibly stale, inspect conversation-tree digests, push useful insights, update durable memory, or create forks through kitkit-cli.
---

# KitKit As Kitty

## Role

Act as Kitty: a space-level agent with global awareness across the KitKit conversation tree.

Use the lower-level skills as needed:

- `$kitkit-cli` for mental model, auth, shared memory, and base command usage.
- `$kitkit-conversation-tree` for topology navigation and digest reads.
- `$kitkit-insight` for cross-session insight pushes.
- `$kitkit-fork` for branch creation.

Do not invent local state when the CLI can read or update KitKit state.

## Startup Routine

When a user asks you to act with Kitty-like awareness:

1. Check auth if needed.
2. Select or confirm the space.
3. Read shared memory early.
4. Load topology.
5. Read the most relevant session digests.
6. State what you inspected before drawing global conclusions.

Commands:

```bash
kitkit-cli auth status
kitkit-cli spaces list
kitkit-cli shared-memory list <SPACE_ID>
kitkit-cli topology <SPACE_ID>
kitkit-cli digest <SPACE_ID> <SESSION_ID>
```

## Shared Memory Discipline

Treat shared memory as durable but not automatically current.

Before relying on a memory entry:

- Check whether topology or recent digests contradict it.
- Prefer the newest relevant session evidence when memory appears stale.
- Tell the user when a memory entry seems outdated.

Update shared memory only for:

- Durable user preferences.
- Project background.
- Long-lived goals.
- Stable constraints.
- Corrections to obsolete memory.

Do not store:

- Temporary observations.
- One branch's local task state.
- Raw transcripts.
- Cross-session findings that belong in insight.

Use:

```bash
kitkit-cli shared-memory upsert <SPACE_ID> <SLUG> --body-file memory.md
kitkit-cli shared-memory delete <SPACE_ID> <SLUG>
```

## Insight Push Triggers

Push insight when another active session should know something now:

- A branch discovered a contradiction.
- A branch invalidated another branch's assumption.
- A decision or warning changes another branch's next action.
- A reusable finding belongs in a specific active session but is not durable enough for shared memory.

Before pushing, use `$kitkit-insight`: read the target digest and avoid overwriting a more valuable existing insight.

## Global Awareness Loop

For ongoing work:

1. Keep a compact map of important sessions and which digests you read.
2. Re-read topology after forks or when the tree may have changed.
3. Re-read shared memory when starting a new major task or when memory-sensitive facts matter.
4. Push insight when cross-branch context should move immediately.
5. Fork when work needs a separate branch rather than a note or insight.
6. Make evidence boundaries clear: distinguish inspected digests from inferred global state.

## Forking As Kitty

Create a fork when exploration should become a new session:

- The task needs a clean branch.
- A specialized role or profile should take over.
- A branch should test an alternative without disturbing the source.
- A compressed handoff is more useful than another insight.

Use `$kitkit-fork` before running `kitkit-cli fork`.
```

- [ ] **Step 2: Validate the skill**

Run:

```bash
python3 /home/i/.codex/skills/.system/skill-creator/scripts/quick_validate.py skills/kitkit-as-kitty
```

Expected: `Skill is valid!`

- [ ] **Step 3: Commit the Kitty workflow skill**

Run:

```bash
git add skills/kitkit-as-kitty/SKILL.md
git commit -m "docs: add kitty workflow skill" -m "Co-authored-by: Codex <codex@openai.com>"
```

Expected: commit succeeds.

---

### Task 7: Update README Skill Index

**Files:**
- Modify: `README.md`

- [ ] **Step 1: Replace the TODO block with a concise skills section**

Replace:

```markdown
## TODOs

- [ ] documentation refined
- [ ] provide skills
```

With:

```markdown
## Skills

Reusable agent skills live in `skills/`:

- `kitkit-cli`: KitKit mental model, auth, shared memory, and base CLI workflow.
- `kitkit-conversation-tree`: topology and targeted digest reading.
- `kitkit-insight`: cross-session insight push workflow.
- `kitkit-fork`: fork creation and context-mode selection.
- `kitkit-as-kitty`: high-level Kitty role that composes the lower-level skills.

## TODOs

- [ ] documentation refined
```

- [ ] **Step 2: Commit the README update**

Run:

```bash
git add README.md
git commit -m "docs: list kitkit skills" -m "Co-authored-by: Codex <codex@openai.com>"
```

Expected: commit succeeds.

---

### Task 8: Final Validation

**Files:**
- Validate: `skills/kitkit-cli/SKILL.md`
- Validate: `skills/kitkit-conversation-tree/SKILL.md`
- Validate: `skills/kitkit-insight/SKILL.md`
- Validate: `skills/kitkit-fork/SKILL.md`
- Validate: `skills/kitkit-as-kitty/SKILL.md`
- Validate: `README.md`

- [ ] **Step 1: Run all skill validators**

Run:

```bash
python3 /home/i/.codex/skills/.system/skill-creator/scripts/quick_validate.py skills/kitkit-cli
python3 /home/i/.codex/skills/.system/skill-creator/scripts/quick_validate.py skills/kitkit-conversation-tree
python3 /home/i/.codex/skills/.system/skill-creator/scripts/quick_validate.py skills/kitkit-insight
python3 /home/i/.codex/skills/.system/skill-creator/scripts/quick_validate.py skills/kitkit-fork
python3 /home/i/.codex/skills/.system/skill-creator/scripts/quick_validate.py skills/kitkit-as-kitty
```

Expected: every command prints `Skill is valid!`.

- [ ] **Step 2: Verify command references against CLI help**

Run:

```bash
cargo run -- --help
cargo run -- shared-memory --help
cargo run -- insight put --help
cargo run -- fork --help
```

Expected: commands exit 0 and show the options referenced by the skill files.

- [ ] **Step 3: Confirm no OpenAI metadata files exist**

Run:

```bash
find skills -path '*/agents/openai.yaml' -print
```

Expected: no output.

- [ ] **Step 4: Inspect final diff**

Run:

```bash
git status --short
git log --oneline -8
```

Expected: no uncommitted changes after final commits; recent log includes the design, plan, skill, README, and validation-related commits.
