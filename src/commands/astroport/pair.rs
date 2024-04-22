use base64::{engine::general_purpose::URL_SAFE, Engine as _};
use serde_json::{json, Value};

use super::{
    types::{AssetInfo, PairInfo},
    COSMWASM_CONTRACT_API, COSMWASM_SMART_QUERY, FACTORY_CONTRACT_CODE_ID, NEUTRON_RPC_URL,
    RESULT_LIMIT,
};
use crate::context::Context;

pub(crate) async fn execute(
    ctx: &Context,
    asset1_denom: &str,
    asset2_denom: &str,
) -> Result<Value, anyhow::Error> {
    Ok(serde_json::to_value(
        get_astroport_pair_info(ctx, asset1_denom, asset2_denom).await?,
    )?)
}

pub(crate) async fn get_astroport_pair_info(
    ctx: &Context,
    asset1_denom: &str,
    asset2_denom: &str,
) -> Result<Option<PairInfo>, anyhow::Error> {
    let base_url = format!(
        "{}/{}/{}/{}",
        NEUTRON_RPC_URL, COSMWASM_CONTRACT_API, FACTORY_CONTRACT_CODE_ID, COSMWASM_SMART_QUERY,
    );

    let mut matching_pair: Option<PairInfo>;
    let mut last_pair: Option<Vec<AssetInfo>> = None;
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

        matching_pair = pairs.into_iter().find(|p| {
            let asset1 = asset_denom(p.asset_infos.first().unwrap());
            let asset2 = asset_denom(p.asset_infos.last().unwrap());
            (asset1 == asset1_denom && asset2 == asset2_denom)
                || (asset1 == asset2_denom && asset2 == asset1_denom)
        });
        if matching_pair.is_some() {
            break;
        }

        if res_count < &RESULT_LIMIT {
            break;
        }
    }
    Ok(matching_pair)
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
