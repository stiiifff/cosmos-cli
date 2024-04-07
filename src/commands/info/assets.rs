use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::context::Context;

// Inspired by https://github.com/PeggyJV/chain-registry

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct AssetInfo {
    pub name: String,
    pub description: String,
    pub symbol: String,
    pub denom: String,
    pub decimals: u8,
    pub coingecko_id: String,
    pub base: String,
    pub display: String,
    pub denom_units: Vec<DenomUnit>,
    #[serde(rename = "logo_URIs")]
    pub logo_uris: LogoURIs,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct DenomUnit {
    pub denom: String,
    pub exponent: u16,
    pub aliases: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct LogoURIs {
    pub png: String,
    pub svg: String,
}

pub(crate) async fn execute(
    _ctx: &Context,
    chain_id: &str,
    asset_name: Option<&str>,
) -> Result<Value, anyhow::Error> {
    get_chain_assets_info(_ctx, chain_id, asset_name).await
}

async fn get_chain_assets_info(
    ctx: &Context,
    chain_id: &str,
    asset_name: Option<&str>,
) -> Result<Value, anyhow::Error> {
    let mut json: Value = ctx
        .api_get(&format!(
            "https://chains.cosmos.directory/{}/assetlist",
            chain_id
        ))
        .await?;
    let assets_obj = json["assets"].take();
    let assets: Vec<AssetInfo> = serde_json::from_value(assets_obj).unwrap_or_default();
    if let Some(asset_name) = asset_name {
        let asset = assets.iter().find(|asset| {
            asset.name == asset_name
                || asset.symbol == asset_name
                || asset.denom == asset_name
                || asset.display == asset_name
                || asset.base == asset_name
        });
        if let Some(asset) = asset {
            return Ok(serde_json::to_value(asset)?);
        } else {
            return Err(anyhow::anyhow!("Asset not found"));
        }
    }
    Ok(serde_json::to_value(assets)?)
}
