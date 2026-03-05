# todoer

A small Rust CLI that stores project-scoped todos in SQLite.

## Quick start

```
# initialize DB (uses XDG data location by default)
cargo run -- init --project "My Project"

# create a task
cargo run -- new "write docs"

# list tasks
cargo run -- list

# update status
cargo run -- task update status <uuid> COMPLETED
```

## Release how-to

Prereqs:
- GitHub Actions secrets: `CARGO_REGISTRY_TOKEN` set to your crates.io token.
- `just` installed.

Release flow:
1. Start a release branch and bump version:
```
just release-start 1.2.3
```
2. Open a PR from `release/v1.2.3` to `main` and merge after CI passes.
3. Tag and push the release (triggers publish):
```
just release-tag 1.2.3
```

## Install (dev)

```
cargo build
```

## Configuration

Config file (optional):

- `$XDG_CONFIG_HOME/todoer/config.toml`
- `~/.config/todoer/config.toml`

```
db_path = "/absolute/path/to/todoer.db"
```

## Project file

Create a `.todoer.toml` in your project (or a parent directory):

```
project = "My Project"
```

The tool searches upward from the current directory to your home directory and uses the first `.todoer.toml` it finds.

## Usage

```
# initialize DB and project
cargo run -- init --project "My Project"

# create a task
cargo run -- new "write docs"

# read description from stdin
printf "from stdin" | cargo run -- new -

# list tasks
cargo run -- list

# list all tasks across projects
cargo run -- list --all

# show task status
cargo run -- task status <uuid>

# update status
cargo run -- task update status <uuid> COMPLETED

# add a note
cargo run -- task note <uuid> "note text"

# show task with notes
cargo run -- task show <uuid>
```

## JSON output

All commands support `--json` for machine-readable output. The JSON schema is in `schema/todoer-output.schema.json`.
