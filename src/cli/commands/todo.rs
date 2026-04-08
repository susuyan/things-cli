use std::io::{self, BufRead};

use crate::cli::args::TodoCommand;
use crate::cli::GlobalOpts;
use crate::config::store::{FileStore, ConfigStore};
use crate::core::executor::{Executor, OpenExecutor};
use crate::core::parser;
use crate::core::url_builder::{Command, ThingsUrl};

pub fn handle(cmd: TodoCommand, global: &GlobalOpts, json: bool) -> anyhow::Result<()> {
    match cmd {
        TodoCommand::Add {
            titles,
            notes,
            when,
            deadline,
            tags,
            list,
            list_id,
            heading,
            checklist,
            completed,
            canceled,
            show_quick_entry,
            reveal,
            stdin,
            repeat,
            repeat_until,
        } => {
            handle_add(
                titles,
                notes,
                when,
                deadline,
                tags,
                list,
                list_id,
                heading,
                checklist,
                completed,
                canceled,
                show_quick_entry,
                reveal,
                stdin,
                repeat,
                repeat_until,
                global,
                json,
            )?;
        }
        TodoCommand::Update {
            id,
            title,
            notes,
            prepend_notes,
            append_notes,
            when,
            deadline,
            tags,
            add_tags,
            list,
            list_id,
            heading,
            complete,
            uncomplete,
            cancel,
            duplicate,
            reveal,
            repeat,
            repeat_until,
            no_repeat,
        } => {
            handle_update(
                id, title, notes, prepend_notes, append_notes, when, deadline, tags, add_tags,
                list, list_id, heading, complete, uncomplete, cancel, duplicate, reveal,
                repeat, repeat_until, no_repeat, global, json,
            )?;
        }
        TodoCommand::Delete { id, force } => {
            handle_delete(&id, force, json)?;
        }
        TodoCommand::Get { id } => {
            handle_get(&id, global.json)?;
        }
        TodoCommand::Find { query } => {
            handle_find(&query, global.json)?;
        }
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn handle_add(
    titles: Vec<String>,
    notes: Option<String>,
    when: Option<String>,
    deadline: Option<String>,
    tags: Vec<String>,
    list: Option<String>,
    list_id: Option<String>,
    heading: Option<String>,
    checklist: Vec<String>,
    completed: bool,
    canceled: bool,
    show_quick_entry: bool,
    reveal: bool,
    stdin: bool,
    repeat: Option<String>,
    repeat_until: Option<String>,
    global: &GlobalOpts,
    json: bool,
) -> anyhow::Result<()> {
    let store = FileStore::new()?;
    let config = store.load_config()?;

    // 收集标题
    let titles: Vec<String> = if stdin {
        // 从 stdin 读取
        io::stdin()
            .lock()
            .lines()
            .filter_map(|l| l.ok())
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect()
    } else {
        titles
    };

    if titles.is_empty() {
        return Err(anyhow::anyhow!("No titles provided"));
    }

    // 解析参数
    let when_param = when
        .as_deref()
        .map(parser::parse_when)
        .transpose()?
        .map(|w| w.to_url_param());

    let deadline_param = deadline;

    // 合并标签（命令行 + 配置默认）
    let mut all_tags = config.default_tags.clone();
    all_tags.extend(tags);
    let tags_str = if all_tags.is_empty() {
        None
    } else {
        Some(all_tags.join(","))
    };

    // 确定列表
    let (final_list, final_list_id) = if list_id.is_some() {
        (None, list_id)
    } else if list.is_some() {
        (list, None)
    } else if config.default_list.is_some() {
        (config.default_list, None)
    } else {
        (None, None)
    };

    // 构建 URL
    let url = if titles.len() == 1 {
        // 单条添加
        let mut url = ThingsUrl::new(Command::Add)
            .param("title", &titles[0])
            .param_opt("notes", notes.as_deref())
            .param_opt("when", when_param.as_deref())
            .param_opt("deadline", deadline_param.as_deref())
            .param_opt("tags", tags_str.as_deref())
            .param_opt("list", final_list.as_deref())
            .param_opt("list-id", final_list_id.as_deref())
            .param_opt("heading", heading.as_deref());

        // 添加 checklist（逗号分隔）
        if !checklist.is_empty() {
            let checklist_str = checklist.join("\n");
            url = url.param("checklist-items", &checklist_str);
        }

        // 布尔参数
        if completed {
            url = url.param_bool("completed", true);
        }
        if canceled {
            url = url.param_bool("canceled", true);
        }
        if show_quick_entry {
            url = url.param_bool("show-quick-entry", true);
        }
        if reveal {
            url = url.param_bool("reveal", true);
        }

        // 重复任务
        if let Some(ref repeat_pattern) = repeat {
            if let Some(pattern) = crate::core::models::RepeatPattern::parse(repeat_pattern) {
                url = url.param("repeat", &pattern.to_url_param());
            }
        }
        if let Some(ref until) = repeat_until {
            url = url.param("repeat-until", until);
        }

        url.build()
    } else {
        // 批量添加 - 使用 titles 参数
        let mut url = ThingsUrl::new(Command::Add)
            .param_multiline("titles", &titles)
            .param_opt("notes", notes.as_deref())
            .param_opt("when", when_param.as_deref())
            .param_opt("deadline", deadline_param.as_deref())
            .param_opt("tags", tags_str.as_deref())
            .param_opt("list", final_list.as_deref())
            .param_opt("list-id", final_list_id.as_deref())
            .param_opt("heading", heading.as_deref());

        if completed {
            url = url.param_bool("completed", true);
        }
        if canceled {
            url = url.param_bool("canceled", true);
        }
        if reveal {
            url = url.param_bool("reveal", true);
        }

        // 重复任务（批量不支持重复，但保持一致性）
        if let Some(ref repeat_pattern) = repeat {
            if let Some(pattern) = crate::core::models::RepeatPattern::parse(repeat_pattern) {
                url = url.param("repeat", &pattern.to_url_param());
            }
        }
        if let Some(ref until) = repeat_until {
            url = url.param("repeat-until", until);
        }

        url.build()
    };

    if global.debug {
        eprintln!("URL: {}", url);
    }

    // dry-run 模式：只打印 URL 不执行
    if global.dry_run {
        if json {
            let output = serde_json::json!({
                "success": true,
                "operation": "add",
                "type": "todo",
                "dry_run": true,
                "data": {
                    "titles": titles,
                    "url": url
                }
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        } else {
            eprintln!("[Dry Run] URL: {}", url);
        }
        return Ok(());
    }

    // 执行
    let executor = OpenExecutor::new();
    let result = executor.execute(&url)?;

    if json {
        let output = if result.success {
            serde_json::json!({
                "success": true,
                "operation": "add",
                "type": "todo",
                "data": {
                    "titles": titles,
                    "count": titles.len()
                }
            })
        } else {
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "EXECUTION_FAILED",
                    "message": "Failed to add todo"
                }
            })
        };
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else if result.success {
        if titles.len() == 1 {
            println!("✓ Todo added: {}", titles[0]);
        } else {
            println!("✓ {} todos added", titles.len());
        }
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn handle_update(
    id: String,
    title: Option<String>,
    notes: Option<String>,
    prepend_notes: Option<String>,
    append_notes: Option<String>,
    when: Option<String>,
    deadline: Option<String>,
    tags: Vec<String>,
    add_tags: Vec<String>,
    list: Option<String>,
    list_id: Option<String>,
    heading: Option<String>,
    complete: bool,
    uncomplete: bool,
    cancel: bool,
    duplicate: bool,
    reveal: bool,
    repeat: Option<String>,
    repeat_until: Option<String>,
    no_repeat: bool,
    global: &GlobalOpts,
    json: bool,
) -> anyhow::Result<()> {
    let store = FileStore::new()?;

    // 需要 auth-token
    let auth_token = global
        .auth_token
        .clone()
        .or_else(|| store.get_auth_token().unwrap_or(None))
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Authentication required. Run `things config set-auth-token` first."
            )
        })?;

    // 解析 when
    let when_param = when
        .as_deref()
        .map(parser::parse_when)
        .transpose()?
        .map(|w| w.to_url_param());

    // 构建 URL
    let mut url = ThingsUrl::new(Command::Update(id.clone()))
        .param_opt("title", title.as_deref())
        .param_opt("notes", notes.as_deref())
        .param_opt("prepend-notes", prepend_notes.as_deref())
        .param_opt("append-notes", append_notes.as_deref())
        .param_opt("when", when_param.as_deref())
        .param_opt("list", list.as_deref())
        .param_opt("list-id", list_id.as_deref())
        .param_opt("heading", heading.as_deref());

    // deadline 特殊处理：空字符串表示清除
    if let Some(ref d) = deadline {
        if d.is_empty() {
            url = url.param("deadline", "");
        } else {
            url = url.param("deadline", d);
        }
    }

    // 标签
    if !tags.is_empty() {
        url = url.param("tags", &tags.join(","));
    }
    if !add_tags.is_empty() {
        url = url.param("add-tags", &add_tags.join(","));
    }

    // 状态处理
    if complete {
        url = url.param("completed", "true");
    } else if uncomplete {
        url = url.param("completed", "false");
    } else if cancel {
        url = url.param("canceled", "true");
    }

    // 其他选项
    if duplicate {
        url = url.param_bool("duplicate", true);
    }
    if reveal {
        url = url.param_bool("reveal", true);
    }

    // 重复任务
    if no_repeat {
        url = url.param("repeat", "false");
    } else if let Some(ref repeat_pattern) = repeat {
        if let Some(pattern) = crate::core::models::RepeatPattern::parse(repeat_pattern) {
            url = url.param("repeat", &pattern.to_url_param());
        }
    }
    if let Some(ref until) = repeat_until {
        url = url.param("repeat-until", until);
    }

    let url = url.build_with_auth(&auth_token);

    if global.debug {
        eprintln!("URL: {}", url);
    }

    // dry-run 模式：只打印 URL 不执行
    if global.dry_run {
        if json {
            let output = serde_json::json!({
                "success": true,
                "operation": "update",
                "type": "todo",
                "dry_run": true,
                "data": {
                    "id": id,
                    "url": url
                }
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        } else {
            eprintln!("[Dry Run] URL: {}", url);
        }
        return Ok(());
    }

    // 执行
    let executor = OpenExecutor::new();
    let result = executor.execute(&url)?;

    if json {
        let output = if result.success {
            serde_json::json!({
                "success": true,
                "operation": "update",
                "type": "todo",
                "data": {
                    "id": id
                }
            })
        } else {
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "EXECUTION_FAILED",
                    "message": format!("Failed to update todo: {}", id)
                }
            })
        };
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else if result.success {
        println!("✓ Todo updated: {}", id);
    }

    Ok(())
}

use crate::core::applescript;

fn handle_delete(id: &str, force: bool, json: bool) -> anyhow::Result<()> {
    // 确认删除（非 JSON 模式下）
    if !force && !json {
        let confirmed = dialoguer::Confirm::new()
            .with_prompt(format!("Are you sure you want to delete todo '{}'", id))
            .default(false)
            .interact()?;

        if !confirmed {
            if json {
                let output = serde_json::json!({
                    "success": false,
                    "error": {
                        "code": "CANCELLED",
                        "message": "Deletion cancelled by user"
                    }
                });
                println!("{}", serde_json::to_string_pretty(&output)?);
            } else {
                println!("Cancelled");
            }
            return Ok(());
        }
    }

    // 使用 AppleScript 删除
    match applescript::delete_todo(id) {
        Ok(_) => {
            if json {
                let output = serde_json::json!({
                    "success": true,
                    "operation": "delete",
                    "type": "todo",
                    "data": {
                        "id": id
                    }
                });
                println!("{}", serde_json::to_string_pretty(&output)?);
            } else {
                println!("✓ Todo deleted: {}", id);
            }
        }
        Err(e) => {
            if json {
                let output = serde_json::json!({
                    "success": false,
                    "error": {
                        "code": "DELETE_FAILED",
                        "message": e.to_string()
                    }
                });
                println!("{}", serde_json::to_string_pretty(&output)?);
            } else {
                return Err(e);
            }
        }
    }

    Ok(())
}

use crate::db::ThingsDb;
use colored::Colorize;
use serde::Serialize;

/// JSON 输出结构
#[derive(Serialize)]
struct TaskJsonOutput {
    success: bool,
    #[serde(rename = "type")]
    data_type: String,
    data: crate::db::Task,
}

fn handle_get(id: &str, json: bool) -> anyhow::Result<()> {
    // 检查数据库是否可访问
    if let Err(e) = crate::db::check_database_access() {
        if json {
            let error_output = serde_json::json!({
                "success": false,
                "error": e.to_string()
            });
            println!("{}", serde_json::to_string_pretty(&error_output)?);
        } else {
            eprintln!("{}", e);
        }
        return Ok(());
    }

    let db = ThingsDb::open_default()?;

    // 获取任务
    let task = db.get_task(id)?;

    match task {
        Some(task) => {
            if json {
                let output = TaskJsonOutput {
                    success: true,
                    data_type: "task".to_string(),
                    data: task,
                };
                println!("{}", serde_json::to_string_pretty(&output)?);
            } else {
                print_task_details(&task);
            }
        }
        None => {
            if json {
                let error_output = serde_json::json!({
                    "success": false,
                    "error": format!("Task not found: {}", id)
                });
                println!("{}", serde_json::to_string_pretty(&error_output)?);
            } else {
                eprintln!("{} Task not found: {}", "Error:".red().bold(), id);
            }
        }
    }

    Ok(())
}

fn print_task_details(task: &crate::db::Task) {
    println!("{}", "Task Details".bold().underline());
    println!();

    // ID
    println!("  {}: {}", "ID".bold(), task.uuid);

    // Title
    println!("  {}: {}", "Title".bold(), task.title);

    // Status
    let status_str = match task.status {
        crate::db::TaskStatus::Inbox => "Inbox".normal(),
        crate::db::TaskStatus::Today => "Today".green(),
        crate::db::TaskStatus::Evening => "This Evening".blue(),
        crate::db::TaskStatus::Anytime => "Anytime".normal(),
        crate::db::TaskStatus::Upcoming => "Upcoming".yellow(),
        crate::db::TaskStatus::Someday => "Someday".dimmed(),
        crate::db::TaskStatus::Completed => "Completed".green(),
        crate::db::TaskStatus::Canceled => "Canceled".red(),
        crate::db::TaskStatus::Trashed => "Trashed".red().dimmed(),
    };
    println!("  {}: {}", "Status".bold(), status_str);

    // Notes
    if let Some(ref notes) = task.notes {
        if !notes.is_empty() {
            println!("  {}: {}", "Notes".bold(), notes);
        }
    }

    // Tags
    if !task.tags.is_empty() {
        let tags_str = format!("#{}", task.tags.join(" #"));
        println!("  {}: {}", "Tags".bold(), tags_str.cyan());
    }

    // Deadline
    if let Some(deadline) = task.deadline {
        println!("  {}: {}", "Deadline".bold(), deadline.to_string().yellow());
    }

    // Project
    if let Some(ref project) = task.project {
        println!("  {}: {}", "Project".bold(), project.truecolor(128, 128, 128));
    }

    // Area
    if let Some(ref area) = task.area {
        println!("  {}: {}", "Area".bold(), area.truecolor(128, 128, 128));
    }

    // Creation Date
    if let Some(creation_date) = task.creation_date {
        println!("  {}: {}", "Created".bold(), creation_date.format("%Y-%m-%d %H:%M"));
    }

    // Completion Date
    if let Some(completion_date) = task.completion_date {
        let formatted = completion_date.format("%Y-%m-%d %H:%M").to_string();
        println!("  {}: {}", "Completed".bold(), formatted.green());
    }
}

/// JSON 输出结构（任务列表）
#[derive(Serialize)]
struct TasksJsonOutput {
    success: bool,
    #[serde(rename = "type")]
    data_type: String,
    count: usize,
    data: Vec<crate::db::Task>,
}

fn handle_find(query: &str, json: bool) -> anyhow::Result<()> {
    // 检查数据库是否可访问
    if let Err(e) = crate::db::check_database_access() {
        if json {
            let error_output = serde_json::json!({
                "success": false,
                "error": e.to_string()
            });
            println!("{}", serde_json::to_string_pretty(&error_output)?);
        } else {
            eprintln!("{}", e);
        }
        return Ok(());
    }

    let db = ThingsDb::open_default()?;

    // 搜索任务
    let tasks = db.search_tasks_by_title(query)?;

    if json {
        let output = TasksJsonOutput {
            success: true,
            data_type: "tasks".to_string(),
            count: tasks.len(),
            data: tasks,
        };
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        print_found_tasks(&tasks, query);
    }

    Ok(())
}

fn print_found_tasks(tasks: &[crate::db::Task], query: &str) {
    if tasks.is_empty() {
        println!("No tasks found matching '{}'", query.dimmed());
        return;
    }

    println!("{} {}", "Found".bold(), format!("{} tasks:", tasks.len()).bold());

    for task in tasks {
        let status_icon = match task.status {
            crate::db::TaskStatus::Completed => "✓".green(),
            crate::db::TaskStatus::Canceled => "✗".red(),
            _ => "•".normal(),
        };

        let title = if task.status == crate::db::TaskStatus::Completed
            || task.status == crate::db::TaskStatus::Canceled
        {
            task.title.as_str().dimmed().to_string()
        } else {
            task.title.clone()
        };

        let short_id = &task.uuid[..8];

        println!(
            "  {} {} {}",
            status_icon,
            title,
            format!("[{}]", short_id).truecolor(128, 128, 128)
        );
    }
}
