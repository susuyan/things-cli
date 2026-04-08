//! AppleScript execution module for Things 3 operations
//!
//! Things URL Scheme doesn't support delete operations, so we use AppleScript
//! as a fallback for those actions.

use std::process::Command as ProcessCommand;

use super::ThingsError;

/// Execute AppleScript raw command (without Things app check)
fn execute_applescript_raw(script: &str) -> anyhow::Result<String> {
    let output = ProcessCommand::new("osascript")
        .arg("-e")
        .arg(script)
        .output()?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(ThingsError::CommandFailed(format!(
            "AppleScript execution failed: {}",
            stderr
        )).into());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(stdout.trim().to_string())
}

/// Execute AppleScript and return the result
/// Checks if Things 3 is running before executing
pub fn execute_applescript(script: &str) -> anyhow::Result<String> {
    // Check if Things is running before executing AppleScript
    if !is_things_running()? {
        return Err(ThingsError::AppNotRunning.into());
    }

    execute_applescript_raw(script)
}

/// Check if Things 3 is running
pub fn is_things_running() -> anyhow::Result<bool> {
    let script = r#"tell application "System Events" to return (name of processes) contains "Things3""#;
    let result = execute_applescript_raw(script)?;
    Ok(result == "true")
}

/// Delete a todo by ID using AppleScript
pub fn delete_todo(id: &str) -> anyhow::Result<()> {
    // Ensure ID doesn't contain quotes to prevent injection
    if id.contains('"') {
        return Err(ThingsError::InvalidId("ID contains invalid characters".to_string()).into());
    }

    let script = format!(
        r#"tell application "Things3"
    set found to false
    repeat with t in to dos
        if id of t is "{}" then
            delete t
            set found to true
            exit repeat
        end if
    end repeat
    if not found then
        return "not found"
    end if
    return "deleted"
end tell"#,
        id
    );

    let result = execute_applescript(&script)?;

    if result == "not found" {
        return Err(ThingsError::AppError(format!("Todo with ID '{}' not found", id)).into());
    }

    if result != "deleted" {
        return Err(ThingsError::AppError(format!("Failed to delete todo: {}", result)).into());
    }

    Ok(())
}

/// Delete a project by ID using AppleScript
pub fn delete_project(id: &str) -> anyhow::Result<()> {
    if id.contains('"') {
        return Err(ThingsError::InvalidId("ID contains invalid characters".to_string()).into());
    }

    let script = format!(
        r#"tell application "Things3"
    set found to false
    repeat with p in projects
        if id of p is "{}" then
            delete p
            set found to true
            exit repeat
        end if
    end repeat
    if not found then
        return "not found"
    end if
    return "deleted"
end tell"#,
        id
    );

    let result = execute_applescript(&script)?;

    if result == "not found" {
        return Err(ThingsError::AppError(format!("Project with ID '{}' not found", id)).into());
    }

    if result != "deleted" {
        return Err(ThingsError::AppError(format!("Failed to delete project: {}", result)).into());
    }

    Ok(())
}

/// Delete an area by ID using AppleScript
pub fn delete_area(id: &str) -> anyhow::Result<()> {
    if id.contains('"') {
        return Err(ThingsError::InvalidId("ID contains invalid characters".to_string()).into());
    }

    let script = format!(
        r#"tell application "Things3"
    set found to false
    repeat with a in areas
        if id of a is "{}" then
            delete a
            set found to true
            exit repeat
        end if
    end repeat
    if not found then
        return "not found"
    end if
    return "deleted"
end tell"#,
        id
    );

    let result = execute_applescript(&script)?;

    if result == "not found" {
        return Err(ThingsError::AppError(format!("Area with ID '{}' not found", id)).into());
    }

    if result != "deleted" {
        return Err(ThingsError::AppError(format!("Failed to delete area: {}", result)).into());
    }

    Ok(())
}

/// Create a new area using AppleScript
/// Returns the ID of the created area
pub fn create_area(title: &str) -> anyhow::Result<String> {
    // Escape quotes in title
    let safe_title = title.replace('"', "\\\"");

    let script = format!(
        r#"tell application "Things3"
    set newArea to make new area with properties {{name:"{}"}}
    return id of newArea
end tell"#,
        safe_title
    );

    execute_applescript(&script)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_script_injection_protection() {
        // Test that IDs with quotes are rejected
        let result = delete_todo(r#"test"); do something evil; "#);
        assert!(result.is_err());
    }

    #[test]
    fn test_is_things_running_doesnt_crash() {
        // This test just verifies the function doesn't panic
        // It may return true or false depending on whether Things is running
        let _ = is_things_running();
    }
}
