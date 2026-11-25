mod auth;
mod config;
mod error;
mod html;
mod http;

use crate::{config::Config, error::Result};

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let config = Config::from_env()?;

    let _ = auth::authenticate(&config).await?;

    println!("Authentication successful");

    Ok(())
}
