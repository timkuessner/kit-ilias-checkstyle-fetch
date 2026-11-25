mod config;
mod error;
mod http;

use url::Url;

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
    let client = HttpClient::new()?;

    let url = Url::parse("https://ilias.studium.kit.edu")?;

    let (final_url, _) = client.get_html(&url).await?;

    println!("Final URL: {final_url}");

    Ok(())
}
