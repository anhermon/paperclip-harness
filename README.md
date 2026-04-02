# paperclip-harness

A Rust-native, provider-agnostic agent harness.

> **Status:** v0 — single-turn agent loop with Claude, SQLite memory, tool registry.

## Quick start

```bash
export ANTHROPIC_API_KEY=sk-...
cargo run -- run --goal "What is 2+2?"

# Or use the echo provider (no API key needed)
cargo run -- run --provider echo --goal "hello"

# Check config
cargo run -- config --check
```

## Workspace layout

```
crates/
├── core/     # Provider trait, message types, config, session
├── tools/    # Tool registry, schema validation, built-in tools
├── memory/   # SQLite episodic memory (sqlx + FTS5)
└── cli/      # clap CLI: harness run / config / memory
```

## Providers

| Backend  | Env var             | Notes                          |
|----------|---------------------|-------------------------------|
| `claude` | `ANTHROPIC_API_KEY` | Default. claude-sonnet-4-5     |
| `echo`   | —                   | Echoes input, useful for tests |

## Memory

Episodes are stored in `~/.paperclip/harness/memory.db` (SQLite).
Full-text search via FTS5: `harness memory search "my query"`.

## License

MIT OR Apache-2.0
