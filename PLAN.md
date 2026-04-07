# Things CLI 实施计划

基于与 things3-cli 的对比分析，优先实现缺失的核心功能。

## Phase 1: 删除功能 (使用 AppleScript)

Things URL Scheme 不支持删除操作，必须使用 AppleScript。

### 任务 1.1: AppleScript 执行器
- [ ] 创建 `core/applescript.rs` 模块
- [ ] 实现 `AppleScriptExecutor` 结构体
- [ ] 实现 `execute_applescript(script: &str) -> Result<String>` 函数

### 任务 1.2: Todo 删除命令
- [ ] 添加 `TodoCommand::Delete` 子命令
- [ ] 实现 `todo::handle_delete(id: &str, force: bool)`
- [ ] 编写删除 todo 的 AppleScript
- [ ] 添加 `--force` 跳过确认

### 任务 1.3: Project 删除命令
- [ ] 添加 `ProjectCommand::Delete` 子命令
- [ ] 实现 `project::handle_delete()`
- [ ] 编写删除 project 的 AppleScript

### 任务 1.4: Area 删除命令
- [x] 创建 `AreaCommand` 命令枚举
- [x] 添加 `Commands::Area(AreaCommand)`
- [x] 实现 `area::handle_delete()`
- [x] 编写删除 area 的 AppleScript

## Phase 2: 区域管理 (Area Management)

### 任务 2.1: Area 添加
- [ ] 添加 `AreaCommand::Add { title, tags }`
- [ ] 实现 `area::handle_add()`
- [ ] 使用 URL Scheme: `things:///add-area?title=...`

### 任务 2.2: Area 更新
- [ ] 添加 `AreaCommand::Update { id, title, tags }`
- [ ] 实现 `area::handle_update()`
- [ ] 使用 URL Scheme + auth-token

### 任务 2.3: Area 显示
- [ ] 添加 `AreaCommand::Show { id }`
- [ ] 从数据库读取 area 详情

## Phase 3: 重复任务支持

### 任务 3.1: 重复参数解析
- [x] 添加 `RepeatPattern` 枚举 (Daily, Weekly, Monthly, Yearly)
- [x] 实现 `--repeat` 参数解析
- [x] 实现 `--repeat-until` 参数

### 任务 3.2: 重复任务添加到 Todo
- [x] 修改 `todo add` 支持 `--repeat`
- [x] 使用 URL Scheme: `things:///add?title=...&repeat=...`

### 任务 3.3: 重复任务更新
- [x] 修改 `todo update` 支持 `--repeat`
- [x] 支持取消重复 `--no-repeat`

## Phase 4: 增强功能

### 任务 4.1: --dry-run 模式
- [ ] 添加全局 `--dry-run` 标志
- [ ] 打印 URL 但不执行

### 任务 4.2: 更多列表命令
- [ ] `things list trash`
- [ ] `things list created-today`
- [ ] `things list logbook`

### 任务 4.3: 命令别名
- [ ] `things create-project` = `things project add`
- [ ] `things create-area` = `things area add`

## 技术实现细节

### AppleScript 删除 Todo 示例
```applescript
tell application "Things3"
    set todoList to to dos
    repeat with t in todoList
        if id of t is "{id}" then
            delete t
            return "deleted"
        end if
    end repeat
    return "not found"
end tell
```

### URL Scheme 参考
- `add-area`: 创建区域
- `update-area`: 更新区域
- `repeat` 参数: day, week, month, year

## 依赖添加
```toml
# Cargo.toml 可能需要添加
[dependencies]
# 用于 AppleScript
osascript = "0.3"  # 或其他合适的 crate
```

## 测试计划
- [ ] 单元测试: AppleScript 生成
- [ ] 集成测试: 删除命令参数解析
- [ ] 手动测试: 与 Things 3 实际交互
