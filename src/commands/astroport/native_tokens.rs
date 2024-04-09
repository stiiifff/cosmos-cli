use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::context::Context;

const NEUTRON_RPC_URL: &str = "https://rest-kralum.neutron-1.neutron.org";
const COIN_REGISTRY_CONTRACT_ADDRESS: &str =
    "neutron1jzzv6r5uckwd64n6qan3suzker0kct5w565f6529zjyumfcx96kqtcswn3";
const COSMWASM_CONTRACT_API: &str = "cosmwasm/wasm/v1/contract";
const COSMWASM_SMART_QUERY: &str = "smart";

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct AssetInfo {
    pub denom: String,
    pub decimals: u8,
}

pub(crate) async fn execute(ctx: &Context) -> Result<Value, anyhow::Error> {
    get_astroport_native_tokens_info(ctx).await
}

async fn get_astroport_native_tokens_info(ctx: &Context) -> Result<Value, anyhow::Error> {
    let query = URL_SAFE.encode(b"{ \"native_tokens\": {} }");
    let mut json: Value = ctx
        .api_get(&format!(
            "{}/{}/{}/{}/{}",
            NEUTRON_RPC_URL,
            COSMWASM_CONTRACT_API,
            COIN_REGISTRY_CONTRACT_ADDRESS,
            COSMWASM_SMART_QUERY,
            query
        ))
        .await?;
    let assets_obj = json["data"].take();
    let assets: Vec<AssetInfo> = serde_json::from_value(assets_obj).unwrap_or_default();
    Ok(serde_json::to_value(assets)?)
}
