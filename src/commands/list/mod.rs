use serde_json::Value;

use crate::{context::Context, types::ListSubcommand};

mod assets;
mod chains;
mod paths;

pub(crate) async fn execute_subcmd(
    ctx: &Context,
    subcmd: &ListSubcommand,
) -> Result<Value, anyhow::Error> {
    match subcmd {
        ListSubcommand::Chains => chains::execute(ctx).await,
        ListSubcommand::Assets { chain_id } => assets::execute(ctx, chain_id).await,
        ListSubcommand::Paths => paths::execute(ctx).await,
    }
}
