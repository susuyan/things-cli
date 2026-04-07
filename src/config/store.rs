use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use super::{config_file_path, KEYCHAIN_ACCOUNT, KEYCHAIN_SERVICE};

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

    /// 设置 auth-token（存储到 keychain）
    fn set_auth_token(&self, token: &str) -> anyhow::Result<()>;

    /// 获取 auth-token（从 keychain）
    fn get_auth_token(&self) -> anyhow::Result<Option<String>>;

    /// 删除 auth-token
    fn delete_auth_token(&self) -> anyhow::Result<()>;

    /// 检查是否已配置 auth-token
    fn has_auth_token(&self) -> anyhow::Result<bool> {
        Ok(self.get_auth_token()?.is_some())
    }
}

/// 组合存储：敏感信息存 keychain，其他存文件
pub struct CompositeStore {
    file: FileStore,
    keychain: KeychainStore,
}

impl CompositeStore {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            file: FileStore::new()?,
            keychain: KeychainStore::new(),
        })
    }
}

/// 环境变量名（用于 auth-token）
pub const AUTH_TOKEN_ENV_VAR: &str = "THINGS_AUTH_TOKEN";

impl ConfigStore for CompositeStore {
    fn load_config(&self) -> anyhow::Result<Config> {
        self.file.load()
    }

    fn save_config(&self, config: &Config) -> anyhow::Result<()> {
        self.file.save(config)
    }

    fn set_auth_token(&self, token: &str) -> anyhow::Result<()> {
        eprintln!(
            "Note: Auth token is now stored in environment variable '{}'.",
            AUTH_TOKEN_ENV_VAR
        );
        eprintln!("Please set it in your shell profile or .env file:");
        eprintln!("  export {}='{}'", AUTH_TOKEN_ENV_VAR, token);
        eprintln!();
        eprintln!("Falling back to keychain storage for backward compatibility.");
        self.keychain.set(token)
    }

    fn get_auth_token(&self) -> anyhow::Result<Option<String>> {
        // 1. 优先从环境变量读取
        if let Ok(token) = std::env::var(AUTH_TOKEN_ENV_VAR) {
            if !token.is_empty() {
                return Ok(Some(token));
            }
        }

        // 2. 后备：从 keychain 读取
        self.keychain.get()
    }

    fn delete_auth_token(&self) -> anyhow::Result<()> {
        // 检查环境变量是否存在
        if std::env::var(AUTH_TOKEN_ENV_VAR).is_ok() {
            eprintln!(
                "Note: Auth token is set via environment variable '{}'.",
                AUTH_TOKEN_ENV_VAR
            );
            eprintln!("Please unset it from your shell profile or .env file:");
            eprintln!("  unset {}", AUTH_TOKEN_ENV_VAR);
            eprintln!();
        }

        // 同时删除 keychain 中的 token
        self.keychain.delete()
    }
}

/// 文件存储（用于非敏感配置）
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

/// Keychain 存储（用于 auth-token）
pub struct KeychainStore {
    entry: keyring::Entry,
}

impl KeychainStore {
    pub fn new() -> Self {
        Self {
            entry: keyring::Entry::new(KEYCHAIN_SERVICE, KEYCHAIN_ACCOUNT)
                .expect("Failed to create keychain entry"),
        }
    }

    pub fn set(&self, token: &str) -> anyhow::Result<()> {
        self.entry.set_password(token)?;
        Ok(())
    }

    pub fn get(&self) -> anyhow::Result<Option<String>> {
        match self.entry.get_password() {
            Ok(token) => Ok(Some(token)),
            Err(keyring::Error::NoEntry) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub fn delete(&self) -> anyhow::Result<()> {
        match self.entry.delete_credential() {
            Ok(()) => Ok(()),
            Err(keyring::Error::NoEntry) => Ok(()), // 不存在也算成功
            Err(e) => Err(e.into()),
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
