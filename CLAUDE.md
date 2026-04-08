# Things CLI

A command-line interface for [Things 3](https://culturedcode.com/things/) on macOS, built with Rust.

## Overview

Things CLI allows you to create, update, and manage todos, projects, and areas directly from the terminal using Things 3's URL Scheme and AppleScript integration.

### Key Features

- **Todo Management**: Add and update todos with rich metadata (notes, deadlines, tags, checklists)
- **Project Management**: Create and manage projects with todos
- **Area Management**: Full area support via AppleScript (add, update, delete)
- **Smart Date Parsing**: Natural language dates like "today", "tomorrow", "in 3 days", "next monday"
- **Database Integration**: Read and list tasks, projects, areas, and tags directly from Things database
- **Batch Operations**: Import multiple items via JSON
- **Environment Auth**: Auth token via THINGS_AUTH_TOKEN environment variable
- **Repeat Tasks**: Support for daily, weekly, monthly, yearly repeating patterns

## Quick Start

### Prerequisites

- macOS with Things 3 installed
- Rust toolchain (for building from source)

### Build and Run

```bash
# Clone and build
git clone https://github.com/susuyan/things-cli.git
cd things-cli
cargo build --release

# Install locally
cp target/release/things /usr/local/bin/

# Verify
things --version
```

### Basic Usage

```bash
# Add a simple todo
things todo add "Buy milk"

# Add a todo with details
things todo add "Call mom" --when today --tags "Personal" --notes "Discuss weekend plans"

# Add multiple todos
things todo add "Task 1" "Task 2" "Task 3" --list "Shopping"

# Show today's tasks
things show today

# List inbox tasks
things list inbox

# Search
things search "work"
```

## Architecture

Things CLI follows a layered architecture:

```
┌─────────────────────────────────────┐
│           CLI Layer                 │
│    (clap argument parsing)          │
├─────────────────────────────────────┤
│         Command Handlers            │
│  (todo, project, area, list, etc.)  │
├─────────────────────────────────────┤
│          Core Logic                 │
│  (URL Builder, Executor, Parser)    │
├─────────────────────────────────────┤
│       Integration Layer             │
│  (Things URL Scheme, AppleScript,   │
│   SQLite Database)                  │
└─────────────────────────────────────┘
```

### Module Structure

```
src/
├── main.rs              # Application entry point
├── lib.rs               # Library exports
├── cli/                 # CLI argument and command handling
│   ├── args.rs          # clap argument definitions
│   ├── commands/        # Command implementations
│   │   ├── todo.rs
│   │   ├── project.rs
│   │   ├── area.rs
│   │   ├── list.rs
│   │   ├── show.rs
│   │   ├── search.rs
│   │   ├── batch.rs
│   │   └── config.rs
│   └── display.rs       # Output formatting
├── core/                # Core business logic
│   ├── url_builder.rs   # Things URL Scheme construction
│   ├── executor.rs      # URL execution (open command)
│   ├── applescript.rs   # AppleScript for unsupported operations
│   ├── parser.rs        # Date and input parsing
│   └── models.rs        # Data models
├── db/                  # Database access
│   └── store.rs         # SQLite queries for Things database
├── config/              # Configuration management
│   └── store.rs         # File storage (environment for auth)
└── json/                # JSON batch operations
    └── types.rs
```

For detailed architecture documentation, see [ARCHITECTURE.md](ARCHITECTURE.md).

## Development Guide

### Building

```bash
# Development build
cargo build

# Release build
cargo build --release

# Check without building
cargo check

# Run clippy lints
cargo clippy
```

### Project Conventions

- **URL Scheme First**: Use Things URL Scheme for operations when available
- **AppleScript Fallback**: Use AppleScript for operations not supported by URL Scheme (delete, area add/update)
- **Database for Reading**: Query SQLite directly for listing operations (read-only)
- **Environment Variables**: Auth token configured via THINGS_AUTH_TOKEN

### Adding New Commands

1. Define arguments in `src/cli/args.rs`
2. Implement handler in `src/cli/commands/<feature>.rs`
3. Add to command router in `src/main.rs`
4. Write tests in `tests/integration_tests.rs`
5. Add E2E test in `e2e/test_<feature>.sh`

## Testing

The project has three levels of testing:

### Unit Tests

```bash
cargo test
```

Unit tests cover core logic:
- URL building and encoding
- Date parsing
- Model serialization
- Config storage

### Integration Tests

```bash
cargo test --test integration_tests
```

Integration tests verify CLI behavior without requiring Things 3 to be running:
- Command parsing
- Help output
- Error handling
- Flag validation

### E2E Tests

```bash
cd e2e && ./run_all.sh
```

**Warning**: E2E tests create real data in your Things 3 database!

E2E tests verify actual functionality against the Things 3 app:
- Todo CRUD operations
- Project management
- Area management
- Batch operations
- Show and search

## Configuration

### Auth Token (Required for Updates)

To update existing todos or projects, you need to set an auth token:

1. Open Things 3 → Settings → General → Things URLs
2. Copy your Authorization Token
3. Run: `things config set-auth-token` and paste the token

The token is configured via the THINGS_AUTH_TOKEN environment variable.

### Environment Variables

| Variable | Description |
|----------|-------------|
| `THINGS_AUTH_TOKEN` | Auth token for updates |
| `THINGS_DEBUG` | Enable debug output |

## Roadmap

### Completed ✅

- [x] Core URL Scheme integration
- [x] Todo add/update
- [x] Project add/update
- [x] Area management (via AppleScript)
- [x] Delete operations (via AppleScript)
- [x] Repeat task support
- [x] Database query for listings
- [x] Batch import from JSON
- [x] Shell completions
- [x] E2E test suite
- [x] GitHub repository setup
- [x] **Agent Support: `--json` output for all commands**
- [x] **Agent Support: `--dry-run` preview mode**
- [x] **Agent Support: `get` and `find` commands**
- [x] **Agent Support: Environment variable auth token**

### In Progress 🚧

- [x] Homebrew formula
- [x] GitHub Actions CI/CD
- [x] Version release workflow

### Planned 📋

- [ ] `--foreground` execution mode
- [ ] Additional list views (logbook, all)
- [ ] Command aliases (create-project, create-area)
- [ ] Interactive mode

## Claude Code Integration

Things CLI is optimized for use with Claude Code and other AI agents.

### Quick Setup for Claude Code

1. **Install the CLI**
   ```bash
   cargo build --release
   cp target/release/things /usr/local/bin/
   ```

2. **Configure Auth Token (via environment variable)**
   ```bash
   # Add to ~/.zshrc or ~/.bashrc
   export THINGS_AUTH_TOKEN="your-token-from-things-app"
   ```

3. **Start using in Claude Code**
   ```
   You: 查看我今天的任务
   Claude: [runs: things list today --json]
   
   You: 添加一个任务叫"完成报告"
   Claude: [runs: things todo add "完成报告" --when today --json]
   ```

### Agent-Friendly Features

- **`--json`**: All commands support JSON output for structured parsing
- **`--dry-run`**: Preview operations without executing
- **`todo find`**: Search tasks by title to get IDs
- **Environment variable auth**: `THINGS_AUTH_TOKEN` for non-interactive use

### Example Agent Workflows

```bash
# Workflow 1: Find and complete a task
TASK_ID=$(things todo find "Buy milk" --json | jq -r '.data[0].uuid')
things todo update $TASK_ID --complete --json

# Workflow 2: Add project with todos
PROJECT_ID=$(things project add "Website Launch" --json | jq -r '.data.id')
things todo add "Design mockup" --list-id $PROJECT_ID --json
things todo add "Write content" --list-id $PROJECT_ID --json

# Workflow 3: Review today's tasks and reschedule incomplete
things list today --json | jq -r '.data[] | select(.status != "Completed") | .uuid' | \
  while read id; do
    things todo update $id --when tomorrow --json
  done
```

For detailed integration options, see [CLAUDE_CODE_INTEGRATION.md](CLAUDE_CODE_INTEGRATION.md).

## Related Documents

- [README.md](README.md) - User documentation
- [ARCHITECTURE.md](ARCHITECTURE.md) - Detailed architecture design
- [COMPARISON.md](COMPARISON.md) - Comparison with things3-cli (Go)
- [e2e/README.md](e2e/README.md) - E2E testing guide
- [CLAUDE_CODE_INTEGRATION.md](CLAUDE_CODE_INTEGRATION.md) - Claude Code integration guide

## License

MIT License - see [LICENSE](LICENSE) file for details.
