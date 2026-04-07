//! 常用数据库查询

use super::models::{Task, TaskFilter, TaskStatus};
use super::ThingsDb;

impl ThingsDb {
    /// 获取收件箱任务
    pub fn get_inbox_tasks(&self) -> anyhow::Result<Vec<Task>> {
        self.get_tasks(Some(TaskFilter::inbox()))
    }

    /// 获取今日任务（包括今晚）
    pub fn get_today_tasks(&self) -> anyhow::Result<Vec<Task>> {
        self.get_tasks(Some(TaskFilter::today()))
    }

    /// 获取今晚任务
    pub fn get_evening_tasks(&self) -> anyhow::Result<Vec<Task>> {
        self.get_tasks(Some(TaskFilter::with_status(TaskStatus::Evening)))
    }

    /// 获取待办任务
    pub fn get_upcoming_tasks(&self) -> anyhow::Result<Vec<Task>> {
        self.get_tasks(Some(TaskFilter::upcoming()))
    }

    /// 获取某天任务
    pub fn get_someday_tasks(&self) -> anyhow::Result<Vec<Task>> {
        self.get_tasks(Some(TaskFilter::someday()))
    }

    /// 获取任意时间任务
    pub fn get_anytime_tasks(&self) -> anyhow::Result<Vec<Task>> {
        self.get_tasks(Some(TaskFilter::with_status(TaskStatus::Anytime)))
    }

    /// 获取已完成任务
    pub fn get_completed_tasks(&self) -> anyhow::Result<Vec<Task>> {
        self.get_tasks(Some(TaskFilter::completed()))
    }

    /// 获取已取消任务
    pub fn get_canceled_tasks(&self) -> anyhow::Result<Vec<Task>> {
        self.get_tasks(Some(TaskFilter::canceled()))
    }

    /// 获取今日完成的任务
    pub fn get_completed_today(&self) -> anyhow::Result<Vec<Task>> {
        let mut tasks = self.get_completed_tasks()?;
        let today = chrono::Local::now().date_naive();

        tasks.retain(|t| {
            t.completion_date
                .map(|d| d.date() == today)
                .unwrap_or(false)
        });

        Ok(tasks)
    }

    /// 获取今日创建的任务
    #[allow(dead_code)]
    pub fn get_created_today(&self) -> anyhow::Result<Vec<Task>> {
        let mut tasks = self.get_tasks(None)?;
        let today = chrono::Local::now().date_naive();

        tasks.retain(|t| {
            t.creation_date
                .map(|d| d.date() == today)
                .unwrap_or(false)
        });

        Ok(tasks)
    }

    /// 获取带截止日期的任务
    pub fn get_tasks_with_deadlines(&self) -> anyhow::Result<Vec<Task>> {
        let mut tasks = self.get_tasks(Some(TaskFilter {
            include_trashed: false,
            ..Default::default()
        }))?;

        tasks.retain(|t| t.deadline.is_some());
        tasks.sort_by(|a, b| a.deadline.cmp(&b.deadline));

        Ok(tasks)
    }
}
