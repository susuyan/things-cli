use crate::cli::args::AreaCommand;
use crate::cli::GlobalOpts;
use crate::core::applescript;

pub fn handle(cmd: AreaCommand, global: &GlobalOpts) -> anyhow::Result<()> {
    match cmd {
        AreaCommand::Add { title, tags, reveal } => {
            handle_add(title, tags, reveal, global)?;
        }
        AreaCommand::Update {
            id,
            title,
            tags,
            add_tags,
            reveal,
        } => {
            handle_update(id, title, tags, add_tags, reveal, global)?;
        }
        AreaCommand::Delete { id, force } => {
            handle_delete(&id, force)?;
        }
    }

    Ok(())
}

fn handle_add(
    title: String,
    _tags: Vec<String>,
    _reveal: bool,
    _global: &GlobalOpts,
) -> anyhow::Result<()> {
    // URL Scheme 不支持 add-area，使用 AppleScript
    let area_id = applescript::create_area(&title)?;

    println!("✓ Area added: {} (id: {})", title, area_id);

    // 注意：AppleScript 创建的 area 不支持直接添加标签或 reveal
    // 这些功能需要通过 update 命令或 JSON 批量操作来实现

    Ok(())
}

fn handle_update(
    id: String,
    title: Option<String>,
    _tags: Vec<String>,
    _add_tags: Vec<String>,
    _reveal: bool,
    _global: &GlobalOpts,
) -> anyhow::Result<()> {
    // URL Scheme 不支持 update-area，使用 AppleScript
    if let Some(t) = title {
        // Escape quotes in title
        let safe_title = t.replace('"', "\\\"");

        let script = format!(
            r#"tell application "Things3"
    set targetArea to area id "{}"
    set name of targetArea to "{}"
end tell"#,
            id, safe_title
        );

        applescript::execute_applescript(&script)?;
    }

    // 注意：AppleScript 不支持直接更新 area 的标签

    println!("✓ Area updated: {}", id);
    Ok(())
}

fn handle_delete(id: &str, force: bool) -> anyhow::Result<()> {
    // 确认删除
    if !force {
        let confirmed = dialoguer::Confirm::new()
            .with_prompt(format!("Are you sure you want to delete area '{}'", id))
            .default(false)
            .interact()?;

        if !confirmed {
            println!("Cancelled");
            return Ok(());
        }
    }

    // 使用 AppleScript 删除
    applescript::delete_area(id)?;

    println!("✓ Area deleted: {}", id);
    Ok(())
}
