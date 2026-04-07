use colored::Colorize;

mod cli;
mod config;
mod core;
mod db;

fn main() {
    if let Err(e) = cli::run() {
        eprintln!("{} {}", "Error:".red().bold(), e);
        std::process::exit(1);
    }
}
