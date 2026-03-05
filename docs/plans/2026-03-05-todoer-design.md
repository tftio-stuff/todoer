# todoer design

Date: 2026-03-05

## Summary

todoer is a Rust CLI that manages per-project todo tasks stored in a single SQLite database at the user’s XDG data location. Projects are resolved via a local `.todoer.toml` (searched upward from the current directory to the user’s home directory), or via `--project` overrides. Tasks have UUID primary keys, creation dates, descriptions, statuses, and optional notes.

## Goals

- Simple, durable CLI for adding and managing todos by project.
- SQLite storage in XDG data location, with config override for DB path.
- Project discovery via `.todoer.toml` in the filesystem hierarchy.
- Machine-readable JSON output and a published JSON schema.

## Non-Goals (for v1)

- Remote sync or collaboration.
- Advanced filtering (tags, priorities, date ranges).
- Multiple databases or per-project DB files.

## Storage and Schema

### XDG locations

- Config: `$XDG_CONFIG_HOME/todoer/config.toml` if set, else `~/.config/todoer/config.toml`.
- Data: `$XDG_DATA_HOME/todoer/todoer.db` if set, else `~/.local/share/todoer/todoer.db`.

### Config file (v1)

`config.toml` contains only:

```
# $XDG_CONFIG_HOME/todoer/config.toml
# or ~/.config/todoer/config.toml

db_path = "/absolute/path/to/todoer.db"
```

If `db_path` is absent, use the default XDG data location.

### Project discovery

Search for `.todoer.toml` starting at the current working directory and walking upward to the user’s home directory (inclusive). The first file found is used. The file only contains:

```
project = "My Project"
```

If `.todoer.toml` is missing, `init` uses `--project` or, if inside a git repo, the repo name. Other commands require `--project` unless `.todoer.toml` is present or `--all` is used (for `list`).

### Schema

Project primary key is a normalized key derived from project name (lowercased, trimmed; normalization rule to be finalized and kept stable). Example tables:

- `projects`
  - `name_key TEXT PRIMARY KEY`
  - `name TEXT NOT NULL`
- `tasks`
  - `id TEXT PRIMARY KEY` (UUID)
  - `project_key TEXT NOT NULL` (FK to `projects`)
  - `created_at TEXT NOT NULL` (ISO 8601 UTC)
  - `description TEXT NOT NULL`
  - `status TEXT NOT NULL` (enum values)
- `task_notes`
  - `id INTEGER PRIMARY KEY AUTOINCREMENT`
  - `task_id TEXT NOT NULL` (FK to `tasks`)
  - `created_at TEXT NOT NULL` (ISO 8601 UTC)
  - `note TEXT NOT NULL`

Status enum values: `NEW`, `IN-PROGRESS`, `COMPLETED`, `ABANDONED`.

## CLI

### Common flags

- `--project <name>`: override project name for `new` and `list`.
- `--all`: `list` only, ignore project scoping.
- `--json`: machine-readable output for all commands.

### Commands

#### `init`

Idempotently ensure DB exists, schema is created, config location is valid, and `.todoer.toml` (if present) is valid. If `.todoer.toml` is missing, choose project name from `--project` or git repo name; if neither, error. Ensures project row exists. JSON output returns resolved project, db path, and schema creation status.

#### `new`

Creates a task with UUID v4, `created_at = now (UTC)`, `status=NEW`. Description is required; `-` means read from stdin. Project resolves as `--project` > `.todoer.toml` > error. JSON output returns created task.

#### `list`

Lists tasks for a project (or all projects with `--all`). Ordering: `created_at ASC`. Table output includes UUID, status, created_at, description. JSON output returns an array of tasks.

#### `task`

- `task status <uuid>`: show description, status, created_at.
- `task update status <uuid> <STATUS>`: update status and return updated state.
- `task note <uuid> <note|->`: add note (stdin if `-`).
- `task show <uuid>`: show description, status, created_at, then notes in reverse chronological order.

## JSON Output and Schema

All commands honor `--json` and return a top-level object:

```
{
  "ok": true,
  "command": "list",
  "data": { ... }
}
```

On error:

```
{
  "ok": false,
  "command": "new",
  "error": {
    "code": "PROJECT_NOT_FOUND",
    "message": "No project specified and no .todoer.toml found.",
    "details": { ... }
  }
}
```

A JSON Schema will be published for the top-level response and for each command’s data payloads.

## Error Handling

- Non-JSON mode: concise, actionable errors.
- JSON mode: structured error object.
- Validate `.todoer.toml` strictly: `project` required and non-empty; unknown keys ignored for forward compatibility.

## Testing

- Unit tests: project normalization, XDG config resolution, `.todoer.toml` discovery.
- Integration tests: schema creation, `new` insertion, `list` ordering, `task` flows.
- Optional: JSON schema validation tests using sample outputs.
