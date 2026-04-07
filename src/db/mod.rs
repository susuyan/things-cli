use std::path::PathBuf;

pub mod models;
pub mod queries;
pub mod store;

pub use models::{Area, Project, Tag, Task, TaskStatus};
pub use store::ThingsDb;

/// 查找 Things 数据库路径
pub fn find_database_path() -> anyhow::Result<PathBuf> {
    let home = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;

    // 主要路径: Group Containers
    let pattern = home.join(
        "Library/Group Containers/JLMPQHK86H.com.culturedcode.ThingsMac/ThingsData-*/Things Database.thingsdatabase/main.sqlite"
    );

    if let Ok(paths) = glob::glob(&pattern.to_string_lossy()) {
        let mut matches: Vec<_> = paths
            .filter_map(|p| p.ok())
            .filter(|p| p.exists())
            .collect();

        if !matches.is_empty() {
            // 按修改时间排序，返回最新的
            matches.sort_by(|a, b| {
                let a_time = std::fs::metadata(a)
                    .ok()
                    .and_then(|m| m.modified().ok());
                let b_time = std::fs::metadata(b)
                    .ok()
                    .and_then(|m| m.modified().ok());
                match (a_time, b_time) {
                    (Some(a), Some(b)) => b.cmp(&a),
                    _ => std::cmp::Ordering::Equal,
                }
            });
            return Ok(matches[0].clone());
        }
    }

    // 旧版本路径
    let legacy = home.join(
        "Library/Group Containers/JLMPQHK86H.com.culturedcode.ThingsMac/Things Database.thingsdatabase/main.sqlite"
    );
    if legacy.exists() {
        return Ok(legacy);
    }

    Err(anyhow::anyhow!("Things database not found. Make sure Things 3 is installed."))
}

/// 检查数据库是否可访问
pub fn check_database_access() -> anyhow::Result<()> {
    let path = find_database_path()?;
    match std::fs::metadata(&path) {
        Ok(_) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => {
            Err(anyhow::anyhow!(
                "Permission denied. Please grant Full Disk Access to your terminal:\n\
                 1. Open System Settings -> Privacy & Security -> Full Disk Access\n\
                 2. Add your terminal application (e.g., Terminal, iTerm, Cursor)"
            ))
        }
        Err(e) => Err(e.into()),
    }
}
