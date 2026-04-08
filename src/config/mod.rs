use std::path::PathBuf;

pub mod store;

pub use store::Config;

/// 获取配置目录
pub fn config_dir() -> anyhow::Result<PathBuf> {
    let dir = dirs::config_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?
        .join("things-cli");

    if !dir.exists() {
        std::fs::create_dir_all(&dir)?;
    }

    Ok(dir)
}

/// 获取配置文件路径
pub fn config_file_path() -> anyhow::Result<PathBuf> {
    Ok(config_dir()?.join("config.toml"))
}
