use serde_json::Value;

use crate::{context::Context, types::AstroportSubcommand};

const NEUTRON_RPC_URL: &str = "https://rest-kralum.neutron-1.neutron.org";
const COIN_REGISTRY_CONTRACT_ADDRESS: &str =
    "neutron1jzzv6r5uckwd64n6qan3suzker0kct5w565f6529zjyumfcx96kqtcswn3";
const FACTORY_CONTRACT_CODE_ID: &str =
    "neutron1hptk0k5kng7hjy35vmh009qd5m6l33609nypgf2yc6nqnewduqasxplt4e";
const COSMWASM_CONTRACT_API: &str = "cosmwasm/wasm/v1/contract";
const COSMWASM_SMART_QUERY: &str = "smart";
const RESULT_LIMIT: usize = 30;

mod native_tokens;
mod pair;
mod pairs;
mod pool;
mod types;

pub(crate) async fn execute_subcmd(
    ctx: &Context,
    subcmd: &AstroportSubcommand,
) -> Result<Value, anyhow::Error> {
    match subcmd {
        AstroportSubcommand::NativeTokens => native_tokens::execute(ctx).await,
        AstroportSubcommand::Pairs => pairs::execute(ctx).await,
        AstroportSubcommand::Pair {
            asset1_denom,
            asset2_denom,
        } => pair::execute(ctx, asset1_denom, asset2_denom).await,
        AstroportSubcommand::Pool {
            asset1_denom,
            asset2_denom,
        } => pool::execute(ctx, asset1_denom, asset2_denom).await,
    }
}
