use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use serde_json::{json, Value};

use super::{types::PoolInfo, COSMWASM_CONTRACT_API, COSMWASM_SMART_QUERY, NEUTRON_RPC_URL};
use crate::context::Context;

pub(crate) async fn execute(
    ctx: &Context,
    asset1_denom: &str,
    asset2_denom: &str,
) -> Result<Value, anyhow::Error> {
    match super::pair::get_astroport_pair_info(ctx, asset1_denom, asset2_denom).await? {
        Some(pair_info) => Ok(serde_json::to_value(
            get_astroport_pool_info(ctx, &pair_info.contract_addr).await?,
        )?),
        None => Ok(Value::Null),
    }
}

async fn get_astroport_pool_info(
    ctx: &Context,
    pool_addr: &str,
) -> Result<PoolInfo, anyhow::Error> {
    let base_url = format!(
        "{}/{}/{}/{}",
        NEUTRON_RPC_URL, COSMWASM_CONTRACT_API, pool_addr, COSMWASM_SMART_QUERY,
    );

    let smart_query = URL_SAFE.encode(json!({ "pool": {} }).to_string());
    let mut json: Value = ctx
        .api_get(&format!("{}/{}", base_url, smart_query))
        .await?;
    let pool_obj = json["data"].take();

    let pool: PoolInfo = serde_json::from_value(pool_obj).unwrap_or_default();
    Ok(pool)
}
