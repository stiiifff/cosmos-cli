use serde::Deserialize;
use serde_json::Value;

use crate::context::Context;

const GIT_REF: &str = "HEAD";
const REPO_URL: &str = "https://api.github.com/repos/cosmos/chain-registry/contents";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

pub(crate) async fn execute(_ctx: &Context) -> Result<Value, anyhow::Error> {
    Ok(list_paths(_ctx).await?.into())
}

async fn list_paths(ctx: &Context) -> Result<Vec<String>, anyhow::Error> {
    let url = format!("{}/_IBC?ref={}", REPO_URL, GIT_REF,);
    let json: Value = ctx.api_get(&url).await?;
    let contents: Vec<Content> = serde_json::from_value(json).unwrap_or_default();

    Ok(contents
        .iter()
        .filter(|c| c.type_field == "file" && !c.name.starts_with('_') && c.name.ends_with(".json"))
        .map(|c| c.name[..c.name.len() - ".json".len()].to_string())
        .collect())
}
