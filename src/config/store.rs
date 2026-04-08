use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::config_file_path;

/// 应用配置（非敏感信息）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// 默认列表/区域
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_list: Option<String>,

    /// 默认标签
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub default_tags: Vec<String>,

    /// 是否启用调试模式
    #[serde(default)]
    pub debug: bool,
}

/// 配置存储 trait
pub trait ConfigStore {
    /// 加载配置
    fn load_config(&self) -> anyhow::Result<Config>;

    /// 保存配置
    fn save_config(&self, config: &Config) -> anyhow::Result<()>;

    /// 获取 auth-token（从环境变量）
    fn get_auth_token(&self) -> anyhow::Result<Option<String>>;

    /// 检查是否已配置 auth-token
    fn has_auth_token(&self) -> anyhow::Result<bool> {
        Ok(self.get_auth_token()?.is_some())
    }
}

/// 环境变量名（用于 auth-token）
pub const AUTH_TOKEN_ENV_VAR: &str = "THINGS_AUTH_TOKEN";

/// 文件存储（用于配置）
pub struct FileStore {
    path: PathBuf,
}

impl FileStore {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            path: config_file_path()?,
        })
    }

    pub fn load(&self) -> anyhow::Result<Config> {
        if !self.path.exists() {
            return Ok(Config::default());
        }

        let content = std::fs::read_to_string(&self.path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn save(&self, config: &Config) -> anyhow::Result<()> {
        let content = toml::to_string_pretty(config)?;
        std::fs::write(&self.path, content)?;
        Ok(())
    }
}

impl ConfigStore for FileStore {
    fn load_config(&self) -> anyhow::Result<Config> {
        self.load()
    }

    fn save_config(&self, config: &Config) -> anyhow::Result<()> {
        self.save(config)
    }

    fn get_auth_token(&self) -> anyhow::Result<Option<String>> {
        match std::env::var(AUTH_TOKEN_ENV_VAR) {
            Ok(token) if !token.is_empty() => Ok(Some(token)),
            _ => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_config_serialization() {
        let config = Config {
            default_list: Some("Work".to_string()),
            default_tags: vec!["urgent".to_string(), "important".to_string()],
            debug: false,
        };

        let toml_str = toml::to_string_pretty(&config).unwrap();
        let parsed: Config = toml::from_str(&toml_str).unwrap();

        assert_eq!(config.default_list, parsed.default_list);
        assert_eq!(config.default_tags, parsed.default_tags);
    }

    #[test]
    fn test_file_store() {
        let temp_dir = TempDir::new().unwrap();
        let store = FileStore {
            path: temp_dir.path().join("config.toml"),
        };

        // 加载不存在的配置应返回默认
        let config = store.load().unwrap();
        assert!(config.default_list.is_none());

        // 保存配置
        let new_config = Config {
            default_list: Some("Personal".to_string()),
            default_tags: vec!["home".to_string()],
            debug: true,
        };
        store.save(&new_config).unwrap();

        // 重新加载
        let loaded = store.load().unwrap();
        assert_eq!(loaded.default_list, Some("Personal".to_string()));
        assert_eq!(loaded.default_tags, vec!["home".to_string()]);
    }
}
