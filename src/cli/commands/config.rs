use colored::Colorize;
use dialoguer::{Confirm, Password};

use crate::cli::args::ConfigCommand;
use crate::config::store::{ConfigStore, FileStore, AUTH_TOKEN_ENV_VAR};

/// 处理配置命令
pub fn handle(cmd: ConfigCommand) -> anyhow::Result<()> {
    let store = FileStore::new()?;

    match cmd {
        ConfigCommand::SetAuthToken { token } => {
            let token = if let Some(t) = token {
                t
            } else {
                // 交互式输入
                println!("{}", "Setting up Things authorization token".bold());
                println!();
                println!("To modify existing Things data, you need an authorization token.");
                println!("You can find it in Things settings:");
                println!("  Mac: Things → Settings → General → Enable Things URLs → Manage");
                println!("  iOS: Settings → General → Things URLs");
                println!();

                Password::new()
                    .with_prompt("Enter your auth-token")
                    .interact()?
            };

            println!();
            println!("{}", "Auth token configuration".bold());
            println!();
            println!(
                "Set the environment variable in your shell:"
            );
            println!("  export {}='{}'", AUTH_TOKEN_ENV_VAR.cyan(), token);
            println!();
            println!(
                "You can add this to your {} or {} file.",
                "~/.zshrc".dimmed(),
                "~/.bashrc".dimmed()
            );
        }

        ConfigCommand::DeleteAuthToken => {
            let has_token = store.has_auth_token()?;
            if !has_token {
                println!("{}", "No auth-token is currently set".yellow());
                return Ok(());
            }

            let confirm = Confirm::new()
                .with_prompt("Are you sure you want to remove the auth-token from environment?")
                .default(false)
                .interact()?;

            if confirm {
                println!("{}", "Please remove the following from your shell profile:".yellow());
                println!("  unset {}", AUTH_TOKEN_ENV_VAR.cyan());
            } else {
                println!("Cancelled");
            }
        }

        ConfigCommand::CheckAuthToken => {
            // 检查环境变量
            let has_token = store.has_auth_token()?;

            if has_token {
                println!("{}", "✓ Auth token is configured via environment variable".green());
                println!("  Variable: {}", AUTH_TOKEN_ENV_VAR.cyan());
            } else {
                println!("{}", "✗ Auth token is not configured".red());
                println!();
                println!("Set the environment variable:");
                println!("  export {}='your-token'", AUTH_TOKEN_ENV_VAR.cyan());
                println!();
                println!("Or run {} for interactive setup", "things config set-auth-token".cyan());
            }
        }

        ConfigCommand::SetDefaultList { list } => {
            let mut config = store.load_config()?;
            config.default_list = Some(list.clone());
            store.save_config(&config)?;
            println!("{} {}", "✓ Default list set to:".green(), list.cyan());
        }

        ConfigCommand::SetDefaultTags { tags } => {
            let mut config = store.load_config()?;
            config.default_tags = tags.clone();
            store.save_config(&config)?;
            println!(
                "{} {}",
                "✓ Default tags set to:".green(),
                tags.join(", ").cyan()
            );
        }

        ConfigCommand::Show => {
            show_config(&store)?;
        }

        ConfigCommand::Edit => {
            let config_path = crate::config::config_file_path()?;

            // 确保文件存在
            if !config_path.exists() {
                let default_config = crate::config::Config::default();
                store.save_config(&default_config)?;
            }

            // 使用系统默认编辑器打开
            edit::edit_file(&config_path)?;
            println!("{} {}", "✓ Config file edited:".green(), config_path.display());
        }
    }

    Ok(())
}

/// 显示当前配置
fn show_config(store: &FileStore) -> anyhow::Result<()> {
    println!("{}", "Things CLI Configuration".bold().underline());
    println!();

    // 显示 auth-token 状态
    let has_token = store.has_auth_token()?;

    if has_token {
        println!("Auth Token: {}", "✓ configured via environment variable".green());
        println!("  Variable: {}", AUTH_TOKEN_ENV_VAR.cyan());
    } else {
        println!("Auth Token: {}", "✗ not configured".red());
    }

    // 显示其他配置
    let config = store.load_config()?;

    println!(
        "Default List: {}",
        config
            .default_list
            .as_ref()
            .map(|s: &String| s.cyan().to_string())
            .unwrap_or_else(|| "not set".dimmed().to_string())
    );

    println!(
        "Default Tags: {}",
        if config.default_tags.is_empty() {
            "not set".dimmed().to_string()
        } else {
            config.default_tags.join(", ").cyan().to_string()
        }
    );

    println!(
        "Debug Mode: {}",
        if config.debug { "on".yellow() } else { "off".dimmed() }
    );

    println!();
    println!(
        "Config file: {}",
        crate::config::config_file_path()?.display()
    );

    Ok(())
}
