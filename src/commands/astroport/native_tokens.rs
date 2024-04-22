use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use serde_json::Value;

use super::{
    types::NativeTokentInfo, COIN_REGISTRY_CONTRACT_ADDRESS, COSMWASM_CONTRACT_API,
    COSMWASM_SMART_QUERY, NEUTRON_RPC_URL,
};
use crate::context::Context;

pub(crate) async fn execute(ctx: &Context) -> Result<Value, anyhow::Error> {
    Ok(serde_json::to_value(
        get_astroport_native_tokens_info(ctx).await?,
    )?)
}

async fn get_astroport_native_tokens_info(
    ctx: &Context,
) -> Result<Vec<NativeTokentInfo>, anyhow::Error> {
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
    let assets: Vec<NativeTokentInfo> = serde_json::from_value(assets_obj).unwrap_or_default();
    Ok(assets)
}
