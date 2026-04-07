# Things CLI

A command-line interface for [Things 3](https://culturedcode.com/things/) on macOS, built with Rust.

[![Rust](https://img.shields.io/badge/rust-2024%20edition-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

## Features

- **Todo Management**: Add and update todos with rich metadata (notes, deadlines, tags, checklists)
- **Project Management**: Create and manage projects with todos
- **Smart Date Parsing**: Natural language dates like "today", "tomorrow", "in 3 days", "next monday"
- **Database Integration**: Read and list tasks, projects, areas, and tags directly from Things database
- **Batch Operations**: Import multiple items via JSON
- **Secure Storage**: Auth tokens stored in macOS Keychain
- **Shell Completions**: Support for bash, zsh, fish, and PowerShell

## Installation

### Prerequisites

- macOS with Things 3 installed
- Rust toolchain (for building from source)

### Build from Source

```bash
git clone https://github.com/yourusername/things-cli.git
cd things-cli
cargo build --release

# Install to /usr/local/bin
sudo cp target/release/things /usr/local/bin/
```

### Homebrew (Coming Soon)

```bash
brew install things-cli
```

## Quick Start

```bash
# Add a simple todo
things todo add "Buy milk"

# Add a todo with details
things todo add "Call mom" --when today --tags "Personal" --notes "Discuss weekend plans"

# Add multiple todos at once
things todo add "Task 1" "Task 2" "Task 3" --list "Shopping"

# Show today's tasks
things show today

# List inbox tasks
things list inbox
```

## Configuration

### Auth Token (Required for Updates)

To update existing todos or projects, you need an auth token.

#### For Agent/Scripting Usage (Recommended)

Set the token as an environment variable:

```bash
export THINGS_AUTH_TOKEN="your-token-here"
```

Add this to your `~/.zshrc`, `~/.bashrc`, or project `.env` file.

#### For Interactive Usage

1. Open Things 3 → Settings → General → Things URLs
2. Copy your Authorization Token
3. Run: `things config set-auth-token` and paste the token

The token will be stored in your macOS Keychain.

### Default Settings

```bash
# Set default list for new todos
things config set-default-list "Personal"

# Set default tags
things config set-default-tags "work,important"
```

## Commands

### Todo Commands

```bash
# Add todos
things todo add "Title" [OPTIONS]

Options:
  -n, --notes <NOTES>          Add notes
  -w, --when <WHEN>            Schedule (today, tomorrow, evening, 2026-03-25, in 3 days)
  -t, --tags <TAGS>            Tags (comma-separated)
  -l, --list <LIST>            Add to list/project by title
      --list-id <ID>           Add to list/project by ID
      --heading <HEADING>      Add under specific heading
      --checklist <ITEMS>      Checklist items (comma-separated)
      --completed              Mark as completed
      --canceled               Mark as canceled
      --reveal                 Show in Things after creation
      --stdin                  Read titles from stdin
      --repeat <PATTERN>       Repeat pattern (day, week, month, year, 2-day, 3-week)
      --repeat-until <DATE>    Repeat until date (YYYY-MM-DD)

# Update todos
things todo update <ID> [OPTIONS]

Options:
  -t, --title <TITLE>          New title
  -n, --notes <NOTES>          Replace notes
      --prepend-notes <TEXT>   Prepend to notes
      --append-notes <TEXT>    Append to notes
  -w, --when <WHEN>            New schedule
      --deadline <DATE>        New deadline (empty to clear)
      --tags <TAGS>            Replace tags
      --add-tags <TAGS>        Add tags
  -l, --list <LIST>            Move to list
      --complete               Mark as complete
      --uncomplete             Mark as incomplete
      --cancel                 Cancel the todo
      --duplicate              Create a copy before updating
      --reveal                 Show after updating
      --repeat <PATTERN>       Repeat pattern (day, week, month, year)
      --repeat-until <DATE>    Repeat until date
      --no-repeat              Cancel repeat

# Delete todos
things todo delete <ID> [OPTIONS]

Options:
  -f, --force                  Delete without confirmation
```

### Project Commands

```bash
# Add a project
things project add "Project Name" [OPTIONS]

Options:
  -n, --notes <NOTES>          Project notes
  -w, --when <WHEN>            Schedule
      --deadline <DATE>        Deadline
  -t, --tags <TAGS>            Tags
  -a, --area <AREA>            Add to area
      --todos <TODOS>          Initial todos (comma-separated)
      --json <FILE>            Create from JSON file

# Update a project
things project update <ID> [OPTIONS]
# (Similar options to todo update)

# Delete a project
things project delete <ID> [--force]
```

### Area Commands

```bash
# Add an area (使用 AppleScript)
things area add "Area Name"

# Update an area (使用 AppleScript)
things area update <ID> --title "New Name"

# Delete an area (使用 AppleScript)
things area delete <ID> [--force]
```

**注意**：Area 操作使用 AppleScript 实现，因为 Things URL Scheme 不支持 area 相关命令。

### List Commands (Database Query)

```bash
things list inbox           # Show inbox tasks
things list today           # Show today's tasks
things list evening         # Show tonight's tasks
things list upcoming        # Show upcoming tasks
things list someday         # Show someday tasks
things list anytime         # Show anytime tasks
things list completed       # Show completed tasks
things list completed-today # Show tasks completed today
things list canceled        # Show canceled tasks
things list deadlines       # Show tasks with deadlines
things list projects        # Show all projects
things list areas           # Show all areas
things list tags            # Show all tags
```

### Show & Search

```bash
# Show a specific list or item
things show [QUERY] [OPTIONS]

Examples:
  things show                 # Show inbox (default)
  things show today           # Show today list
  things show "My Project"    # Show project by title
  things show --id <ID>       # Show item by ID

# Search
things search "keyword"       # Search for keyword
things search                 # Open search window
```

### Batch Operations

```bash
# Import from JSON file
things batch import tasks.json
things batch import - < tasks.json  # From stdin

# Generate template JSON
things batch template
```

### JSON Format

```json
[
  {
    "type": "project",
    "attributes": {
      "title": "Go Shopping",
      "items": [
        {
          "type": "to-do",
          "attributes": {
            "title": "Buy milk"
          }
        },
        {
          "type": "to-do",
          "attributes": {
            "title": "Buy bread",
            "when": "today"
          }
        }
      ]
    }
  },
  {
    "type": "to-do",
    "attributes": {
      "title": "Pick up dry cleaning",
      "when": "evening",
      "tags": ["Errand"]
    }
  }
]
```

### Configuration Commands

```bash
things config set-auth-token [TOKEN]    # Set auth token
things config delete-auth-token         # Remove auth token
things config check-auth-token          # Check if token is set
things config set-default-list <LIST>   # Set default list
things config set-default-tags <TAGS>   # Set default tags
things config show                      # Show current config
things config edit                      # Edit config file
```

## Date Formats

The `--when` parameter accepts various formats:

| Format | Example | Description |
|--------|---------|-------------|
| Keywords | `today`, `tomorrow`, `evening` | Special keywords |
| ISO Date | `2026-03-25` | Specific date |
| Date/Time | `2026-03-25@14:30` | Date with time |
| Relative | `in 3 days` | Relative date |
| Weekday | `next monday` | Next occurrence |

## Environment Variables

| Variable | Description |
|----------|-------------|
| `THINGS_AUTH_TOKEN` | Override auth token from keychain |
| `THINGS_DEBUG` | Enable debug output |

## Shell Completions

Generate shell completions:

```bash
# Bash
things --generate-completions bash > /usr/local/etc/bash_completion.d/things

# Zsh
things --generate-completions zsh > /usr/local/share/zsh/site-functions/_things

# Fish
things --generate-completions fish > ~/.config/fish/completions/things.fish
```

## Examples

### Daily Workflow

```bash
# Morning: Review today's tasks
things list today

# Add tasks throughout the day
things todo add "Review PR" --when today --tags "work"
things todo add "Buy groceries" --when evening --list "Personal"

# Complete a task
things todo update <ID> --complete

# Evening: Check completed tasks
things list completed-today
```

### Project Setup

```bash
# Create a project with initial todos
things project add "Launch Website" \
  --deadline "2026-06-01" \
  --todos "Design mockups,Setup domain,Write content"
```

### Batch Import

```bash
# Create template
cat > project.json << 'EOF'
[
  {
    "type": "project",
    "attributes": {
      "title": "Weekly Review",
      "items": [
        {"type": "to-do", "attributes": {"title": "Review goals"}},
        {"type": "to-do", "attributes": {"title": "Plan next week"}}
      ]
    }
  }
]
EOF

# Import
things batch import project.json
```

## Troubleshooting

### "Things app not found"

Make sure Things 3 is installed from the Mac App Store.

### "Authentication required"

Set your auth token: `things config set-auth-token`

### "URL scheme not enabled"

Enable Things URLs in Things 3 settings:
Things → Settings → General → Things URLs → Enable

### Database access denied

The CLI reads from Things database in read-only mode. If you get permission errors:

1. Grant Full Disk Access to Terminal/iTerm in System Settings
2. Or run: `things list` commands which handle permissions gracefully

## Architecture

See [ARCHITECTURE.md](ARCHITECTURE.md) for detailed design documentation.

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Cultured Code](https://culturedcode.com/) for creating Things 3
- Built with [Rust](https://www.rust-lang.org/) and [clap](https://github.com/clap-rs/clap)
