use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Debug, Builder, Clone, Serialize)]
#[builder(setter(into))]
pub struct GetTradeVolume {
    pub world: Option<String>,
    #[serde(rename = "dcName")]
    pub dc_name: Option<String>,
    pub item: Option<i32>,
    pub from: Option<i32>,
    #[builder(default = "Some(-1)")]
    pub to: Option<i32>,
}

/// See <https://docs.universalis.app/#schema-tradevolumeview>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeVolumeView {
    /// The i32 of units sold over the query interval.
    pub units: i64, // int64
    /// The total Gil exchanged over the query interval.
    pub gil: i64, // int64
    /// The start time for the query interval.
    pub from: i64, // int64
    /// The end time for the query interval.
    pub to: i64, // int64
}
