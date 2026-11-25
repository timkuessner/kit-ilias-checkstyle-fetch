mod auth;
mod client;
mod config;
mod crawl;
mod error;
mod form;
mod html;

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
    let client = auth::authenticate(&config).await?;
    println!("Authentication successful");
    crawl::run(&client).await?;
    Ok(())
}
