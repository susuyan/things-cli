use std::io::{self, BufRead};

use crate::cli::args::TodoCommand;
use crate::cli::GlobalOpts;
use crate::config::store::{CompositeStore, ConfigStore};
use crate::core::executor::{Executor, OpenExecutor};
use crate::core::parser;
use crate::core::url_builder::{Command, ThingsUrl};

pub fn handle(cmd: TodoCommand, global: &GlobalOpts) -> anyhow::Result<()> {
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
                repeat, repeat_until, no_repeat, global,
            )?;
        }
        TodoCommand::Delete { id, force } => {
            handle_delete(&id, force)?;
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
) -> anyhow::Result<()> {
    let store = CompositeStore::new()?;
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

    // 执行
    let executor = OpenExecutor::new();
    let result = executor.execute(&url)?;

    if result.success {
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

    // 执行
    let executor = OpenExecutor::new();
    let result = executor.execute(&url)?;

    if result.success {
        println!("✓ Todo updated: {}", id);
    }

    Ok(())
}

use crate::core::applescript;

fn handle_delete(id: &str, force: bool) -> anyhow::Result<()> {
    // 确认删除
    if !force {
        let confirmed = dialoguer::Confirm::new()
            .with_prompt(format!("Are you sure you want to delete todo '{}'", id))
            .default(false)
            .interact()?;

        if !confirmed {
            println!("Cancelled");
            return Ok(());
        }
    }

    // 使用 AppleScript 删除
    applescript::delete_todo(id)?;

    println!("✓ Todo deleted: {}", id);
    Ok(())
}
