use anyhow::Error;
use clap::Parser;
use dotenv::dotenv;
use log::{error, info, warn};

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    pretty_env_logger::init();

    info!("such information");
    warn!("o_O");
    error!("much error");

    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name);
    }

    Ok(())
}
