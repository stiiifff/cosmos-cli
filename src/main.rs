use anyhow::Error;
use clap::Parser;
use dotenv::dotenv;
use types::*;

mod commands;
mod types;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    pretty_env_logger::init();

    let cli = Cli::parse();
    commands::execute_cmd(cli.command).await?;

    Ok(())
}
