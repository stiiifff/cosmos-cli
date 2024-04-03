use clap::{Parser, Subcommand};

/// Cosmos CLI
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    // /// Optional name to operate on
    // name: Option<String>,

    // /// Sets a custom config file
    // #[arg(short, long, value_name = "FILE")]
    // config: Option<PathBuf>,

    // /// Turn debugging information on
    // #[arg(short, long, action = clap::ArgAction::Count)]
    // debug: u8,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Get information about resources in the Cosmos ecosystem
    Info {
        #[command(subcommand)]
        subcmd: InfoSubcommand,
    },
    /// List resources in the Cosmos ecosystem
    List {
        #[command(subcommand)]
        subcmd: ListSubcommand,
    },
}

/// Get information about resources in the Cosmos ecosystem
#[derive(Subcommand)]
pub(crate) enum InfoSubcommand {
    /// Get information about a specific chain in the Cosmos ecosystem
    Chain {
        /// The chain ID
        chain_id: String,
    },
}

/// List resources in the Cosmos ecosystem
#[derive(Subcommand)]
pub(crate) enum ListSubcommand {
    /// List all chains in the Cosmos ecosystem
    Chains,
}
