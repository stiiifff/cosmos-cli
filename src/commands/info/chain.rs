use anyhow::Error;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::context::Context;

// Inspired by https://github.com/PeggyJV/chain-registry

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct ChainInfo {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub chain_name: String,
    pub status: String,
    pub network_type: String,
    pub pretty_name: String,
    pub chain_id: String,
    pub bech32_prefix: String,
    pub daemon_name: String,
    pub node_home: String,
    pub slip44: u32,
    pub genesis: Genesis,
    pub codebase: Codebase,
    pub peers: Peers,
    pub apis: Apis,
    pub fees: Fees,
    pub staking: Staking,
    pub website: String,
    pub update_link: String,
    pub key_algos: Vec<String>,
    pub explorers: Vec<Explorer>,
    pub name: String,
    pub path: String,
    pub symbol: String,
    pub display: String,
    pub denom: String,
    pub decimals: u8,
    pub coingecko_id: String,
    pub image: String,
    pub height: u64,
    pub best_apis: Apis,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Genesis {
    pub genesis_url: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Codebase {
    pub git_repo: String,
    pub recommended_version: String,
    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::new")]
    pub compatible_versions: Vec<String>,
    pub binaries: Binaries,
    pub cosmos_sdk_version: String,
    pub tendermint_version: String,
    pub cosmwasm_version: String,
    pub cosmwasm_enabled: bool,
    pub ibc_go_version: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Binaries {
    #[serde(rename = "linux/amd64")]
    pub linux_amd_64: String,
    #[serde(rename = "linux/arm64")]
    pub linux_arm_64: String,
    #[serde(rename = "darwin/amd64")]
    pub darwin_amd_64: String,
    #[serde(rename = "darwin/arm64")]
    pub darwin_arm_64: String,
    #[serde(rename = "windows/amd64")]
    pub windows_amd_64: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Peers {
    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::new")]
    pub seeds: Vec<Seed>,
    pub persistent_peers: Vec<PersistentPeer>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Seed {
    pub id: String,
    pub address: String,
    pub provider: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct PersistentPeer {
    pub id: String,
    pub address: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Apis {
    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::new")]
    pub rpc: Vec<Rpc>,
    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::new")]
    pub rest: Vec<Rest>,
    pub grpc: Vec<Grpc>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Rpc {
    pub address: String,
    pub provider: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Rest {
    pub address: String,
    pub provider: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Grpc {
    pub address: String,
    pub provider: Option<String>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Fees {
    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::new")]
    pub fee_tokens: Vec<FeeToken>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct FeeToken {
    pub denom: String,
    pub fixed_min_gas_price: f32,
    pub low_gas_price: f32,
    pub average_gas_price: f32,
    pub high_gas_price: f32,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Staking {
    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::new")]
    pub staking_tokens: Vec<StakingToken>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct StakingToken {
    pub denom: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Explorer {
    pub kind: String,
    pub url: String,
    pub tx_page: String,
    pub account_page: String,
}

pub(crate) async fn execute(ctx: &Context, chain_id: &str) -> Result<Value, Error> {
    Ok(serde_json::to_value(get_chain_info(ctx, chain_id).await?)?)
}

async fn get_chain_info(ctx: &Context, chain_id: &str) -> Result<ChainInfo, Error> {
    let mut json: Value = ctx
        .api_get(&format!("https://chains.cosmos.directory/{}", chain_id))
        .await?;
    let achain_obj = json["chain"].take();
    let chain: ChainInfo = serde_json::from_value(achain_obj).unwrap_or_default();
    Ok(chain)
}
