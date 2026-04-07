use std::str::FromStr;

use crate::cli::args::ShowCommand;
use crate::cli::GlobalOpts;
use crate::core::executor::{Executor, OpenExecutor};
use crate::core::models::BuiltinList;
use crate::core::url_builder::{Command, ThingsUrl};

pub fn handle(cmd: ShowCommand, global: &GlobalOpts) -> anyhow::Result<()> {
    // 构建 URL
    let url = if let Some(id) = cmd.id {
        // 通过 ID 显示
        ThingsUrl::new(Command::Show)
            .param("id", &id)
            .build()
    } else if let Some(query) = cmd.query {
        // 检查是否是内置列表
        if let Ok(builtin) = BuiltinList::from_str(&query) {
            // 内置列表
            let mut url = ThingsUrl::new(Command::Show)
                .param("id", builtin.as_str());

            // 添加过滤器
            if !cmd.filter.is_empty() {
                url = url.param("filter", &cmd.filter.join(","));
            }

            url.build()
        } else {
            // 通过标题查找
            let mut url = ThingsUrl::new(Command::Show)
                .param("query", &query);

            if !cmd.filter.is_empty() {
                url = url.param("filter", &cmd.filter.join(","));
            }

            url.build()
        }
    } else {
        // 没有提供 query 或 id，默认显示 inbox
        ThingsUrl::new(Command::Show)
            .param("id", "inbox")
            .build()
    };

    if global.debug {
        eprintln!("URL: {}", url);
    }

    // 执行
    let executor = OpenExecutor::new();
    executor.execute(&url)?;

    Ok(())
}

pub fn handle_search(query: Option<String>, global: &GlobalOpts) -> anyhow::Result<()> {
    let url = ThingsUrl::new(Command::Search)
        .param_opt("query", query.as_deref())
        .build();

    if global.debug {
        eprintln!("URL: {}", url);
    }

    let executor = OpenExecutor::new();
    executor.execute(&url)?;

    Ok(())
}

pub fn handle_version(verbose: bool) -> anyhow::Result<()> {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    if verbose {
        println!("  URL Scheme version: 2");
        println!("  Platform: macOS");
    }

    Ok(())
}
