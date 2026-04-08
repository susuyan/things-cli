//! Integration tests for things-cli
//!
//! These tests verify the CLI interface works correctly.
//! They use the --help flag to test command parsing without
//! actually invoking Things.

use std::process::Command;

fn things_bin() -> Command {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_things"));
    cmd.env("NO_COLOR", "1");
    cmd
}

#[test]
fn test_version() {
    let output = things_bin()
        .arg("--version")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("things"));
    assert!(stdout.contains("0.1.0"));
}

#[test]
fn test_help() {
    let output = things_bin()
        .arg("--help")
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Usage:"));
    assert!(stdout.contains("Commands:"));
    assert!(stdout.contains("todo"));
    assert!(stdout.contains("project"));
    assert!(stdout.contains("show"));
    assert!(stdout.contains("search"));
    assert!(stdout.contains("list"));
    assert!(stdout.contains("config"));
}

#[test]
fn test_todo_add_help() {
    let output = things_bin()
        .args(["todo", "add", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Help is in Chinese
    assert!(stdout.contains("添加待办事项"));
    assert!(stdout.contains("--notes"));
    assert!(stdout.contains("--when"));
    assert!(stdout.contains("--tags"));
}

#[test]
fn test_todo_update_help() {
    let output = things_bin()
        .args(["todo", "update", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Help is in Chinese
    assert!(stdout.contains("更新待办事项"));
    assert!(stdout.contains("<ID>"));
}

#[test]
fn test_project_add_help() {
    let output = things_bin()
        .args(["project", "add", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Help is in Chinese
    assert!(stdout.contains("添加项目"));
    assert!(stdout.contains("--area"));
    assert!(stdout.contains("--todos"));
}

#[test]
fn test_show_help() {
    let output = things_bin()
        .args(["show", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Help is in Chinese
    assert!(stdout.contains("显示"));
    assert!(stdout.contains("--id"));
    assert!(stdout.contains("--filter"));
}

#[test]
fn test_search_help() {
    let output = things_bin()
        .args(["search", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Help is in Chinese
    assert!(stdout.contains("搜索"));
}

#[test]
fn test_list_help() {
    let output = things_bin()
        .args(["list", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Commands:"));
    assert!(stdout.contains("inbox"));
    assert!(stdout.contains("today"));
}

#[test]
fn test_batch_help() {
    let output = things_bin()
        .args(["batch", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("import"));
    assert!(stdout.contains("template"));
}

#[test]
fn test_config_help() {
    let output = things_bin()
        .args(["config", "--help"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("set-auth-token"));
    assert!(stdout.contains("show"));
}

#[test]
fn test_version_flag() {
    let output = things_bin()
        .args(["version"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("things"));
}

#[test]
fn test_debug_flag() {
    // Test that --debug flag is accepted (it enables debug output during execution)
    let output = things_bin()
        .args(["--debug", "version"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
}

#[test]
fn test_list_subcommands() {
    // Test that all list subcommands are valid
    let subcommands = [
        "inbox", "today", "evening", "upcoming", "someday",
        "anytime", "completed", "completed-today", "canceled",
        "deadlines", "projects", "areas", "tags"
    ];

    for cmd in &subcommands {
        let output = things_bin()
            .args(["list", cmd, "--help"])
            .output()
            .unwrap_or_else(|_| panic!("Failed to execute list {}", cmd));

        assert!(output.status.success(), "list {} should have valid help", cmd);
    }
}

#[test]
fn test_batch_template_output() {
    let output = things_bin()
        .args(["batch", "template"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    // Should output valid JSON
    assert!(stdout.contains("type"));
    assert!(stdout.contains("project"));
    assert!(stdout.contains("to-do"));

    // Verify it's valid JSON
    let json: serde_json::Value = serde_json::from_str(&stdout)
        .expect("Output should be valid JSON");
    assert!(json.is_array());
}

#[test]
fn test_invalid_command() {
    let output = things_bin()
        .args(["invalid-command"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("error"));
}

#[test]
fn test_todo_add_missing_required() {
    // todo add requires at least one title
    let output = things_bin()
        .args(["todo", "add"])
        .output()
        .expect("Failed to execute command");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("required"));
}

#[test]
fn test_todo_add_with_repeat() {
    // Test that --repeat option is valid (dry-run via --debug to see URL)
    let output = things_bin()
        .args(["todo", "add", "Test Task", "--repeat", "day", "--debug"])
        .output()
        .expect("Failed to execute command");

    assert!(output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    // Debug output should contain the URL with repeat parameter
    assert!(stderr.contains("repeat=day"));
}
