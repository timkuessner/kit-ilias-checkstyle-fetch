use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Config error: {0}")]
    Config(String),
}

pub type Result<T> = std::result::Result<T, AppError>;