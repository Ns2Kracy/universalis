use serde::{Deserialize, Serialize};

use super::entities::MinimizedSaleView;

#[derive(Debug, Clone, Serialize)]
pub struct GetMarketBoardSaleHistory {
    #[serde(rename = "itemIds")]
    pub item_ids: String,
    #[serde(rename = "worldDcRegion")]
    pub world_dc_region: String,
    #[serde(rename = "entriesToReturn")]
    pub entries_to_return: Option<String>,
    #[serde(rename = "statsWithin")]
    pub stats_within: Option<String>,
    #[serde(rename = "entriesWithin")]
    pub entries_within: Option<String>,
    #[serde(rename = "minSalePrice")]
    pub min_sale_price: Option<String>,
    #[serde(rename = "maxSalePrice")]
    pub max_sale_price: Option<String>,
}

/// See <https://docs.universalis.app/#schema-historyview>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryView {
    /// The item ID.
    #[serde(rename = "itemID")]
    pub item_id: i32, // int32
    /// The world ID, if applicable.
    #[serde(rename = "worldID")]
    pub world_id: Option<i32>, // int32
    /// The last upload time for this endpoint, in milliseconds since the UNIX epoch.
    #[serde(rename = "lastUploadTime")]
    pub last_upload_time: i64, // int64
    /// The historical sales.
    pub entries: Option<Vec<MinimizedSaleView>>,
    /// The DC name, if applicable.
    #[serde(rename = "dcName")]
    pub dc_name: Option<String>,
    /// The region name, if applicable.
    #[serde(rename = "regionName")]
    pub region_name: Option<String>,
    /// A map of quantities to sale counts, representing the i32 of sales of each quantity.
    #[serde(rename = "stackSizeHistogram")]
    pub stack_size_histogram: Option<serde_json::Value>,
    /// A map of quantities to NQ sale counts, representing the i32 of sales of each quantity.
    #[serde(rename = "stackSizeHistogramNQ")]
    pub stack_size_histogram_nq: Option<serde_json::Value>,
    /// A map of quantities to HQ sale counts, representing the i32 of sales of each quantity.
    #[serde(rename = "stackSizeHistogramHQ")]
    pub stack_size_histogram_hq: Option<serde_json::Value>,
    /// The average i32 of sales per day, over the past seven days (or the entirety of the shown sales, whichever comes first).
    #[serde(rename = "regularSaleVelocity")]
    pub regular_sale_velocity: i32,
    /// The average i32 of NQ sales per day, over the past seven days (or the entirety of the shown sales, whichever comes first).
    #[serde(rename = "nqSaleVelocity")]
    pub nq_sale_velocity: i32,
    /// The average i32 of HQ sales per day, over the past seven days (or the entirety of the shown sales, whichever comes first).
    #[serde(rename = "hqSaleVelocity")]
    pub hq_sale_velocity: i32,
    /// The world name, if applicable.
    #[serde(rename = "worldName")]
    pub world_name: Option<String>,
}
