use serde::Deserialize;
use serde_json::Value;
use std::fmt::Display;

use crate::context::Context;

#[derive(Debug, Deserialize)]
pub struct Asset {
    pub symbol: String,
}

impl Display for Asset {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.symbol)
    }
}

pub(crate) async fn execute(_ctx: &Context, chain_id: &str) -> Result<Value, anyhow::Error> {
    Ok(list_chain_assets(_ctx, chain_id).await?.into())
}

async fn list_chain_assets(ctx: &Context, chain_id: &str) -> Result<Vec<String>, anyhow::Error> {
    let mut json: Value = ctx
        .api_get(&format!(
            "https://chains.cosmos.directory/{}/assetlist",
            chain_id
        ))
        .await?;
    let assets_obj = json["assets"].take();
    let assets: Vec<Asset> = serde_json::from_value(assets_obj).unwrap_or_default();
    let asset_names: Vec<String> = assets.into_iter().map(|x| x.symbol).collect();
    Ok(asset_names)
}
