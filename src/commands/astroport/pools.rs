use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::context::Context;

const NEUTRON_RPC_URL: &str = "https://rest-kralum.neutron-1.neutron.org";
const FACTORY_CONTRACT_CODE_ID: &str =
    "neutron1hptk0k5kng7hjy35vmh009qd5m6l33609nypgf2yc6nqnewduqasxplt4e";
const COSMWASM_CONTRACT_API: &str = "cosmwasm/wasm/v1/contract";
const COSMWASM_SMART_QUERY: &str = "smart";

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct PoolInfo {
    pub contract_addr: String,
    pub liquidity_token: String,
    pub pair_type: PairType,
    pub asset_infos: Vec<AssetInfo>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct PairType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xyk: Option<XykPair>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stable: Option<StablePair>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<CustomPair>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct XykPair {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct StablePair {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct CustomPair (String);

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct AssetInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    token: Option<Token>,
    #[serde(skip_serializing_if = "Option::is_none")]
    native_token: Option<NativeToken>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct NativeToken {
    pub denom: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Token {
    pub contract_addr: String,
}

pub(crate) async fn execute(ctx: &Context) -> Result<Value, anyhow::Error> {
    get_astroport_pools_info(ctx).await
}

async fn get_astroport_pools_info(ctx: &Context) -> Result<Value, anyhow::Error> {
    let base_url = format!(
        "{}/{}/{}/{}",
        NEUTRON_RPC_URL,
        COSMWASM_CONTRACT_API,
        FACTORY_CONTRACT_CODE_ID,
        COSMWASM_SMART_QUERY,
    );
    let smart_query = URL_SAFE.encode(b"{ \"pairs\": { \"limit\": 30 } }");
    let mut json: Value = ctx
        .api_get(&format!("{}/{}", base_url, smart_query))
        .await?;
    let pools_obj = json["data"].take()["pairs"].take();
    let pools: Vec<PoolInfo> = serde_json::from_value(pools_obj).unwrap_or_default();
    Ok(serde_json::to_value(pools)?)
}
