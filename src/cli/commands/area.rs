use crate::cli::args::AreaCommand;
use crate::cli::GlobalOpts;
use crate::core::applescript;
use crate::db::store::ThingsDb;
use serde_json::json;

pub fn handle(cmd: AreaCommand, global: &GlobalOpts, json: bool) -> anyhow::Result<()> {
    match cmd {
        AreaCommand::Add { title, tags, reveal } => {
            handle_add(title, tags, reveal, global, json)?;
        }
        AreaCommand::Update {
            id,
            title,
            tags,
            add_tags,
            reveal,
        } => {
            handle_update(id, title, tags, add_tags, reveal, global, json)?;
        }
        AreaCommand::Delete { id, force } => {
            handle_delete(&id, force, json)?;
        }
        AreaCommand::Get { id } => {
            handle_get(&id, global.json)?;
        }
    }

    Ok(())
}

fn handle_add(
    title: String,
    _tags: Vec<String>,
    _reveal: bool,
    _global: &GlobalOpts,
    json: bool,
) -> anyhow::Result<()> {
    // URL Scheme 不支持 add-area，使用 AppleScript
    match applescript::create_area(&title) {
        Ok(area_id) => {
            if json {
                let output = json!({
                    "success": true,
                    "operation": "add",
                    "type": "area",
                    "data": {
                        "title": title,
                        "id": area_id
                    }
                });
                println!("{}", serde_json::to_string_pretty(&output)?);
            } else {
                println!("✓ Area added: {} (id: {})", title, area_id);
            }
        }
        Err(e) => {
            if json {
                let output = json!({
                    "success": false,
                    "error": {
                        "code": "CREATE_FAILED",
                        "message": e.to_string()
                    }
                });
                println!("{}", serde_json::to_string_pretty(&output)?);
            } else {
                return Err(e);
            }
        }
    }

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
    json: bool,
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

        match applescript::execute_applescript(&script) {
            Ok(_) => {
                if json {
                    let output = json!({
                        "success": true,
                        "operation": "update",
                        "type": "area",
                        "data": {
                            "id": id,
                            "title": t
                        }
                    });
                    println!("{}", serde_json::to_string_pretty(&output)?);
                } else {
                    println!("✓ Area updated: {}", id);
                }
            }
            Err(e) => {
                if json {
                    let output = json!({
                        "success": false,
                        "error": {
                            "code": "UPDATE_FAILED",
                            "message": e.to_string()
                        }
                    });
                    println!("{}", serde_json::to_string_pretty(&output)?);
                } else {
                    return Err(e);
                }
            }
        }
    } else {
        // No title to update
        if json {
            let output = json!({
                "success": true,
                "operation": "update",
                "type": "area",
                "data": {
                    "id": id,
                    "message": "No changes made"
                }
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        } else {
            println!("✓ Area updated: {}", id);
        }
    }

    // 注意：AppleScript 不支持直接更新 area 的标签

    Ok(())
}

fn handle_delete(id: &str, force: bool, json: bool) -> anyhow::Result<()> {
    // 确认删除（非 JSON 模式下）
    if !force && !json {
        let confirmed = dialoguer::Confirm::new()
            .with_prompt(format!("Are you sure you want to delete area '{}'", id))
            .default(false)
            .interact()?;

        if !confirmed {
            if json {
                let output = json!({
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
    match applescript::delete_area(id) {
        Ok(_) => {
            if json {
                let output = json!({
                    "success": true,
                    "operation": "delete",
                    "type": "area",
                    "data": {
                        "id": id
                    }
                });
                println!("{}", serde_json::to_string_pretty(&output)?);
            } else {
                println!("✓ Area deleted: {}", id);
            }
        }
        Err(e) => {
            if json {
                let output = json!({
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

fn handle_get(id: &str, json: bool) -> anyhow::Result<()> {
    let db = ThingsDb::open_default()?;

    match db.get_area_by_id(id)? {
        Some(area) => {
            if json {
                let output = json!({
                    "success": true,
                    "type": "area",
                    "data": area
                });
                println!("{}", serde_json::to_string_pretty(&output)?);
            } else {
                println!("ID: {}", area.uuid);
                println!("Title: {}", area.title);
                println!("Index: {}", area.index);
            }
            Ok(())
        }
        None => {
            if json {
                let output = json!({
                    "success": false,
                    "error": format!("Area not found: {}", id)
                });
                println!("{}", serde_json::to_string_pretty(&output)?);
            } else {
                eprintln!("Error: Area not found: {}", id);
            }
            std::process::exit(1);
        }
    }
}
