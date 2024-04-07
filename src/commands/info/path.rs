use anyhow::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::context::Context;

const GIT_REF: &str = "HEAD";
const RAW_FILE_REPO_URL: &str = "https://raw.githubusercontent.com/cosmos/chain-registry";

// Inspired by https://github.com/PeggyJV/chain-registry

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "snake_case")]
pub struct IBCPath {
    pub chain_1: ChainInfo,
    pub chain_2: ChainInfo,
    pub channels: Vec<ChannelInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "snake_case")]
pub struct ChainInfo {
    pub chain_name: String,
    pub client_id: String,
    pub connection_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "snake_case")]
pub struct ChannelInfo {
    pub chain_1: ChannelPort,
    pub chain_2: ChannelPort,
    pub ordering: String,
    pub version: String,
    pub tags: Tags,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "snake_case")]
pub struct ChannelPort {
    pub channel_id: String,
    pub port_id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(default, rename_all = "snake_case")]
pub struct Tags {
    pub dex: String,
    pub preferred: bool,
    pub properties: String,
    pub status: String,
}

/// Represents an IBC path tag
#[allow(unused)]
pub enum Tag {
    Dex(String),
    Preferred(bool),
    Properties(String),
    Status(String),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

pub(crate) async fn execute(_ctx: &Context, chain_a: &str, chain_b: &str) -> Result<Value, Error> {
    let path = format!(
        "_IBC/{}-{}.json",
        chain_a.min(chain_b),
        chain_a.max(chain_b)
    );

    let data = get_file_content(GIT_REF, &path).await?;
    let path: IBCPath = serde_json::from_str(&data).unwrap_or_default();

    Ok(serde_json::to_value(path)?)
}

async fn get_file_content(r#ref: &str, path: &str) -> Result<String, Error> {
    let url = format!("{}/{}/{}", RAW_FILE_REPO_URL, r#ref, path);
    let response = reqwest::get(url).await?; //.text().await?
    response.text().await.map_err(Error::from)
}
