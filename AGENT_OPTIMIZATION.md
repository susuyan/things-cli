# Things CLI - Agent 使用优化评估

## 当前状态评估

### 适合 Agent 使用的方面 ✅

1. **命令结构清晰** - 子命令结构易于理解和调用
2. **支持批量操作** - `batch import` 支持 JSON 批量导入
3. **有 `--force` 标志** - 删除操作支持非交互模式
4. **环境变量支持** - `THINGS_AUTH_TOKEN` 和 `THINGS_DEBUG`
5. **数据库查询** - `list` 命令可以直接查询 Things 数据库

### 不适合 Agent 使用的方面 ❌

1. **输出格式不可解析** - 所有输出都是人类可读格式（带颜色、图标）
2. **缺少 JSON 输出模式** - Agent 无法可靠解析 `list` 结果
3. **删除操作有交互** - 虽然有 `--force`，但默认行为是交互式
4. **缺少 `--dry-run`** - 无法预览操作结果
5. **错误信息非结构化** - 错误输出为文本，没有标准格式
6. **无 ID 查询功能** - 无法通过标题查找并返回 ID
7. **缺少 `get` 命令** - 无法获取单个任务/项目/区域的详细信息

---

## 高优先级优化建议

### 1. 添加 `--json` 全局输出标志

**需求**: 所有命令支持输出 JSON 格式

```bash
things list today --json
things todo add "Test" --json
things project list --json
```

**预期输出**:
```json
{
  "success": true,
  "data": [...],
  "count": 5
}
```

**影响文件**:
- `src/cli/args.rs` - 添加全局 `--json` 选项
- `src/cli/commands/list.rs` - JSON 格式输出
- `src/cli/commands/todo.rs` - 操作结果 JSON 输出
- `src/cli/commands/project.rs` - 操作结果 JSON 输出

### 2. 添加 `--dry-run` 标志

**需求**: 预览操作而不实际执行

```bash
things todo add "Test" --dry-run
# 输出: things:///add?title=Test
```

**实现方式**:
- 构建 URL 但不调用 `open`
- 返回 URL 和解析后的参数

### 3. 添加 `get` 命令

**需求**: 获取单个实体的详细信息

```bash
things todo get <ID> --json
things project get <ID> --json
things area get <ID> --json
```

**预期输出**:
```json
{
  "id": "xxx",
  "title": "Task",
  "status": "incomplete",
  "tags": ["work"],
  ...
}
```

### 4. 添加 `find` 命令

**需求**: 通过标题查找 ID（Agent 需要 ID 来操作）

```bash
things todo find "Buy milk" --json
# 返回匹配的 todos 列表
```

### 5. 标准化错误输出

**当前问题**: 错误信息格式不一致

**预期格式**:
```json
{
  "success": false,
  "error": {
    "code": "AUTH_REQUIRED",
    "message": "Authentication required",
    "details": "Run `things config set-auth-token` first"
  }
}
```

---

## 中优先级优化建议

### 6. 添加 `--quiet` 标志

抑制所有非错误输出，只返回退出码。

### 7. 添加 `export` 命令

导出数据为 JSON，便于 Agent 备份或迁移：

```bash
things export --project <ID> --json
things export --area <ID> --json
```

### 8. 改进 `--force` 行为

确保所有可能有交互的操作都有 `--force` 或 `--yes` 选项。

### 9. 添加操作确认/回滚机制

Agent 可能需要确认操作成功：

```bash
things todo add "Test" --wait --json
# 等待 Things 处理并返回创建的 ID
```

---

## 低优先级优化建议

### 10. 添加 `validate` 命令

验证 JSON 文件格式是否正确：

```bash
things batch validate tasks.json
```

### 11. 添加 ID 缓存

Agent 频繁操作时，缓存 ID 到标题的映射：

```bash
things cache refresh  # 刷新标题到 ID 的缓存
```

### 12. 支持配置默认值

通过配置文件设置 Agent 友好的默认值：

```toml
[agent]
json_output = true
quiet = false
dry_run = false
```

---

## 实现路线图

### Phase 1: 核心 Agent 支持 (MVP)

1. 添加 `--json` 全局标志
2. 实现 `list --json` 输出
3. 标准化错误 JSON 输出
4. 添加 `--dry-run` 标志

### Phase 2: 查询能力

5. 实现 `todo get` 命令
6. 实现 `project get` 命令
7. 实现 `area get` 命令
8. 实现 `find` 命令

### Phase 3: 完善

9. 所有命令支持 `--json`
10. 添加 `export` 命令
11. 添加配置选项

---

## 示例 Agent 工作流

### 优化后的使用场景

```bash
# 1. 查找任务 ID
TASK_ID=$(things todo find "Buy milk" --json | jq -r '.data[0].id')

# 2. 获取任务详情
things todo get $TASK_ID --json

# 3. 预览更新
things todo update $TASK_ID --when today --dry-run --json

# 4. 执行更新
things todo update $TASK_ID --when today --json

# 5. 验证更新
things todo get $TASK_ID --json | jq '.when'
```

### 当前限制下的工作流

```bash
# 1. 列出所有任务，人工解析输出
things list today

# 2. 更新时需要已知 ID
things todo update <ID> --when today
# 输出：✓ Todo updated: <ID>
# 需要文本解析确认
```

---

## 代码变更预估

| 功能 | 文件变更 | 复杂度 |
|------|----------|--------|
| `--json` 全局标志 | 5-8 个文件 | 中等 |
| `--dry-run` | 3-5 个文件 | 低 |
| `get` 命令 | 4 个文件 | 中等 |
| `find` 命令 | 2 个文件 | 低 |
| 错误标准化 | 10+ 个文件 | 高 |

---

## 优先级总结

| 优先级 | 功能 | 原因 |
|--------|------|------|
| 🔴 P0 | `--json` 输出 | Agent 无法解析当前输出 |
| 🔴 P0 | `--dry-run` | Agent 需要安全预览 |
| 🟡 P1 | `get` 命令 | 需要验证操作结果 |
| 🟡 P1 | `find` 命令 | 需要 ID 来进行操作 |
| 🟢 P2 | 错误标准化 | 提高可靠性 |
| 🟢 P2 | `--quiet` | 简化输出处理 |
