use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;

/// 任务状态
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum TaskStatus {
    Inbox,
    Today,
    Evening,
    Anytime,
    Upcoming,
    Someday,
    Completed,
    Canceled,
    Trashed,
}

impl TaskStatus {
    pub fn from_start_and_status(start: i64, status: i64, trashed: bool) -> Self {
        if trashed {
            return Self::Trashed;
        }
        if status == 3 {
            return Self::Completed;
        }
        if status == 2 {
            return Self::Canceled;
        }

        match start {
            0 => Self::Inbox,
            1 => Self::Today,
            2 => Self::Evening,
            3 => Self::Anytime,
            4 => Self::Upcoming,
            5 => Self::Someday,
            _ => Self::Inbox,
        }
    }

    #[allow(dead_code)]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Inbox => "Inbox",
            Self::Today => "Today",
            Self::Evening => "This Evening",
            Self::Anytime => "Anytime",
            Self::Upcoming => "Upcoming",
            Self::Someday => "Someday",
            Self::Completed => "Completed",
            Self::Canceled => "Canceled",
            Self::Trashed => "Trashed",
        }
    }
}

/// 任务
#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)]
pub struct Task {
    pub uuid: String,
    pub title: String,
    pub notes: Option<String>,
    pub status: TaskStatus,
    pub project: Option<String>,
    pub area: Option<String>,
    pub tags: Vec<String>,
    pub deadline: Option<NaiveDate>,
    pub creation_date: Option<NaiveDateTime>,
    pub completion_date: Option<NaiveDateTime>,
    pub index: i64,
}

/// 项目
#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)]
pub struct Project {
    pub uuid: String,
    pub title: String,
    pub notes: Option<String>,
    pub area: Option<String>,
    pub tags: Vec<String>,
    pub deadline: Option<NaiveDate>,
    pub creation_date: Option<NaiveDateTime>,
    pub completion_date: Option<NaiveDateTime>,
}

/// 区域
#[derive(Debug, Clone, Serialize)]
#[allow(dead_code)]
pub struct Area {
    pub uuid: String,
    pub title: String,
    pub index: i64,
}

/// 标签
#[derive(Debug, Clone, Serialize)]
pub struct Tag {
    pub uuid: String,
    pub title: String,
    pub shortcut: Option<String>,
}

/// 清单项目
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ChecklistItem {
    pub uuid: String,
    pub title: String,
    pub status: i64,  // 0 = 未完成, 1 = 完成, 2 = 取消
    pub index: i64,
}

/// 任务筛选器
#[derive(Debug, Clone, Default)]
#[allow(dead_code)]
pub struct TaskFilter {
    pub status: Option<TaskStatus>,
    pub project: Option<String>,
    pub area: Option<String>,
    pub tag: Option<String>,
    pub search: Option<String>,
    pub include_trashed: bool,
}

impl TaskFilter {
    pub fn with_status(status: TaskStatus) -> Self {
        Self {
            status: Some(status),
            ..Default::default()
        }
    }

    pub fn inbox() -> Self {
        Self::with_status(TaskStatus::Inbox)
    }

    pub fn today() -> Self {
        Self::with_status(TaskStatus::Today)
    }

    pub fn upcoming() -> Self {
        Self::with_status(TaskStatus::Upcoming)
    }

    pub fn someday() -> Self {
        Self::with_status(TaskStatus::Someday)
    }

    pub fn completed() -> Self {
        Self::with_status(TaskStatus::Completed)
    }

    pub fn canceled() -> Self {
        Self::with_status(TaskStatus::Canceled)
    }
}
