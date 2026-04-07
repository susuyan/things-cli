use std::fmt;
use urlencoding::encode;

/// URL Scheme 命令
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub enum Command {
    /// 添加待办事项
    Add,
    /// 添加项目
    AddProject,
    /// 添加区域
    AddArea,
    /// 更新待办事项
    Update(String), // id
    /// 更新项目
    UpdateProject(String), // id
    /// 更新区域
    UpdateArea(String), // id
    /// 显示列表/项目
    Show,
    /// 搜索
    Search,
    /// 获取版本
    Version,
    /// JSON 批量操作
    Json,
}

impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Command::Add => write!(f, "add"),
            Command::AddProject => write!(f, "add-project"),
            Command::AddArea => write!(f, "add-area"),
            Command::Update(_) => write!(f, "update"),
            Command::UpdateProject(_) => write!(f, "update-project"),
            Command::UpdateArea(_) => write!(f, "update-area"),
            Command::Show => write!(f, "show"),
            Command::Search => write!(f, "search"),
            Command::Version => write!(f, "version"),
            Command::Json => write!(f, "json"),
        }
    }
}

/// Things URL 构造器
#[derive(Debug)]
pub struct ThingsUrl {
    command: Command,
    params: Vec<(String, String)>,
}

impl ThingsUrl {
    /// 创建新的 URL 构造器
    pub fn new(command: Command) -> Self {
        Self {
            command,
            params: Vec::new(),
        }
    }

    /// 添加参数
    pub fn param(mut self, key: &str, value: &str) -> Self {
        self.params.push((key.to_string(), value.to_string()));
        self
    }

    /// 可选参数
    pub fn param_opt(mut self, key: &str, value: Option<&str>) -> Self {
        if let Some(v) = value {
            self.params.push((key.to_string(), v.to_string()));
        }
        self
    }

    /// 布尔参数
    pub fn param_bool(mut self, key: &str, value: bool) -> Self {
        self.params.push((key.to_string(), value.to_string()));
        self
    }

    /// 可选布尔参数（只有 true 时才添加）
    #[allow(dead_code)]
    pub fn param_bool_opt(mut self, key: &str, value: Option<bool>) -> Self {
        if let Some(true) = value {
            self.params.push((key.to_string(), "true".to_string()));
        }
        self
    }

    /// 多行字符串参数（用 %0a 分隔）
    pub fn param_multiline(mut self, key: &str, values: &[String]) -> Self {
        if !values.is_empty() {
            let joined = values.join("\n");
            self.params.push((key.to_string(), joined));
        }
        self
    }

    /// 构建最终 URL
    pub fn build(self) -> String {
        let mut url = format!("things:///{}", self.command);

        // 收集参数
        let mut all_params: Vec<(String, String)> = self.params;

        // 对于 Update、UpdateProject 和 UpdateArea 命令，自动添加 id 参数
        match &self.command {
            Command::Update(id) => {
                all_params.insert(0, ("id".to_string(), id.clone()));
            }
            Command::UpdateProject(id) => {
                all_params.insert(0, ("id".to_string(), id.clone()));
            }
            Command::UpdateArea(id) => {
                all_params.insert(0, ("id".to_string(), id.clone()));
            }
            _ => {}
        }

        if !all_params.is_empty() {
            url.push('?');
            let params: Vec<String> = all_params
                .into_iter()
                .map(|(k, v)| format!("{}={}", encode(&k), encode(&v)))
                .collect();
            url.push_str(&params.join("&"));
        }

        url
    }

    /// 为需要 auth-token 的命令构建 URL
    pub fn build_with_auth(self, auth_token: &str) -> String {
        let mut url = format!("things:///{}", self.command);

        // auth-token 总是第一个参数
        let mut params = vec![format!("auth-token={}", encode(auth_token))];

        // 对于 Update、UpdateProject 和 UpdateArea 命令，自动添加 id 参数
        match &self.command {
            Command::Update(id) => {
                params.push(format!("id={}", encode(id)));
            }
            Command::UpdateProject(id) => {
                params.push(format!("id={}", encode(id)));
            }
            Command::UpdateArea(id) => {
                params.push(format!("id={}", encode(id)));
            }
            _ => {}
        }

        // 添加其他参数
        for (k, v) in self.params {
            params.push(format!("{}={}", encode(&k), encode(&v)));
        }

        if !params.is_empty() {
            url.push('?');
            url.push_str(&params.join("&"));
        }

        url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_url() {
        let url = ThingsUrl::new(Command::Add)
            .param("title", "Buy milk")
            .build();
        assert_eq!(url, "things:///add?title=Buy%20milk");
    }

    #[test]
    fn test_multiple_params() {
        let url = ThingsUrl::new(Command::Add)
            .param("title", "Buy milk")
            .param("when", "today")
            .param("tags", "Errand,Shopping")
            .build();
        assert!(url.contains("title=Buy%20milk"));
        assert!(url.contains("when=today"));
        assert!(url.contains("tags=Errand%2CShopping"));
    }

    #[test]
    fn test_optional_params() {
        let url = ThingsUrl::new(Command::Add)
            .param("title", "Test")
            .param_opt("notes", Some("Some notes"))
            .param_opt("deadline", None::<&str>)
            .build();
        assert!(url.contains("notes=Some%20notes"));
        assert!(!url.contains("deadline"));
    }

    #[test]
    fn test_with_auth() {
        let url = ThingsUrl::new(Command::Update("abc123".to_string()))
            .param("title", "Updated")
            .build_with_auth("my-secret-token");
        assert!(url.starts_with("things:///update?"));
        assert!(url.contains("auth-token=my-secret-token"));
        assert!(url.contains("id=abc123"));
    }

    #[test]
    fn test_multiline() {
        let items = vec!["Task 1".to_string(), "Task 2".to_string()];
        let url = ThingsUrl::new(Command::Add)
            .param_multiline("titles", &items)
            .build();
        assert!(url.contains("titles=Task%201%0ATask%202"));
    }
}
