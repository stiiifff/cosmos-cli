use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use serde_json::{json, Value};

use super::{
    types::{AssetInfo, PairInfo},
    COSMWASM_CONTRACT_API, COSMWASM_SMART_QUERY, FACTORY_CONTRACT_CODE_ID, NEUTRON_RPC_URL,
    RESULT_LIMIT,
};
use crate::context::Context;

pub(crate) async fn execute(ctx: &Context) -> Result<Value, anyhow::Error> {
    Ok(get_astroport_pairs_info(ctx).await?.into())
}

async fn get_astroport_pairs_info(ctx: &Context) -> Result<Vec<String>, anyhow::Error> {
    let base_url = format!(
        "{}/{}/{}/{}",
        NEUTRON_RPC_URL, COSMWASM_CONTRACT_API, FACTORY_CONTRACT_CODE_ID, COSMWASM_SMART_QUERY,
    );

    let mut last_pair: Option<Vec<AssetInfo>> = None;
    let mut result = vec![];
    loop {
        let smart_query = URL_SAFE.encode(
            json!({
                "pairs": {
                    "start_after": last_pair,
                    "limit": RESULT_LIMIT,
                }
            })
            .to_string(),
        );
        let mut json: Value = ctx
            .api_get(&format!("{}/{}", base_url, smart_query))
            .await?;
        let pairs_obj = json["data"].take()["pairs"].take();

        let pairs: Vec<PairInfo> = serde_json::from_value(pairs_obj).unwrap_or_default();
        last_pair = pairs.last().map(|p| p.asset_infos.clone());
        let res_count = &pairs.len();

        let pairs: Vec<String> = pairs
            .iter()
            .map(|p| {
                format!(
                    "{} {}",
                    asset_denom(p.asset_infos.first().unwrap()),
                    asset_denom(p.asset_infos.last().unwrap())
                )
            })
            .collect();
        result.extend(pairs);

        if res_count < &RESULT_LIMIT {
            break;
        }
    }
    Ok(result)
}

fn asset_denom(asset: &AssetInfo) -> String {
    if let Some(token) = &asset.token {
        token.contract_addr.clone()
    } else if let Some(native_token) = &asset.native_token {
        native_token.denom.clone()
    } else {
        unreachable!("asset_denom: invalid asset")
    }
}
