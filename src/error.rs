use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Config error: {0}")]
    Config(String),

    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
}

pub type Result<T> = std::result::Result<T, AppError>;
