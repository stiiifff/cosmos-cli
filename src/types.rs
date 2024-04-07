use clap::{Parser, Subcommand, ValueEnum};

/// Cosmos CLI
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Output format
    #[arg(short, long, default_value = "plain")]
    pub output: OutputFormat,
}

#[derive(ValueEnum, Clone)]
pub enum OutputFormat {
    Plain,
    Json,
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
    Path {
        chain_a: String,
        chain_b: String,
    },
}

/// List resources in the Cosmos ecosystem
#[derive(Subcommand)]
pub(crate) enum ListSubcommand {
    /// List all chains in the Cosmos ecosystem
    Chains,
    /// List assets for a specific chain in the Cosmos ecosystem
    Assets {
        /// The chain ID
        chain_id: String,
    },
    /// List all IBC paths in the Cosmos ecosystem
    Paths,
}
