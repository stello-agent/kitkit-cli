# kitkit-cli

*Currently WIP*

Command-line access to KitKit spaces, conversation topology, session digests,
insight push, shared memory, and forks.

The CLI is intended for agents such as Codex and Claude Code that need to act as
a KitKit "main" process: inspect a whole conversation tree, read shared context,
push insights to sessions, and create new fork nodes.

## TODOs

- [ ] documentation refined
- [ ] provide skills

## Build

```bash
cargo build
```

## Authentication

Login is interactive by default:

```bash
kitkit-cli auth login
```

Or pass the email and read the password from stdin:

```bash
printf '%s' "$KITKIT_PASSWORD" \
  | kitkit-cli auth login --email "$KITKIT_EMAIL" --password-stdin
```

The CLI stores access and refresh tokens in the platform config directory via
the `directories` crate:

- Linux: `$XDG_CONFIG_HOME/kitkit-cli/auth.json`, or `~/.config/kitkit-cli/auth.json`
- macOS: `~/Library/Application Support/com.kitkit-agent.kitkit-cli/auth.json`
- Windows: `%APPDATA%\kitkit-agent\kitkit-cli\config\auth.json`

On Unix, the token file is created with `0600` permissions.

Check login state:

```bash
kitkit-cli auth status
```

Remove cached tokens:

```bash
kitkit-cli auth logout
```

## Output

Human-readable output is the default:

```bash
kitkit-cli spaces list
```

Every command supports JSON output:

```bash
kitkit-cli --json spaces list
kitkit-cli --json digest <space-id> <session-id>
```

JSON output is produced from the strongly typed SDK response structs.

## Commands

See `--help`

## License

MIT OR Apache-2.0
