# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Preserving Todos

When working in this repo, use the `todoer` CLI itself to track tasks and preserve context across sessions. This applies the tool to its own development — todos created here are project-scoped to `todoer`.

Use the `todoer-cli` skill for full usage details. Quick reference:

```bash
# Create a task
todoer new "description of work"

# List open tasks
todoer list

# Mark done
todoer task update status <uuid> COMPLETED

# Add a note (e.g., to record why something was deferred)
todoer task note <uuid> "note text"
```

Before starting implementation work, check `todoer list` for existing tasks. After completing a task, mark it `COMPLETED`. If work is interrupted, add a note explaining state so the next session can resume cleanly.

## Commands

```bash
# Build
cargo build

# Run tests
cargo test

# Run a single test file
cargo test --test commands_new

# Run a single test by name
cargo test --test commands_task test_add_note

# Lint (CI enforces zero warnings)
cargo clippy -- -D warnings

# Format check
cargo fmt --check

# Run with args (dev)
cargo run -- <command> [args]
```

## Release

Uses `just` for release automation:
```bash
just release-start 1.2.3   # creates release/v1.2.3 branch, bumps Cargo.toml
just release-tag 1.2.3     # tags and pushes after branch is merged to main
```

The release pipeline publishes to crates.io (crate name: `todoer-robots-cli`).

## Architecture

### Data Flow

Project resolution -> DB connection -> command execution -> output formatting

1. **Project resolution** (`src/project.rs`): Finds the active project by checking `--project` flag, then searching upward for `.todoer.toml`, then falling back to git repo name. Returns a `ResolvedProject` with `name` (display) and `key` (lowercased, used as DB primary key).

2. **DB connection** (`src/config.rs`, `src/db.rs`): Resolves DB path from `~/.config/todoer/config.toml` or XDG default (`~/.local/share/todoer/todoer.db`). Schema is created on first connection via `db::initialize`.

3. **Commands** (`src/commands/`): Each command receives a `Connection` and operates via `repo.rs` functions. Commands do not format output themselves -- they return data to `main.rs`.

4. **Output** (`src/output.rs`): Formats responses as plain text/tables or JSON. All commands accept `--json`. JSON schema is at `schema/todoer-output.schema.json`.

### Key modules

- `src/repo.rs` -- all SQLite CRUD (tasks, notes, projects)
- `src/models.rs` -- `Task`, `TaskNote`, `Status` types
- `src/cli.rs` -- clap command/subcommand definitions
- `src/input.rs` -- resolves description/note from arg or stdin (`-`)
- `src/commands/task.rs` -- handles `task show|status|note|update` subcommands

### Database

SQLite with foreign keys enabled. Three tables: `projects` (name_key PK), `tasks` (UUID PK, project_key FK), `task_notes` (autoincrement PK, task_id FK). Tasks are always created under an existing project; `repo::ensure_project` uses `INSERT OR IGNORE` to auto-register projects.

### Tests

Integration tests in `tests/` use `tempfile::TempDir` for isolated SQLite databases. Each test creates its own DB -- no shared state. Test files map 1:1 to command modules (`commands_new.rs`, `commands_list.rs`, etc.).
