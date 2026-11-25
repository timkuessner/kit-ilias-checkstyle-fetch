mod config;
mod error;
mod http;

use crate::error::Result;
use crate::http::HttpClient;

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let http_client = HttpClient::new()?;

    Ok(())
}
