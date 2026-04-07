# Things CLI - 开发笔记

## 项目状态

### 已完成

- [x] 项目架构设计 (ARCHITECTURE.md)
- [x] 核心模块实现
  - [x] URL Builder - Things URL Scheme 构造
  - [x] 数据模型 - Todo、Project、When 等
  - [x] 执行器 - 调用系统 open 命令
  - [x] 日期解析器 - 自然语言日期解析
- [x] CLI 命令实现
  - [x] todo add/update
  - [x] project add/update
  - [x] show/search
  - [x] batch import/template
  - [x] list (数据库查询)
  - [x] config (keychain 配置)
- [x] 数据库模块 - SQLite 读取 Things 数据库
- [x] 配置模块 - keychain 存储 auth-token
- [x] 编译警告清理
- [x] 单元测试 (13 tests passing)
- [x] README.md 文档
- [x] LICENSE (MIT)

### 待完成

- [x] 集成测试 (16 tests passing)
- [ ] 实际功能测试 (需要 Things 3 应用)
- [ ] Shell 补全生成
- [ ] Homebrew formula
- [ ] GitHub Actions CI/CD
- [ ] 版本发布流程

### 🔴 高优先级 (对比 things3-cli 缺失的功能)

详见 [COMPARISON.md](COMPARISON.md)

1. **删除功能** - AppleScript 实现 ✅
   - [x] `things todo delete <ID>`
   - [x] `things project delete <ID>`
   - [x] `things area delete <ID>`

2. **区域管理完整功能** ✅
   - [x] `things area add "Area Name"`
   - [x] `things area update <ID>`

3. **重复任务支持** ✅
   - [x] `--repeat` 参数 (daily, weekly, monthly, yearly)
   - [x] `--repeat-until` 参数
   - [x] `--no-repeat` 取消重复

### 🟡 中优先级

4. **执行模式选项**
   - [ ] `--dry-run` - 预览 URL
   - [ ] `--foreground` - 前台执行

5. **更多列表视图**
   - [ ] `things list trash`
   - [ ] `things list created-today`
   - [ ] `things list logbook`
   - [ ] `things list all`

6. **命令别名**
   - [ ] `things create-project` → `project add`
   - [ ] `things create-area` → `area add`

## 构建

```bash
# 开发构建
cargo build

# 发布构建
cargo build --release

# 运行测试
cargo test

# 检查代码
cargo check
cargo clippy
```

## 测试状态

### 单元测试 (13 tests)
```
test core::parser::tests::test_parse_when_keywords ... ok
test core::parser::tests::test_parse_when_date ... ok
test core::parser::tests::test_parse_datetime ... ok
test core::parser::tests::test_parse_tags ... ok
test core::url_builder::tests::test_basic_url ... ok
test core::url_builder::tests::test_multiple_params ... ok
test core::url_builder::tests::test_optional_params ... ok
test core::url_builder::tests::test_with_auth ... ok
test core::url_builder::tests::test_multiline ... ok
test core::executor::tests::test_mock_executor ... ok
test config::store::tests::test_config_serialization ... ok
test config::store::tests::test_file_store ... ok
test db::store::tests::test_database_path ... ok
```

### 集成测试 (16 tests)
```
test test_batch_help ... ok
test test_config_help ... ok
test test_debug_flag ... ok
test test_help ... ok
test test_invalid_command ... ok
test test_list_help ... ok
test test_list_subcommands ... ok
test test_project_add_help ... ok
test test_search_help ... ok
test test_show_help ... ok
test test_todo_add_help ... ok
test test_todo_add_missing_required ... ok
test test_todo_update_help ... ok
test test_version ... ok
test test_version_flag ... ok
test test_batch_template_output ... ok
```

**Total: 42 tests passing**

## 已知问题

- 无

## 下一步工作

1. 添加更多单元测试覆盖边界情况
2. 编写集成测试
3. 测试与 Things 3 的实际交互
4. 添加 shell 补全支持
