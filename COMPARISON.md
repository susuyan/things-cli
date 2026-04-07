# Things CLI 功能对比分析

对比项目: [ossianhempel/things3-cli](https://github.com/ossianhempel/things3-cli) (Go 实现)

## 功能对比表

| 功能 | things3-cli (Go) | things-cli (Rust) | 状态 |
|------|------------------|-------------------|------|

### 任务/待办管理
| 添加任务 | `add` | `todo add` | ✅ |
| 更新任务 | `update` | `todo update` | ✅ |
| **删除任务** | `delete` | ❌ | 🔴 **缺失** |
| 显示任务详情 | `show` | `show` | ✅ |
| 搜索任务 | `search` | `search` | ✅ |
| 收件箱 | `inbox` | `list inbox` | ✅ |
| 今日 | `today` | `list today` | ✅ |
| 今晚 | - | `list evening` | ✅ |
| 待办 | `upcoming` | `list upcoming` | ✅ |
| 某天 | `someday` | `list someday` | ✅ |
| 任意时间 | `anytime` | `list anytime` | ✅ |
| 已完成 | `completed` | `list completed` | ✅ |
| 今日完成 | `logtoday` | `list completed-today` | ✅ |
| 已取消 | `canceled` | `list canceled` | ✅ |
| 垃圾桶 | `trash` | ❌ | 🟡 可选 |
| 截止日期 | `deadlines` | `list deadlines` | ✅ |
| 重复任务 | `repeating` | ❌ | 🟡 可选 |
| 今日创建 | `createdtoday` | ❌ | 🟡 可选 |
| 所有任务 | `all` | ❌ | 🟡 可选 |
| **重复规则** | `--repeat` | ❌ | 🔴 **缺失** |

### 项目管理
| 添加项目 | `add-project` / `create-project` | `project add` | ✅ |
| 更新项目 | `update-project` | `project update` | ✅ |
| **删除项目** | `delete-project` | ❌ | 🔴 **缺失** |
| 项目列表 | `projects` | `list projects` | ✅ |

### 区域管理
| **添加区域** | `add-area` / `create-area` | ❌ | 🔴 **缺失** |
| **更新区域** | `update-area` | ❌ | 🔴 **缺失** |
| **删除区域** | `delete-area` | ❌ | 🔴 **缺失** |
| 区域列表 | `areas` | `list areas` | ✅ |

### 标签管理
| 标签列表 | `tags` | `list tags` | ✅ |
| 按标签筛选 | `show --tag` | `show --filter` | ✅ |

### 批量操作
| JSON 导入 | - | `batch import` | ✅ |
| 模板生成 | - | `batch template` | ✅ |

### 执行选项
| **干运行预览** | `--dry-run` | ❌ | 🟡 可选 |
| **前台执行** | `--foreground` | ❌ | 🟡 可选 |
| 调试模式 | `-v` / `--verbose` | `--debug` | ✅ |

### 删除实现
| **AppleScript 删除** | ✅ | ❌ | 🔴 **缺失** |

### 文档
| Man 页面 | ✅ | ❌ | 🟡 可选 |
| Shell 补全 | ✅ | ❌ | 🟡 可选 |

## 关键缺失功能 (优先级排序)

### 🔴 高优先级 (核心功能)

1. **删除功能**
   - `things todo delete <ID>`
   - `things project delete <ID>`
   - `things area delete <ID>`
   - 需要 AppleScript 实现，因为 Things URL scheme 不支持删除

2. **区域管理完整功能**
   - `things area add "Area Name"`
   - `things area update <ID> --title "New Name"`
   - `things area delete <ID>`

3. **重复任务支持**
   - `things todo add "Weekly Meeting" --repeat weekly`
   - 支持模式: daily, weekly, monthly, yearly
   - 支持 `--repeat-until 2026-12-31`

### 🟡 中优先级 (增强功能)

4. **执行模式选项**
   - `--dry-run` - 预览 URL 而不执行
   - `--foreground` - 等待 Things 响应

5. **更多列表视图**
   - `things list trash` - 已删除任务
   - `things list created-today` - 今日创建
   - `things list logbook` - 完整日志
   - `things list all` - 所有任务

6. **命令别名**
   - `things create-project` → `things project add`
   - `things create-area` → `things area add`

### 🟢 低优先级 (可选)

7. **文档完善**
   - Man 页面生成
   - Shell 补全脚本 (bash, zsh, fish)

## 技术实现差异

| 特性 | things3-cli (Go) | things-cli (Rust) |
|------|------------------|-------------------|
| 删除实现 | AppleScript | ❌ 未实现 |
| 数据库读取 | ✅ | ✅ |
| URL Scheme | ✅ | ✅ |
| 配置存储 | 文件 | Keychain + 文件 |
| 并发处理 | - | 预留 tokio 支持 |
| 模板系统 | ❌ | ✅ |
| JSON 批量导入 | ❌ | ✅ |

## 建议实现计划

### Phase 1: 删除功能 (最重要)
```bash
# 使用 AppleScript 实现删除
things todo delete <ID> [--force]
things project delete <ID> [--force]
things area delete <ID> [--force]
```

### Phase 2: 区域管理
```bash
things area add "Work" [--tags "..."]
things area update <ID> [--title "..."] [--tags "..."]
```

### Phase 3: 重复任务
```bash
things todo add "Meeting" --repeat weekly --repeat-until 2026-12-31
```

### Phase 4: 增强选项
```bash
things todo add "Test" --dry-run      # 预览 URL
things todo add "Test" --foreground   # 等待响应
```
