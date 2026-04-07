# Things CLI - Rust 架构设计方案

## 1. 项目概述

一个用 Rust 编写的命令行工具，通过 Things URL Scheme 与 Things 3 应用交互。

### 核心设计原则
- **直观**: 命令命名与 Things 概念一致
- **简洁**: 常用操作只需少量参数
- **强大**: 支持复杂的批量操作和 JSON 模式
- **安全**: 敏感配置（auth-token）存储在系统 keychain

---

## 2. 命令结构

```bash
things-cli [COMMAND] [OPTIONS] [ARGS]
```

### 2.1 命令映射

| URL Scheme | CLI 命令 | 说明 |
|------------|----------|------|
| `add` | `todo add` | 添加待办事项 |
| `add-project` | `project add` | 添加项目 |
| `update` | `todo update` | 更新待办事项 |
| `update-project` | `project update` | 更新项目 |
| `show` | `show` | 显示列表/项目/待办 |
| `search` | `search` | 搜索 |
| `version` | `version` | 版本信息 |
| `json` | `batch` / `import` | 批量 JSON 操作 |

### 2.2 完整命令设计

```bash
# === 待办事项 (todo) ===

# 添加待办事项
things-cli todo add "Buy milk"
things-cli todo add "Buy milk" --notes "Low fat" --when today --tags "Errand"
things-cli todo add "Task 1" "Task 2" "Task 3" --list "Shopping"  # 批量添加

# 从 stdin 批量添加
cat tasks.txt | things-cli todo add --list "Shopping"

# 更新待办事项
things-cli todo update <id> --when tomorrow
things-cli todo update <id> --complete
things-cli todo update <id> --append-notes "Additional info"

# === 项目 (project) ===

# 添加项目
things-cli project add "Build treehouse"
things-cli project add "Plan Party" --area "Family" --deadline "2026-12-31"

# 批量添加项目带待办
cat project.json | things-cli project add --json

# 更新项目
things-cli project update <id> --when today
things-cli project update <id> --add-tags "Important"

# === 导航与搜索 (show/search) ===

# 显示列表
things-cli show today
things-cli show inbox
things-cli show "Shopping"              # 按标题匹配
things-cli show --id "Qi9pM1heCNAZxKREgQrwnJ"

# 显示并过滤
things-cli show today --filter "Errand"

# 搜索
things-cli search "vacation"
things-cli search                         # 打开搜索界面

# === 批量操作 (batch) ===

# 从 JSON 文件导入
things-cli batch import plan.json
things-cli batch import - < plan.json     # 从 stdin

# 生成示例 JSON 模板
things-cli batch template --output template.json

# === 配置 (config) ===

# 设置 auth-token
things-cli config set-auth-token
# 交互式提示输入 token

# 查看配置
things-cli config show

# === 版本 ===
things-cli version
things-cli --version
```

---

## 3. 模块架构

```
src/
├── main.rs              # 入口，CLI 参数解析
├── lib.rs               # 库导出
├── cli/                 # CLI 层
│   ├── mod.rs
│   ├── args.rs          # clap 参数定义
│   └── commands/        # 命令处理
│       ├── mod.rs
│       ├── todo.rs
│       ├── project.rs
│       ├── show.rs
│       ├── search.rs
│       ├── batch.rs
│       └── config.rs
├── core/                # 核心逻辑层
│   ├── mod.rs
│   ├── url_builder.rs   # URL Scheme 构造器
│   ├── executor.rs      # URL 执行（调用 open）
│   ├── models.rs        # 数据模型
│   └── parser.rs        # 输入解析（日期、列表等）
├── json/                # JSON 操作
│   ├── mod.rs
│   ├── types.rs         # JSON 结构定义
│   └── validator.rs     # JSON 验证
└── config/              # 配置管理
    ├── mod.rs
    ├── store.rs         # 配置存储（keychain + 文件）
    └── auth.rs          # 授权管理
```

---

## 4. 核心组件设计

### 4.1 URL Builder

```rust
// core/url_builder.rs

pub struct ThingsUrl {
    command: Command,
    params: Vec<(String, String)>,
}

pub enum Command {
    Add,
    AddProject,
    Update(String),      // id
    UpdateProject(String), // id
    Show,
    Search,
    Version,
    Json,
}

impl ThingsUrl {
    pub fn new(command: Command) -> Self;
    pub fn param(mut self, key: &str, value: &str) -> Self;
    pub fn param_opt(mut self, key: &str, value: Option<&str>) -> Self;
    pub fn param_bool(mut self, key: &str, value: bool) -> Self;
    pub fn build(self) -> String;
}
```

### 4.2 执行器

```rust
// core/executor.rs

pub trait Executor {
    fn execute(&self, url: &str) -> Result<ExecutionResult>;
}

pub struct OpenExecutor;

impl Executor for OpenExecutor {
    fn execute(&self, url: &str) -> Result<ExecutionResult> {
        // macOS: open "things:///..."
        // 未来可扩展支持 x-callback-url 回调处理
    }
}

pub struct ExecutionResult {
    pub success: bool,
    pub x_things_id: Option<String>,  // 返回的 ID
}
```

### 4.3 配置存储

```rust
// config/store.rs

pub struct Config {
    pub auth_token: Option<String>,
    pub default_list: Option<String>,
    pub default_tags: Vec<String>,
}

pub trait ConfigStore {
    fn load(&self) -> Result<Config>;
    fn save(&self, config: &Config) -> Result<()>;
    fn set_auth_token(&self, token: &str) -> Result<()>;
    fn get_auth_token(&self) -> Result<Option<String>>;
}

// macOS Keychain 实现
pub struct KeychainStore;

// 文件配置实现（非敏感信息）
pub struct FileStore {
    path: PathBuf,
}
```

---

## 5. 数据模型

### 5.1 待办事项

```rust
// core/models.rs

pub struct Todo {
    pub title: String,
    pub notes: Option<String>,
    pub when: Option<When>,
    pub deadline: Option<NaiveDate>,
    pub tags: Vec<String>,
    pub list: Option<ListRef>,
    pub heading: Option<String>,
    pub completed: bool,
    pub canceled: bool,
    pub checklist_items: Vec<String>,
}

pub enum When {
    Today,
    Tomorrow,
    Evening,
    Anytime,
    Someday,
    Date(NaiveDate),
    DateTime(NaiveDateTime),
    Natural(String),  // "in 3 days", "next tuesday"
}

pub enum ListRef {
    Inbox,
    Id(String),
    Title(String),
}
```

### 5.2 JSON 类型

```rust
// json/types.rs

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ThingsObject {
    #[serde(rename = "to-do")]
    Todo(TodoAttributes),
    #[serde(rename = "project")]
    Project(ProjectAttributes),
    #[serde(rename = "heading")]
    Heading(HeadingAttributes),
    #[serde(rename = "checklist-item")]
    ChecklistItem(ChecklistItemAttributes),
}

#[derive(Serialize, Deserialize)]
pub struct TodoAttributes {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub when: Option<String>,
    // ... 其他字段
}

// 支持 operation 字段
#[derive(Serialize, Deserialize)]
pub struct Operation {
    #[serde(flatten)]
    pub object: ThingsObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<OperationType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}
```

---

## 6. 依赖选择

```toml
[dependencies]
# CLI
clap = { version = "4", features = ["derive", "cargo"] }
clap_complete = "4"          # shell 补全生成

# 序列化/反序列化
serde = { version = "1", features = ["derive"] }
serde_json = "1"
toml = "0.8"                 # 配置文件

# 日期时间
chrono = { version = "0.4", features = ["serde"] }

# 错误处理
thiserror = "1"
anyhow = "1"

# 交互式输入
dialoguer = "0.11"           # 交互式提示
indicatif = "0.17"           # 进度条

# HTTP/URL
urlencoding = "2"            # URL 编码

# 配置存储
keyring = "2"                # 系统 keychain 访问
dirs = "5"                   # 获取配置目录

# 其他
colored = "2"                # 终端颜色输出
handlebars = "5"             # 模板（用于生成示例）
```

---

## 7. 特殊功能设计

### 7.1 智能日期解析

```rust
// core/parser.rs

pub fn parse_when(input: &str) -> Result<When> {
    match input.to_lowercase().as_str() {
        "today" | "今" | "今天" => Ok(When::Today),
        "tomorrow" | "明" | "明天" => Ok(When::Tomorrow),
        "evening" | "今晚" => Ok(When::Evening),
        "anytime" | "任意时间" => Ok(When::Anytime),
        "someday" | "某天" => Ok(When::Someday),
        _ => {
            // 尝试解析日期: 2026-03-25
            if let Ok(date) = NaiveDate::parse_from_str(input, "%Y-%m-%d") {
                return Ok(When::Date(date));
            }
            // 尝试自然语言: "in 3 days", "next monday"
            Ok(When::Natural(input.to_string()))
        }
    }
}
```

### 7.2 批量添加优化

```rust
// 支持从多行文本批量创建
// things-cli todo add --list "Shopping" < tasks.txt

pub fn batch_add_todos<R: BufRead>(reader: R, options: &AddOptions) -> Result<Vec<String>> {
    let titles: Vec<String> = reader
        .lines()
        .filter_map(|l| l.ok())
        .filter(|l| !l.trim().is_empty())
        .collect();
    
    // 如果超过一定数量，使用 titles 参数一次性发送
    // 否则逐个发送
}
```

### 7.3 模板系统

```bash
# 生成常用模板
things-cli batch template --type daily-review
things-cli batch template --type weekly-plan
things-cli batch template --type project-skeleton
```

---

## 8. 错误处理策略

```rust
#[derive(thiserror::Error, Debug)]
pub enum ThingsError {
    #[error("Things app not found. Please install Things 3 from the App Store.")]
    AppNotFound,
    
    #[error("URL scheme not enabled. Please enable it in Things settings.")]
    SchemeNotEnabled,
    
    #[error("Authentication required. Please run `things-cli config set-auth-token`")]
    AuthRequired,
    
    #[error("Invalid date format: {0}")]
    InvalidDate(String),
    
    #[error("Invalid ID: {0}")]
    InvalidId(String),
    
    #[error("Things returned an error: {0}")]
    ThingsError(String),
    
    #[error(transparent)]
    Io(#[from] std::io::Error),
    
    #[error(transparent)]
    Json(#[from] serde_json::Error),
}
```

---

## 9. 测试策略

```rust
// 单元测试
#[cfg(test)]
mod tests {
    #[test]
    fn test_url_builder() {
        let url = ThingsUrl::new(Command::Add)
            .param("title", "Buy milk")
            .param("when", "today")
            .build();
        assert_eq!(url, "things:///add?title=Buy%20milk&when=today");
    }
    
    #[test]
    fn test_date_parser() {
        assert!(matches!(parse_when("today"), Ok(When::Today)));
        assert!(matches!(parse_when("2026-03-25"), Ok(When::Date(_))));
    }
}

// 集成测试（使用 mock executor）
#[test]
fn test_add_todo() {
    let mock = MockExecutor::new();
    let cli = ThingsCli::new(mock);
    
    cli.run(&["todo", "add", "Test task"]);
    
    assert_eq!(mock.last_url(), "things:///add?title=Test%20task");
}
```

---

## 10. 扩展性考虑

### 10.1 未来可能的扩展

1. **x-callback-url 回调处理**
   - 启动本地 HTTP 服务器接收回调
   - 获取创建的 ID 进行后续操作

2. **Things Cloud API**
   - 如果 Things 开放 Cloud API，可以无缝切换

3. **插件系统**
   - 支持自定义脚本扩展

4. **同步功能**
   - 双向同步（需要 Things 提供查询接口）

### 10.2 配置热重载

```rust
pub struct ConfigWatcher {
    // 监听配置文件变化
    // 自动重新加载
}
```

---

## 11. 开发路线图

### Phase 1: 基础功能 (MVP)
- [ ] 项目脚手架
- [ ] `todo add` 基础功能
- [ ] `show` 基础功能
- [ ] `search` 基础功能
- [ ] 配置文件管理

### Phase 2: 完整功能
- [ ] `todo update`
- [ ] `project add/update`
- [ ] `batch import`
- [ ] 智能日期解析
- [ ] Shell 补全

### Phase 3: 高级功能
- [ ] 批量操作优化
- [ ] 模板系统
- [ ] 交互式模式
- [ ] 更好的错误提示

### Phase 4: 生态
- [ ] Homebrew 发布
- [ ] 文档完善
- [ ] 示例脚本集
