use anyhow::Error;
use clap::Parser;
use context::Context;
use dotenv::dotenv;
use json_to_table::json_to_table;
use types::*;

use crate::utils::scalar_to_plain;

mod commands;
mod context;
mod types;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    pretty_env_logger::init();

    let ctx = Context::init().await?;
    let cli = Cli::parse();
    let res = commands::execute_cmd(&ctx, &cli.command).await?;

    let output: String = match cli.output {
        OutputFormat::Json => res.to_string(),
        OutputFormat::Plain => match cli.command {
            Commands::Info { .. }
            | Commands::Astroport {
                subcmd:
                    AstroportSubcommand::NativeTokens { .. }
                    | AstroportSubcommand::Pair { .. }
                    | AstroportSubcommand::Pool { .. },
            } => json_to_table(&res).collapse().to_string(),
            Commands::List { .. } | Commands::Astroport { .. } => scalar_to_plain(res),
        },
    };
    println!("{}", output);

    Ok(())
}
