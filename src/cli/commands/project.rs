use crate::cli::args::ProjectCommand;
use crate::cli::GlobalOpts;
use crate::config::store::{FileStore, ConfigStore};
use crate::core::executor::{Executor, OpenExecutor};
use crate::core::parser;
use crate::core::url_builder::{Command, ThingsUrl};

pub fn handle(cmd: ProjectCommand, global: &GlobalOpts, json: bool) -> anyhow::Result<()> {
    match cmd {
        ProjectCommand::Add {
            title,
            notes,
            when,
            deadline,
            tags,
            area,
            area_id,
            todos,
            completed,
            canceled,
            reveal,
            json: json_file,
        } => {
            if let Some(json_file) = json_file {
                handle_add_from_json(&json_file, reveal, global, json)?;
            } else {
                handle_add(
                    title,
                    notes,
                    when,
                    deadline,
                    tags,
                    area,
                    area_id,
                    todos,
                    completed,
                    canceled,
                    reveal,
                    global,
                    json,
                )?;
            }
        }
        ProjectCommand::Update {
            id,
            title,
            notes,
            prepend_notes,
            append_notes,
            when,
            deadline,
            tags,
            add_tags,
            area,
            area_id,
            complete,
            uncomplete,
            cancel,
            duplicate,
            reveal,
        } => {
            handle_update(
                id, title, notes, prepend_notes, append_notes, when, deadline, tags, add_tags,
                area, area_id, complete, uncomplete, cancel, duplicate, reveal, global, json,
            )?;
        }
        ProjectCommand::Delete { id, force } => {
            handle_delete(&id, force, json)?;
        }
        ProjectCommand::Get { id } => {
            handle_get(&id, global.json)?;
        }
        ProjectCommand::Find { title } => {
            handle_find(&title, global.json)?;
        }
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn handle_add(
    title: String,
    notes: Option<String>,
    when: Option<String>,
    deadline: Option<String>,
    tags: Vec<String>,
    area: Option<String>,
    area_id: Option<String>,
    todos: Vec<String>,
    completed: bool,
    canceled: bool,
    reveal: bool,
    global: &GlobalOpts,
    json: bool,
) -> anyhow::Result<()> {
    // 解析参数
    let when_param = when
        .as_deref()
        .map(parser::parse_when)
        .transpose()?
        .map(|w| w.to_url_param());

    let deadline_param = deadline;

    // 标签
    let tags_str = if tags.is_empty() {
        None
    } else {
        Some(tags.join(","))
    };

    // 确定区域
    let (final_area, final_area_id) = if area_id.is_some() {
        (None, area_id)
    } else {
        (area, None)
    };

    // 构建 URL
    let mut url = ThingsUrl::new(Command::AddProject)
        .param("title", &title)
        .param_opt("notes", notes.as_deref())
        .param_opt("when", when_param.as_deref())
        .param_opt("deadline", deadline_param.as_deref())
        .param_opt("tags", tags_str.as_deref())
        .param_opt("area", final_area.as_deref())
        .param_opt("area-id", final_area_id.as_deref());

    // 待办事项（逗号分隔转为换行分隔）
    if !todos.is_empty() {
        url = url.param("to-dos", &todos.join("\n"));
    }

    // 布尔参数
    if completed {
        url = url.param_bool("completed", true);
    }
    if canceled {
        url = url.param_bool("canceled", true);
    }
    if reveal {
        url = url.param_bool("reveal", true);
    }

    let url = url.build();

    if global.debug {
        eprintln!("URL: {}", url);
    }

    // dry-run 模式：只打印 URL 不执行
    if global.dry_run {
        if json {
            let output = serde_json::json!({
                "success": true,
                "operation": "add",
                "type": "project",
                "dry_run": true,
                "data": {
                    "title": title,
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
                "type": "project",
                "data": {
                    "title": title
                }
            })
        } else {
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "EXECUTION_FAILED",
                    "message": "Failed to add project"
                }
            })
        };
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else if result.success {
        println!("✓ Project added: {}", title);
    }

    Ok(())
}

fn handle_add_from_json(
    file: &str,
    reveal: bool,
    global: &GlobalOpts,
    json: bool,
) -> anyhow::Result<()> {
    // 读取 JSON 文件
    let json_data = std::fs::read_to_string(file)
        .map_err(|e| anyhow::anyhow!("Failed to read file '{}': {}", file, e))?;

    // 验证 JSON 格式
    let _: serde_json::Value = serde_json::from_str(&json_data)
        .map_err(|e| anyhow::anyhow!("Invalid JSON format: {}", e))?;

    // 构建 URL（不需要 auth-token，因为是 add-project 不是 update）
    let mut url = ThingsUrl::new(Command::Json)
        .param("data", &json_data);

    if reveal {
        url = url.param_bool("reveal", true);
    }

    let url = url.build();

    if global.debug {
        eprintln!("URL: {}", url);
    }

    // dry-run 模式：只打印 URL 不执行
    if global.dry_run {
        if json {
            let output = serde_json::json!({
                "success": true,
                "operation": "add",
                "type": "project",
                "source": "json_file",
                "dry_run": true,
                "data": {
                    "file": file,
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
                "type": "project",
                "source": "json_file",
                "data": {
                    "file": file
                }
            })
        } else {
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "EXECUTION_FAILED",
                    "message": "Failed to create project from JSON"
                }
            })
        };
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else if result.success {
        println!("✓ Project created from JSON");
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
    area: Option<String>,
    area_id: Option<String>,
    complete: bool,
    uncomplete: bool,
    cancel: bool,
    duplicate: bool,
    reveal: bool,
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
    let mut url = ThingsUrl::new(Command::UpdateProject(id.clone()))
        .param_opt("title", title.as_deref())
        .param_opt("notes", notes.as_deref())
        .param_opt("prepend-notes", prepend_notes.as_deref())
        .param_opt("append-notes", append_notes.as_deref())
        .param_opt("when", when_param.as_deref())
        .param_opt("area", area.as_deref())
        .param_opt("area-id", area_id.as_deref());

    // deadline 特殊处理
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
                "type": "project",
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
                "type": "project",
                "data": {
                    "id": id
                }
            })
        } else {
            serde_json::json!({
                "success": false,
                "error": {
                    "code": "EXECUTION_FAILED",
                    "message": format!("Failed to update project: {}", id)
                }
            })
        };
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else if result.success {
        println!("✓ Project updated: {}", id);
    }

    Ok(())
}

use crate::core::applescript;
use crate::db::ThingsDb;
use colored::Colorize;
use serde::Serialize;

fn handle_delete(id: &str, force: bool, json: bool) -> anyhow::Result<()> {
    // 确认删除（非 JSON 模式下）
    if !force && !json {
        let confirmed = dialoguer::Confirm::new()
            .with_prompt(format!("Are you sure you want to delete project '{}'", id))
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
    match applescript::delete_project(id) {
        Ok(_) => {
            if json {
                let output = serde_json::json!({
                    "success": true,
                    "operation": "delete",
                    "type": "project",
                    "data": {
                        "id": id
                    }
                });
                println!("{}", serde_json::to_string_pretty(&output)?);
            } else {
                println!("✓ Project deleted: {}", id);
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

/// JSON 输出结构
#[derive(Serialize)]
struct ProjectJsonOutput {
    success: bool,
    #[serde(rename = "type")]
    data_type: String,
    data: crate::db::Project,
}

#[derive(Serialize)]
struct ProjectsJsonOutput {
    success: bool,
    #[serde(rename = "type")]
    data_type: String,
    count: usize,
    data: Vec<crate::db::Project>,
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

    // 获取项目
    let project = db.get_project_by_id(id)?;

    match project {
        Some(project) => {
            if json {
                let output = ProjectJsonOutput {
                    success: true,
                    data_type: "project".to_string(),
                    data: project,
                };
                println!("{}", serde_json::to_string_pretty(&output)?);
            } else {
                print_project_details(&project);
            }
        }
        None => {
            if json {
                let error_output = serde_json::json!({
                    "success": false,
                    "error": format!("Project not found: {}", id)
                });
                println!("{}", serde_json::to_string_pretty(&error_output)?);
            } else {
                eprintln!("{} Project not found: {}", "Error:".red().bold(), id);
            }
        }
    }

    Ok(())
}

fn handle_find(title: &str, json: bool) -> anyhow::Result<()> {
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

    // 搜索项目
    let projects = db.search_projects_by_title(title)?;

    if json {
        let output = ProjectsJsonOutput {
            success: true,
            data_type: "projects".to_string(),
            count: projects.len(),
            data: projects,
        };
        println!("{}", serde_json::to_string_pretty(&output)?);
    } else {
        print_projects(&projects, title);
    }

    Ok(())
}

fn print_project_details(project: &crate::db::Project) {
    println!("{}", "Project Details".bold().underline());
    println!();

    // ID
    println!("  {}: {}", "ID".bold(), project.uuid);

    // Title
    println!("  {}: {}", "Title".bold(), project.title);

    // Notes
    if let Some(ref notes) = project.notes && !notes.is_empty() {
        println!("  {}: {}", "Notes".bold(), notes);
    }

    // Tags
    if !project.tags.is_empty() {
        let tags_str = format!("#{}", project.tags.join(" #"));
        println!("  {}: {}", "Tags".bold(), tags_str.cyan());
    }

    // Deadline
    if let Some(deadline) = project.deadline {
        println!("  {}: {}", "Deadline".bold(), deadline.to_string().yellow());
    }

    // Area
    if let Some(ref area) = project.area {
        println!("  {}: {}", "Area".bold(), area.truecolor(128, 128, 128));
    }

    // Creation Date
    if let Some(creation_date) = project.creation_date {
        println!("  {}: {}", "Created".bold(), creation_date.format("%Y-%m-%d %H:%M"));
    }

    // Completion Date
    if let Some(completion_date) = project.completion_date {
        let formatted = completion_date.format("%Y-%m-%d %H:%M").to_string();
        println!("  {}: {}", "Completed".bold(), formatted.green());
    }
}

fn print_projects(projects: &[crate::db::Project], query: &str) {
    println!("{}", format!("Projects matching '{}'", query).bold().underline());
    if projects.is_empty() {
        println!("  {}", "No projects found".dimmed());
        return;
    }

    for project in projects {
        let area_info = project.area.as_ref()
            .map(|a: &String| format!(" [{}]", a.truecolor(128, 128, 128)))
            .unwrap_or_default();

        let tag_info = if !project.tags.is_empty() {
            format!(" {}", format!("#{}" , project.tags.join(" #")).cyan())
        } else {
            String::new()
        };

        println!(
            "  • {}{}{}{}",
            project.title.bold(),
            area_info,
            tag_info,
            format!(" [{}]", &project.uuid[..8]).truecolor(128, 128, 128)
        );
    }

    println!();
    println!("  Total: {}", projects.len().to_string().bold());
}
