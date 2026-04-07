use std::path::Path;

use rusqlite::{Connection, OptionalExtension, Result as SqliteResult};
use chrono::{NaiveDate, NaiveDateTime, TimeZone, Utc};

use super::models::{Area, Project, Tag, Task, TaskFilter, TaskStatus};

/// 将 Unix 时间戳 (REAL) 转换为 NaiveDateTime
fn timestamp_to_datetime(ts: f64) -> Option<NaiveDateTime> {
    let secs = ts as i64;
    let nanos = ((ts - secs as f64) * 1_000_000_000.0) as u32;
    Utc.timestamp_opt(secs, nanos).single().map(|dt| dt.naive_local())
}

/// 将 Unix 时间戳 (INTEGER) 转换为 NaiveDate
fn timestamp_to_date(ts: i64) -> Option<NaiveDate> {
    // Things 使用自 2001-01-01 以来的秒数 (Apple 的参考日期)
    // 或者可能是标准的 Unix 时间戳
    // 这里假设是标准 Unix 时间戳
    let datetime = Utc.timestamp_opt(ts, 0).single()?;
    Some(datetime.date_naive())
}

/// Things 数据库连接
pub struct ThingsDb {
    conn: Connection,
}

impl ThingsDb {
    /// 打开数据库（只读模式）
    pub fn open(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let conn = Connection::open_with_flags(
            path,
            rusqlite::OpenFlags::SQLITE_OPEN_READ_ONLY,
        )?;
        Ok(Self { conn })
    }

    /// 打开默认数据库
    pub fn open_default() -> anyhow::Result<Self> {
        let path = super::find_database_path()?;
        Self::open(path)
    }

    /// 获取所有任务
    pub fn get_tasks(&self, filter: Option<TaskFilter>) -> anyhow::Result<Vec<Task>> {
        let filter = filter.unwrap_or_default();

        let mut sql = String::from(
            "SELECT
                t.\"uuid\", t.\"title\", t.\"notes\", t.\"start\", t.\"status\", t.\"trashed\",
                t.\"project\", t.\"area\", t.\"deadline\", t.\"creationDate\", t.\"stopDate\", t.\"index\"
             FROM TMTask t
             WHERE t.\"type\" = 0"
        );

        // 状态筛选
        if !filter.include_trashed {
            sql.push_str(" AND t.\"trashed\" = 0");
        }

        if let Some(status) = filter.status {
            match status {
                TaskStatus::Inbox => sql.push_str(" AND t.\"start\" = 0 AND t.\"status\" = 0"),
                TaskStatus::Today => sql.push_str(" AND (t.\"start\" = 1 OR t.\"start\" = 2) AND t.\"status\" = 0"),
                TaskStatus::Evening => sql.push_str(" AND t.\"start\" = 2 AND t.\"status\" = 0"),
                TaskStatus::Anytime => sql.push_str(" AND t.\"start\" = 3 AND t.\"status\" = 0"),
                TaskStatus::Upcoming => sql.push_str(" AND t.\"start\" = 4 AND t.\"status\" = 0"),
                TaskStatus::Someday => sql.push_str(" AND t.\"start\" = 5 AND t.\"status\" = 0"),
                TaskStatus::Completed => sql.push_str(" AND t.\"status\" = 3"),
                TaskStatus::Canceled => sql.push_str(" AND t.\"status\" = 2"),
                TaskStatus::Trashed => sql.push_str(" AND t.\"trashed\" = 1"),
            }
        }

        sql.push_str(" ORDER BY t.\"index\"");

        let mut stmt = self.conn.prepare(&sql)?;

        let task_iter = stmt.query_map([], |row| {
            let start: i64 = row.get(3)?;
            let status: i64 = row.get(4)?;
            let trashed: i64 = row.get(5)?;
            let deadline_ts: Option<i64> = row.get(8)?;
            let creation_ts: Option<f64> = row.get(9)?;
            let stop_ts: Option<f64> = row.get(10)?;

            Ok(Task {
                uuid: row.get(0)?,
                title: row.get(1)?,
                notes: row.get(2)?,
                status: TaskStatus::from_start_and_status(start, status, trashed != 0),
                project: row.get(6)?,
                area: row.get(7)?,
                tags: vec![],
                deadline: deadline_ts.and_then(timestamp_to_date),
                creation_date: creation_ts.and_then(timestamp_to_datetime),
                completion_date: stop_ts.and_then(timestamp_to_datetime),
                index: row.get(11)?,
            })
        })?;

        let mut result = vec![];
        for task in task_iter {
            result.push(task?);
        }

        // 加载标签
        for task in &mut result {
            task.tags = self.get_task_tags(&task.uuid)?;
        }

        Ok(result)
    }

    /// 获取单个任务
    #[allow(dead_code)]
    pub fn get_task(&self, uuid: &str) -> anyhow::Result<Option<Task>> {
        let tasks = self.get_tasks(None)?;
        Ok(tasks.into_iter().find(|t| t.uuid == uuid))
    }

    /// 获取任务的标签
    fn get_task_tags(&self, task_uuid: &str) -> SqliteResult<Vec<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT t.\"title\" FROM \"TMTag\" t
             JOIN \"TMTaskTag\" tt ON t.\"uuid\" = tt.\"tags\"
             WHERE tt.\"tasks\" = ?
             ORDER BY t.\"index\""
        )?;

        let tags: rusqlite::Result<Vec<String>> = stmt
            .query_map([task_uuid], |row| row.get(0))?
            .collect();
        tags
    }

    /// 获取所有项目
    pub fn get_projects(&self, _area_uuid: Option<&str>) -> anyhow::Result<Vec<Project>> {
        let mut stmt = self.conn.prepare(
            "SELECT
                \"uuid\", \"title\", \"notes\", \"area\", \"deadline\", \"creationDate\", \"stopDate\"
             FROM \"TMTask\"
             WHERE \"trashed\" = 0 AND \"type\" = 1
             ORDER BY \"index\""
        )?;

        let project_iter = stmt.query_map([], |row| {
            let deadline_ts: Option<i64> = row.get(4)?;
            let creation_ts: Option<f64> = row.get(5)?;
            let stop_ts: Option<f64> = row.get(6)?;

            Ok(Project {
                uuid: row.get(0)?,
                title: row.get(1)?,
                notes: row.get(2)?,
                area: row.get(3)?,
                tags: vec![],
                deadline: deadline_ts.and_then(timestamp_to_date),
                creation_date: creation_ts.and_then(timestamp_to_datetime),
                completion_date: stop_ts.and_then(timestamp_to_datetime),
            })
        })?;

        let mut result = vec![];
        for project in project_iter {
            result.push(project?);
        }

        // 加载标签
        for project in &mut result {
            project.tags = self.get_project_tags(&project.uuid)?;
        }

        Ok(result)
    }

    /// 获取项目的标签
    fn get_project_tags(&self, project_uuid: &str) -> SqliteResult<Vec<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT t.\"title\" FROM \"TMTag\" t
             JOIN \"TMTaskTag\" pt ON t.\"uuid\" = pt.\"tags\"
             WHERE pt.\"tasks\" = ?
             ORDER BY t.\"index\""
        )?;

        let tags: rusqlite::Result<Vec<String>> = stmt
            .query_map([project_uuid], |row| row.get(0))?
            .collect();
        tags
    }

    /// 获取所有区域
    pub fn get_areas(&self) -> anyhow::Result<Vec<Area>> {
        let mut stmt = self.conn.prepare(
            "SELECT \"uuid\", \"title\", \"index\" FROM \"TMArea\"
             WHERE \"trashed\" = 0
             ORDER BY \"index\""
        )?;

        let areas: rusqlite::Result<Vec<Area>> = stmt
            .query_map([], |row| {
                Ok(Area {
                    uuid: row.get(0)?,
                    title: row.get(1)?,
                    index: row.get(2)?,
                })
            })?
            .collect();

        areas.map_err(Into::into)
    }

    /// 获取单个区域
    pub fn get_area_by_id(&self, id: &str) -> anyhow::Result<Option<Area>> {
        let mut stmt = self.conn.prepare(
            "SELECT \"uuid\", \"title\", \"index\" FROM \"TMArea\"
             WHERE \"uuid\" = ? AND \"trashed\" = 0"
        )?;

        let area = stmt
            .query_row([id], |row| {
                Ok(Area {
                    uuid: row.get(0)?,
                    title: row.get(1)?,
                    index: row.get(2)?,
                })
            })
            .optional()?;

        Ok(area)
    }

    /// 获取所有标签
    pub fn get_tags(&self) -> anyhow::Result<Vec<Tag>> {
        let mut stmt = self.conn.prepare(
            "SELECT \"uuid\", \"title\", \"shortcut\" FROM \"TMTag\"
             ORDER BY \"index\""
        )?;

        let tags: rusqlite::Result<Vec<Tag>> = stmt
            .query_map([], |row| {
                Ok(Tag {
                    uuid: row.get(0)?,
                    title: row.get(1)?,
                    shortcut: row.get(2)?,
                })
            })?
            .collect();

        tags.map_err(Into::into)
    }

    /// 通过标题查找任务 ID
    #[allow(dead_code)]
    pub fn find_task_by_title(
        &self, title: &str) -> anyhow::Result<Option<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT \"uuid\" FROM \"TMTask\"
             WHERE \"title\" = ? AND \"trashed\" = 0
             ORDER BY \"creationDate\" DESC
             LIMIT 1"
        )?;

        let result: Option<String> = stmt.query_row([title], |row| row.get(0)).ok();
        Ok(result)
    }

    /// 通过标题查找项目 ID
    #[allow(dead_code)]
    pub fn find_project_by_title(
        &self, title: &str) -> anyhow::Result<Option<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT \"uuid\" FROM \"TMTask\"
             WHERE \"title\" = ? AND \"trashed\" = 0 AND \"type\" = 1
             ORDER BY \"creationDate\" DESC
             LIMIT 1"
        )?;

        let result: Option<String> = stmt.query_row([title], |row| row.get(0)).ok();
        Ok(result)
    }

    /// 通过 ID 获取项目
    pub fn get_project_by_id(&self, id: &str) -> anyhow::Result<Option<Project>> {
        let mut stmt = self.conn.prepare(
            "SELECT
                \"uuid\", \"title\", \"notes\", \"area\", \"deadline\", \"creationDate\", \"stopDate\"
             FROM \"TMTask\"
             WHERE \"uuid\" = ? AND \"trashed\" = 0 AND \"type\" = 1"
        )?;

        let project = stmt
            .query_row([id], |row| {
                let deadline_ts: Option<i64> = row.get(4)?;
                let creation_ts: Option<f64> = row.get(5)?;
                let stop_ts: Option<f64> = row.get(6)?;

                Ok(Project {
                    uuid: row.get(0)?,
                    title: row.get(1)?,
                    notes: row.get(2)?,
                    area: row.get(3)?,
                    tags: vec![],
                    deadline: deadline_ts.and_then(timestamp_to_date),
                    creation_date: creation_ts.and_then(timestamp_to_datetime),
                    completion_date: stop_ts.and_then(timestamp_to_datetime),
                })
            })
            .optional()?;

        // 加载标签
        if let Some(mut project) = project {
            project.tags = self.get_project_tags(&project.uuid)?;
            Ok(Some(project))
        } else {
            Ok(None)
        }
    }

    /// 通过标题搜索项目
    pub fn search_projects_by_title(&self, query: &str) -> anyhow::Result<Vec<Project>> {
        let mut stmt = self.conn.prepare(
            "SELECT
                \"uuid\", \"title\", \"notes\", \"area\", \"deadline\", \"creationDate\", \"stopDate\"
             FROM \"TMTask\"
             WHERE \"trashed\" = 0 AND \"type\" = 1 AND \"title\" LIKE ?
             ORDER BY \"index\""
        )?;

        let search_pattern = format!("%{}%", query);

        let project_iter = stmt.query_map([&search_pattern], |row| {
            let deadline_ts: Option<i64> = row.get(4)?;
            let creation_ts: Option<f64> = row.get(5)?;
            let stop_ts: Option<f64> = row.get(6)?;

            Ok(Project {
                uuid: row.get(0)?,
                title: row.get(1)?,
                notes: row.get(2)?,
                area: row.get(3)?,
                tags: vec![],
                deadline: deadline_ts.and_then(timestamp_to_date),
                creation_date: creation_ts.and_then(timestamp_to_datetime),
                completion_date: stop_ts.and_then(timestamp_to_datetime),
            })
        })?;

        let mut result = vec![];
        for project in project_iter {
            result.push(project?);
        }

        // 加载标签
        for project in &mut result {
            project.tags = self.get_project_tags(&project.uuid)?;
        }

        Ok(result)
    }

    /// 通过标题关键词搜索任务
    pub fn search_tasks_by_title(&self, query: &str) -> anyhow::Result<Vec<Task>> {
        let like_pattern = format!("%{}%", query);

        let sql = String::from(
            "SELECT
                t.\"uuid\", t.\"title\", t.\"notes\", t.\"start\", t.\"status\", t.\"trashed\",
                t.\"project\", t.\"area\", t.\"deadline\", t.\"creationDate\", t.\"stopDate\", t.\"index\"
             FROM TMTask t
             WHERE t.\"type\" = 0 AND t.\"trashed\" = 0 AND t.\"title\" LIKE ?
             ORDER BY t.\"creationDate\" DESC"
        );

        let mut stmt = self.conn.prepare(&sql)?;

        let task_iter = stmt.query_map([&like_pattern], |row| {
            let start: i64 = row.get(3)?;
            let status: i64 = row.get(4)?;
            let trashed: i64 = row.get(5)?;
            let deadline_ts: Option<i64> = row.get(8)?;
            let creation_ts: Option<f64> = row.get(9)?;
            let stop_ts: Option<f64> = row.get(10)?;

            Ok(Task {
                uuid: row.get(0)?,
                title: row.get(1)?,
                notes: row.get(2)?,
                status: TaskStatus::from_start_and_status(start, status, trashed != 0),
                project: row.get(6)?,
                area: row.get(7)?,
                tags: vec![],
                deadline: deadline_ts.and_then(timestamp_to_date),
                creation_date: creation_ts.and_then(timestamp_to_datetime),
                completion_date: stop_ts.and_then(timestamp_to_datetime),
                index: row.get(11)?,
            })
        })?;

        let mut result = vec![];
        for task in task_iter {
            result.push(task?);
        }

        // 加载标签
        for task in &mut result {
            task.tags = self.get_task_tags(&task.uuid)?;
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_database_path() {
        // 这个测试只有在 Things 数据库存在时才会通过
        if let Ok(path) = crate::db::find_database_path() {
            assert!(path.exists());
        }
    }
}
