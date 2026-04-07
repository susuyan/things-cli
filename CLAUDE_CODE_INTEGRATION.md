# Things CLI - Claude Code 集成指南

## 概述

Things CLI 可以通过多种方式与 Claude Code 集成，让 AI 能够管理你的 Things 3 任务。

## 集成方式对比

| 方式 | 复杂度 | 灵活性 | 推荐场景 |
|------|--------|--------|----------|
| **直接使用 Bash** | ⭐ 最低 | ⭐⭐⭐ 高 | 快速开始，偶尔使用 |
| **配置为 Skill** | ⭐⭐ 中 | ⭐⭐⭐ 高 | 频繁使用，需要参数提示 |
| **MCP 服务器** | ⭐⭐⭐ 高 | ⭐⭐⭐⭐ 最高 | 深度集成，复杂工作流 |

---

## 方式一：直接使用 Bash（推荐快速开始）

Claude Code 可以直接使用 Bash 工具调用 `things` 命令。

### 配置步骤

1. **确保 CLI 已安装**

```bash
which things
things --version
```

2. **配置环境变量**

在你的 shell 配置中（~/.zshrc 或 ~/.bashrc）：

```bash
export THINGS_AUTH_TOKEN="your-auth-token-here"
```

3. **开始使用**

直接在 Claude Code 中询问：

```
用户: 帮我查看今天的任务
Claude: 我来帮你查看今天的任务。
       [使用 Bash 工具执行: things list today --json]

用户: 添加一个任务叫"买牛奶"
Claude: 我来帮你添加任务。
       [使用 Bash 工具执行: things todo add "买牛奶" --json]
```

### 优缺点

**优点**:
- 零配置，开箱即用
- 灵活，可以组合各种命令
- 不需要额外开发

**缺点**:
- AI 需要知道正确的命令格式
- 没有智能提示和参数验证
- 每次都需要构造完整命令

---

## 方式二：配置为 Skill（推荐日常使用）

通过 Claude Code 的 Skill 配置，为 Things CLI 添加智能提示和快捷命令。

### 配置步骤

在你的 Claude Code 配置文件（`~/.claude/settings.json`）中添加：

```json
{
  "skills": {
    "things": {
      "description": "Things 3 任务管理",
      "prompt": "Use the `things` CLI to manage Things 3 tasks. Always use --json flag for structured output.",
      "shortcuts": {
        "things-today": {
          "command": "things list today --json",
          "description": "查看今天的任务"
        },
        "things-inbox": {
          "command": "things list inbox --json",
          "description": "查看收件箱"
        },
        "things-add": {
          "command": "things todo add {title} --json",
          "description": "添加任务",
          "args": ["title"]
        },
        "things-find": {
          "command": "things todo find {query} --json",
          "description": "搜索任务",
          "args": ["query"]
        },
        "things-complete": {
          "command": "things todo update {id} --complete --json",
          "description": "完成任务",
          "args": ["id"]
        }
      }
    }
  }
}
```

### 使用方式

配置后，你可以在 Claude Code 中使用以下方式：

```
/things-today              # 快捷查看今天任务
/things-add "买牛奶"       # 快捷添加任务
/things-find "工作"        # 快捷搜索任务
```

或者在对话中：

```
用户: 我今天有什么任务？
Claude: [自动使用 things-today skill 查询]

用户: 添加一个买牛奶的任务
Claude: [自动使用 things-add skill 添加]
```

---

## 方式三：创建 MCP 服务器（推荐深度集成）

如果你需要更高级的集成，可以创建一个 MCP（Model Context Protocol）服务器。

### 为什么不直接做 Skill？

MCP 服务器适合：
- 需要复杂的参数转换
- 需要状态管理
- 需要批量处理
- 需要与多个系统联动

### 简单 MCP 配置示例

创建 `things-mcp.json`：

```json
{
  "name": "things-cli",
  "version": "1.0.0",
  "description": "Things 3 CLI integration for Claude Code",
  "tools": [
    {
      "name": "list_tasks",
      "description": "列出任务",
      "parameters": {
        "type": "object",
        "properties": {
          "list": {
            "type": "string",
            "enum": ["today", "inbox", "upcoming", "someday"],
            "description": "列表类型"
          }
        },
        "required": ["list"]
      }
    },
    {
      "name": "add_task",
      "description": "添加任务",
      "parameters": {
        "type": "object",
        "properties": {
          "title": {
            "type": "string",
            "description": "任务标题"
          },
          "when": {
            "type": "string",
            "description": "时间安排 (today, tomorrow, evening)"
          },
          "tags": {
            "type": "array",
            "items": { "type": "string" },
            "description": "标签"
          }
        },
        "required": ["title"]
      }
    },
    {
      "name": "complete_task",
      "description": "完成任务",
      "parameters": {
        "type": "object",
        "properties": {
          "id": {
            "type": "string",
            "description": "任务 ID"
          }
        },
        "required": ["id"]
      }
    }
  ]
}
```

### 更简单的方案：函数包装器

实际上，对于 Things CLI，更实用的方案是创建一个简单的 wrapper 脚本：

```bash
#!/bin/bash
# things-wrapper.sh

# 简化命令，让 AI 更容易使用

case "$1" in
  today)
    things list today --json
    ;;
  inbox)
    things list inbox --json
    ;;
  add)
    shift
    things todo add "$@" --json
    ;;
  done)
    things todo update "$2" --complete --json
    ;;
  find)
    things todo find "$2" --json
    ;;
  *)
    things "$@"
    ;;
esac
```

---

## 推荐配置（CLAUDE.md）

在你的项目根目录创建/修改 `CLAUDE.md`：

```markdown
# Things CLI Quick Reference

## Available Commands

### List Tasks
```bash
things list today --json          # Today's tasks
things list inbox --json          # Inbox
things list upcoming --json       # Upcoming
```

### Add Tasks
```bash
things todo add "Title" --json
things todo add "Title" --when today --tags work --json
```

### Find Tasks
```bash
things todo find "keyword" --json
```

### Complete Tasks
```bash
things todo update <ID> --complete --json
```

### Projects
```bash
things list projects --json
things project add "Project Name" --json
```

## Agent Workflow Best Practices

1. Always use `--json` for structured output
2. Use `--dry-run` to preview changes
3. Find task ID first, then operate on it
4. Quote titles with spaces

## Example Workflows

### Add and schedule a task
```bash
# Add task
things todo add "Review PR" --when today --tags work --json

# Find it if needed
things todo find "Review PR" --json

# Mark complete
things todo update <ID> --complete --json
```

### Batch create from list
```bash
# Create multiple tasks
for title in "Task 1" "Task 2" "Task 3"; do
  things todo add "$title" --list "Batch" --json
done
```
```

---

## 当前项目 CLAUDE.md 需要更新

我发现当前的 CLAUDE.md 缺少 Agent 使用指南。需要添加：

1. 环境变量配置说明
2. JSON 输出最佳实践
3. 常用 Agent 工作流

让我来更新：
