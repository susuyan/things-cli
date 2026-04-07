pub mod area;
pub mod batch;
pub mod config;
pub mod list;
pub mod project;
pub mod show;
pub mod todo;

use crate::cli::{Cli, Commands};

/// 处理命令分发
pub fn handle_command(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Commands::Todo(cmd) => todo::handle(cmd, &cli.global),
        Commands::Project(cmd) => project::handle(cmd, &cli.global),
        Commands::Area(cmd) => area::handle(cmd, &cli.global),
        Commands::Show(cmd) => show::handle(cmd, &cli.global),
        Commands::Search { query } => show::handle_search(query, &cli.global),
        Commands::Batch(cmd) => batch::handle(cmd, &cli.global),
        Commands::List(cmd) => list::handle(cmd),
        Commands::Config(cmd) => config::handle(cmd),
        Commands::Version { verbose } => show::handle_version(verbose),
    }
}
