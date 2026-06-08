---
name: kitkit-insight
description: "Use when an agent needs to push, update, or reason about KitKit insights: cross-session findings, contradictions, warnings, reusable context, target-session selection, overwrite safety, and concise insight content."
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
