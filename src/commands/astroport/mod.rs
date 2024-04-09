use serde_json::Value;

use crate::{context::Context, types::AstroportSubcommand};

mod native_tokens;
mod pools;

pub(crate) async fn execute_subcmd(
    ctx: &Context,
    subcmd: &AstroportSubcommand,
) -> Result<Value, anyhow::Error> {
    match subcmd {
        AstroportSubcommand::NativeTokens => native_tokens::execute(ctx).await,
        AstroportSubcommand::Pools => pools::execute(ctx).await,
    }
}
