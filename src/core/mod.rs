use thiserror::Error;

pub mod applescript;
pub mod executor;
pub mod models;
pub mod parser;
pub mod url_builder;

#[allow(unused_imports)]
pub use executor::{ExecutionResult, Executor, OpenExecutor};
#[allow(unused_imports)]
pub use url_builder::{Command, ThingsUrl};

/// 核心错误类型
#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum ThingsError {
    #[error("Things app not found. Please install Things 3 from the App Store.")]
    AppNotFound,

    #[error("URL scheme not enabled. Please enable it in Things settings (Settings > General > Things URLs).")]
    SchemeNotEnabled,

    #[error("Authentication required. Please run `things config set-auth-token` to set your authorization token.")]
    AuthRequired,

    #[error("Invalid date format: {0}")]
    InvalidDate(String),

    #[error("Invalid time format: {0}")]
    InvalidTime(String),

    #[error("Invalid ID format: {0}")]
    InvalidId(String),

    #[error("Things returned an error: {0}")]
    ThingsError(String),

    #[error("Command failed: {0}")]
    CommandFailed(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("URL encoding error: {0}")]
    UrlEncode(String),
}

/// 结果类型别名
#[allow(dead_code)]
pub type Result<T> = std::result::Result<T, ThingsError>;
