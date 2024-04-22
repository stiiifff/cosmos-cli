use serde::Deserialize;
use serde_json::Value;
use std::fmt::Display;

use crate::context::Context;

#[derive(Debug, Deserialize)]
pub struct Chain {
    pub name: String,
}

impl Display for Chain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

pub(crate) async fn execute(_ctx: &Context) -> Result<Value, anyhow::Error> {
    Ok(list_chains(_ctx).await?.into())
}

async fn list_chains(ctx: &Context) -> Result<Vec<String>, anyhow::Error> {
    let mut json: Value = ctx.api_get("https://chains.cosmos.directory/").await?;
    let chains_obj = json["chains"].take();
    let chains: Vec<Chain> = serde_json::from_value(chains_obj).unwrap_or_default();
    let chain_names: Vec<String> = chains.into_iter().map(|x| x.name).collect();
    Ok(chain_names)
}
