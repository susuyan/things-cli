# Things CLI Agent 优化实现计划

## 目标
将 Things CLI 优化为 Agent 友好型工具，支持结构化输出、预览操作和 ID 查询。

## 阶段一：核心基础 (Phase 1)

### 任务 1: 添加 `--json` 全局输出标志
**负责人**: Agent-1
**文件**:
- `src/cli/args.rs` - 在 GlobalOpts 中添加 `--json` 标志
- `src/cli/mod.rs` - 传递 json 标志到命令处理
- `src/cli/commands/mod.rs` - 修改 handle_command 支持 json 模式
- `src/cli/commands/list.rs` - 实现 print_tasks_json, print_projects_json 等

**验收标准**:
```bash
things list today --json  # 输出纯 JSON
things list projects --json
things list areas --json
```

**输出格式**:
```json
{
  "success": true,
  "type": "tasks",
  "count": 5,
  "data": [
    {"id": "uuid", "title": "Task", "status": "incomplete", ...}
  ]
}
```

---

### 任务 2: 添加 `--dry-run` 标志
**负责人**: Agent-2
**文件**:
- `src/cli/args.rs` - 在 GlobalOpts 中添加 `--dry-run` 标志
- `src/cli/commands/todo.rs` - handle_add/handle_update 支持 dry-run
- `src/cli/commands/project.rs` - 同上
- `src/core/executor.rs` - DryRunExecutor 或修改 OpenExecutor

**验收标准**:
```bash
things todo add "Test" --dry-run --json
# {"success": true, "dry_run": true, "url": "things:///add?title=Test"}
```

---

## 阶段二：查询能力 (Phase 2)

### 任务 3: 实现 `todo get` 命令
**负责人**: Agent-3
**文件**:
- `src/cli/args.rs` - 添加 TodoCommand::Get { id: String }
- `src/cli/commands/todo.rs` - handle_get 函数
- `src/db/store.rs` - 可能需要添加 get_task_by_id 方法

**验收标准**:
```bash
things todo get <ID> --json
# 返回单个 todo 的详细信息
```

---

### 任务 4: 实现 `todo find` 命令
**负责人**: Agent-4
**文件**:
- `src/cli/args.rs` - 添加 TodoCommand::Find { title: String, limit: Option<usize> }
- `src/cli/commands/todo.rs` - handle_find 函数
- `src/db/store.rs` - 添加 search_tasks_by_title 方法

**验收标准**:
```bash
things todo find "Buy milk" --json
# 返回匹配的 todos 列表
```

---

### 任务 5: 实现 `project get` 和 `project find`
**负责人**: Agent-5
**文件**:
- `src/cli/args.rs` - 添加 ProjectCommand::Get 和 ProjectCommand::Find
- `src/cli/commands/project.rs` - handle_get, handle_find 函数
- `src/db/store.rs` - 添加 get_project_by_id, search_projects_by_title

---

### 任务 6: 实现 `area get` 命令
**负责人**: Agent-6
**文件**:
- `src/cli/args.rs` - 添加 AreaCommand::Get
- `src/cli/commands/area.rs` - handle_get 函数
- `src/db/store.rs` - 添加 get_area_by_id

---

## 阶段三：完善 (Phase 3)

### 任务 7: 标准化错误输出 (JSON 模式)
**负责人**: Agent-7
**文件**:
- `src/cli/mod.rs` - 修改错误处理，支持 JSON 格式错误
- 所有命令处理函数 - 使用统一的错误格式

**错误格式**:
```json
{
  "success": false,
  "error": {
    "code": "AUTH_REQUIRED",
    "message": "..."
  }
}
```

---

### 任务 8: 操作命令支持 `--json` 输出
**负责人**: Agent-8
**文件**:
- `src/cli/commands/todo.rs` - add/update/delete 的 JSON 输出
- `src/cli/commands/project.rs` - 同上
- `src/cli/commands/area.rs` - 同上

**验收标准**:
```bash
things todo add "Test" --json
# {"success": true, "message": "Todo added: Test"}

things todo delete <ID> --force --json
# {"success": true, "message": "Todo deleted: <ID>"}
```

---

## 依赖关系

```
任务 1 (json基础) ──┬── 任务 3 (todo get)
                   ├── 任务 4 (todo find)
                   ├── 任务 5 (project get/find)
                   ├── 任务 6 (area get)
                   └── 任务 8 (操作命令json)

任务 2 (dry-run) ── 独立

任务 7 (错误处理) ── 依赖任务 1
```

## 执行顺序

**第一波** (并行):
- Agent-1: 任务 1 (--json 基础)
- Agent-2: 任务 2 (--dry-run)

**第二波** (等待任务1完成):
- Agent-3: 任务 3 (todo get)
- Agent-4: 任务 4 (todo find)
- Agent-5: 任务 5 (project get/find)
- Agent-6: 任务 6 (area get)

**第三波** (等待前面完成):
- Agent-7: 任务 7 (错误处理)
- Agent-8: 任务 8 (操作命令json)

## 测试验证

每个 Agent 完成后需要:
1. 运行 `cargo build` 确保编译通过
2. 运行 `cargo test` 确保测试通过
3. 手动验证新功能
