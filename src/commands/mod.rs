use serde_json::Value;

use crate::{context::Context, types::Commands};

mod astroport;
mod info;
mod list;

pub(crate) async fn execute_cmd(ctx: &Context, cmd: &Commands) -> Result<Value, anyhow::Error> {
    match cmd {
        Commands::Info { subcmd } => info::execute_subcmd(ctx, subcmd).await,
        Commands::List { subcmd } => list::execute_subcmd(ctx, subcmd).await,
        Commands::Astroport { subcmd } => astroport::execute_subcmd(ctx, subcmd).await,
    }
}
