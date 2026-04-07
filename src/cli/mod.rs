use clap::{Parser, Subcommand};

pub mod args;
pub mod commands;

use args::{AreaCommand, BatchCommand, ConfigCommand, GlobalOpts, ListCommand, ProjectCommand, ShowCommand, TodoCommand};

/// Things CLI - 与 Things 3 交互的命令行工具
#[derive(Parser)]
#[command(
    name = "things",
    version,
    about = "A CLI tool for interacting with Things 3",
    long_about = None
)]
pub struct Cli {
    #[command(flatten)]
    pub global: GlobalOpts,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 待办事项管理
    #[command(subcommand)]
    Todo(TodoCommand),

    /// 项目管理
    #[command(subcommand)]
    Project(ProjectCommand),

    /// 区域管理
    #[command(subcommand)]
    Area(AreaCommand),

    /// 显示列表、项目或待办事项
    #[command(visible_alias = "open")]
    Show(ShowCommand),

    /// 搜索
    Search {
        /// 搜索关键词（可选，不提供则打开搜索界面）
        query: Option<String>,
    },

    /// 批量 JSON 操作
    #[command(subcommand)]
    Batch(BatchCommand),

    /// 列出任务、项目、区域等（从数据库读取）
    #[command(subcommand)]
    List(ListCommand),

    /// 配置管理
    #[command(subcommand)]
    Config(ConfigCommand),

    /// 显示版本信息
    Version {
        /// 同时显示 Things URL Scheme 版本
        #[arg(long)]
        verbose: bool,
    },
}

/// 运行 CLI
pub fn run() -> anyhow::Result<()> {
    let cli = Cli::parse();

    // 初始化日志（如果启用）
    if cli.global.debug {
        // TODO: 初始化 tracing
    }

    commands::handle_command(cli)
}
