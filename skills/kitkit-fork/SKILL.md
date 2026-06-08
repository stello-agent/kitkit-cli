---
name: kitkit-fork
description: "Use when an agent needs to create or plan KitKit fork sessions with kitkit-cli: choose source sessions, select none/inherit/compress context, apply fork profiles, attach skills, pass prompts, or place topology nodes."
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
