use serde_json::Value;

use crate::{context::Context, types::InfoSubcommand};

mod assets;
mod chain;
mod path;

pub(crate) async fn execute_subcmd(
    ctx: &Context,
    subcmd: &InfoSubcommand,
) -> Result<Value, anyhow::Error> {
    match subcmd {
        InfoSubcommand::Chain { chain_id } => chain::execute(ctx, chain_id).await,
        InfoSubcommand::Assets { chain_id } => assets::execute(ctx, chain_id, None).await,
        InfoSubcommand::Asset {
            chain_id,
            asset_name,
        } => assets::execute(ctx, chain_id, Some(asset_name)).await,
        InfoSubcommand::Path { chain_a, chain_b } => path::execute(ctx, chain_a, chain_b).await,
    }
}
