use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct NativeTokentInfo {
    pub denom: String,
    pub decimals: u8,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct PairInfo {
    pub contract_addr: String,
    pub liquidity_token: String,
    pub pair_type: PairType,
    pub asset_infos: Vec<AssetInfo>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct PoolInfo {
    pub assets: Vec<PoolAssetInfo>,
    pub total_share: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct PairType {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub xyk: Option<XykPair>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stable: Option<StablePair>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<CustomPair>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct XykPair {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct StablePair {}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct CustomPair(String);

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct PoolAssetInfo {
    pub amount: String,
    pub info: AssetInfo,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct AssetInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<Token>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_token: Option<NativeToken>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct NativeToken {
    pub denom: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq, Serialize)]
#[serde(default)]
pub struct Token {
    pub contract_addr: String,
}
