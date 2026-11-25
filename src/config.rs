use crate::error::{AppError, Result};

pub struct Config {
    username: String,
    password: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        dotenvy::dotenv().ok();

        let username = std::env::var("KIT_USERNAME")
            .map_err(|_| AppError::Config("KIT_USERNAME is not set".to_string()))?;

        let password = std::env::var("KIT_PASSWORD")
            .map_err(|_| AppError::Config("KIT_PASSWORD is not set".to_string()))?;

        if username.trim().is_empty() {
            return Err(AppError::Config(
                "KIT_USERNAME must not be empty".to_string(),
            ));
        }

        if password.is_empty() {
            return Err(AppError::Config(
                "KIT_PASSWORD must not be empty".to_string(),
            ));
        }

        Ok(Self { username, password })
    }

    pub fn username(&self) -> &str {
        &self.username
    }

    pub fn password(&self) -> &str {
        &self.password
    }
}
