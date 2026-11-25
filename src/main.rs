mod config;
mod error;

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {error}");
        std::process::exit(1);
    }
}

fn run() -> error::Result<()> {
    let config = config::Config::from_env()?;

    println!("Configuration loaded!");
    println!("Username: {}", config.username);

    if !config.password.is_empty() {
        println!("Password is set");
    }

    Ok(())
}
