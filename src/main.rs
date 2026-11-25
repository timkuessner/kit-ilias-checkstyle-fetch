mod config;
mod error;
mod http;

use crate::error::Result;
use crate::http::HttpClient;

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}

async fn run() -> Result<()> {
    let http_client = HttpClient::new()?;

    let body = http_client.get("https://ilias.studium.kit.edu").await?;

    println!("{}", body);

    Ok(())
}
