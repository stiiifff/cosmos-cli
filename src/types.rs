use clap::{Parser, Subcommand, ValueEnum};

/// Cosmos CLI
#[derive(Parser)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Output format
    #[arg(global = true, short, long, default_value = "plain")]
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
    /// Get information about Astroport resources
    Astroport {
        #[command(subcommand)]
        subcmd: AstroportSubcommand,
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
    /// Get information about all assets of a specific chain in the Cosmos ecosystem
    Assets {
        /// The chain ID
        chain_id: String,
    },
    /// Get information about a specific asset of a specific chain in the Cosmos ecosystem
    Asset {
        chain_id: String,
        asset_name: String,
    },
    /// Get information about all IBC paths in the Cosmos ecosystem
    Path { chain_a: String, chain_b: String },
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

/// Get information about Astroport resources
#[derive(Subcommand)]
pub(crate) enum AstroportSubcommand {
    /// List native tokens on Astroport
    NativeTokens,
    /// List pairs on Astroport
    Pairs,
    /// Get information about a specific pair on Astroport
    Pair {
        /// Asset 1 denom
        asset1_denom: String,
        /// Asset 2 denom
        asset2_denom: String,
    },
    /// Get information about a specific pool on Astroport
    Pool {
        /// Asset 1 denom
        asset1_denom: String,
        /// Asset 2 denom
        asset2_denom: String,
    },
}
