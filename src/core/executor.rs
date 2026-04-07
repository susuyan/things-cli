use std::process::Command as ProcessCommand;

use super::ThingsError;

/// 执行结果
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub success: bool,
    /// 返回的 ID（如果适用）
    #[allow(dead_code)]
    pub x_things_id: Option<String>,
}

/// 执行器 trait
pub trait Executor {
    /// 执行 URL
    fn execute(&self, url: &str) -> anyhow::Result<ExecutionResult>;
}

/// 使用系统 `open` 命令的执行器
#[derive(Debug, Clone)]
pub struct OpenExecutor;

impl OpenExecutor {
    pub fn new() -> Self {
        Self
    }
}

impl Default for OpenExecutor {
    fn default() -> Self {
        Self::new()
    }
}

impl Executor for OpenExecutor {
    fn execute(&self, url: &str) -> anyhow::Result<ExecutionResult> {
        #[cfg(target_os = "macos")]
        {
            let output = ProcessCommand::new("open")
                .arg(url)
                .output()?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                // 检查是否是 "Things" 应用不存在
                if stderr.contains("Things") || stderr.contains("does not exist") {
                    return Err(ThingsError::AppNotFound.into());
                }
                return Err(ThingsError::CommandFailed(stderr.to_string()).into());
            }
        }

        #[cfg(not(target_os = "macos"))]
        {
            // 非 macOS 平台暂不支持
            return Err(anyhow::anyhow!(
                "Things 3 is only available on macOS and iOS."
            ));
        }

        // TODO: 未来支持 x-callback-url 时，可以解析返回的 ID
        Ok(ExecutionResult {
            success: true,
            x_things_id: None,
        })
    }
}

/// 模拟执行器（用于测试）
#[derive(Debug, Default)]
#[allow(dead_code)]
pub struct MockExecutor {
    last_url: std::cell::RefCell<Option<String>>,
}

#[allow(dead_code)]
impl MockExecutor {
    pub fn new() -> Self {
        Self {
            last_url: std::cell::RefCell::new(None),
        }
    }

    pub fn last_url(&self) -> Option<String> {
        self.last_url.borrow().clone()
    }
}

impl Executor for MockExecutor {
    fn execute(&self, url: &str) -> anyhow::Result<ExecutionResult> {
        *self.last_url.borrow_mut() = Some(url.to_string());
        Ok(ExecutionResult {
            success: true,
            x_things_id: Some("mock-id-123".to_string()),
        })
    }
}

/// 检查 Things 应用是否已安装
#[cfg(target_os = "macos")]
#[allow(dead_code)]
pub fn check_things_installed() -> bool {
    use std::path::Path;

    // 检查常见的 Things 安装位置
    let paths = [
        "/Applications/Things3.app",
        "/Applications/Things.app",
        "~/Applications/Things3.app",
        "~/Applications/Things.app",
    ];

    for path in &paths {
        let expanded = shellexpand::tilde(path);
        if Path::new(&*expanded).exists() {
            return true;
        }
    }

    // 通过 mdfind 搜索
    if let Ok(output) = ProcessCommand::new("mdfind")
        .arg("kMDItemCFBundleIdentifier == 'com.culturedcode.ThingsMac'")
        .output()
    {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if !stdout.trim().is_empty() {
            return true;
        }
    }

    false
}

#[cfg(not(target_os = "macos"))]
pub fn check_things_installed() -> bool {
    false
}

// 添加 shellexpand 依赖用于处理 ~
// 在实际 Cargo.toml 中添加: shellexpand = "3.1"

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_executor() {
        let executor = MockExecutor::new();
        let result = executor.execute("things:///add?title=Test").unwrap();

        assert!(result.success);
        assert_eq!(result.x_things_id, Some("mock-id-123".to_string()));
        assert_eq!(executor.last_url(), Some("things:///add?title=Test".to_string()));
    }
}
