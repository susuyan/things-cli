use crate::cli::args::ProjectCommand;
use crate::cli::GlobalOpts;
use crate::config::store::{CompositeStore, ConfigStore};
use crate::core::executor::{Executor, OpenExecutor};
use crate::core::parser;
use crate::core::url_builder::{Command, ThingsUrl};

pub fn handle(cmd: ProjectCommand, global: &GlobalOpts) -> anyhow::Result<()> {
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
            json,
        } => {
            if let Some(json_file) = json {
                handle_add_from_json(&json_file, reveal, global)?;
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
                area, area_id, complete, uncomplete, cancel, duplicate, reveal, global,
            )?;
        }
        ProjectCommand::Delete { id, force } => {
            handle_delete(&id, force)?;
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

    // 执行
    let executor = OpenExecutor::new();
    let result = executor.execute(&url)?;

    if result.success {
        println!("✓ Project added: {}", title);
    }

    Ok(())
}

fn handle_add_from_json(
    file: &str,
    reveal: bool,
    global: &GlobalOpts,
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

    // 执行
    let executor = OpenExecutor::new();
    let result = executor.execute(&url)?;

    if result.success {
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
) -> anyhow::Result<()> {
    let store = CompositeStore::new()?;

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

    // 执行
    let executor = OpenExecutor::new();
    let result = executor.execute(&url)?;

    if result.success {
        println!("✓ Project updated: {}", id);
    }

    Ok(())
}

use crate::core::applescript;

fn handle_delete(id: &str, force: bool) -> anyhow::Result<()> {
    // 确认删除
    if !force {
        let confirmed = dialoguer::Confirm::new()
            .with_prompt(format!("Are you sure you want to delete project '{}'", id))
            .default(false)
            .interact()?;

        if !confirmed {
            println!("Cancelled");
            return Ok(());
        }
    }

    // 使用 AppleScript 删除
    applescript::delete_project(id)?;

    println!("✓ Project deleted: {}", id);
    Ok(())
}
