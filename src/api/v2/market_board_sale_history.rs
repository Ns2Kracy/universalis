use serde::{Deserialize, Serialize};

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

/// See <https://docs.universalis.app/#schema-historymultiviewv2>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryMultiViewV2 {
    /// The item IDs that were requested.
    #[serde(rename = "itemIDs")]
    pub item_ids: Option<Vec<i32>>,
    /// The item data that was requested, keyed on the item ID.
    pub items: Option<serde_json::Value>,
    /// The ID of the world requested, if applicable.
    #[serde(rename = "worldID")]
    pub world_id: Option<i32>, // int32
    /// The name of the DC requested, if applicable.
    #[serde(rename = "dcName")]
    pub dc_name: Option<String>,
    /// The name of the region requested, if applicable.
    #[serde(rename = "regionName")]
    pub region_name: Option<String>,
    /// A list of IDs that could not be resolved to any item data.
    #[serde(rename = "unresolvedItems")]
    pub unresolved_items: Option<Vec<i32>>,
    /// The name of the world requested, if applicable.
    #[serde(rename = "worldName")]
    pub world_name: Option<String>,
}

/// See <https://docs.universalis.app/#schema-minimizedsaleview>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimizedSaleView {
    /// Whether or not the item was high-quality.
    pub hq: bool,
    /// The price per unit sold.
    #[serde(rename = "pricePerUnit")]
    pub price_per_unit: i32, // int32
    /// The stack size sold.
    pub quantity: i32, // int32
    /// The buyer's character name. This may be null.
    #[serde(rename = "buyerName")]
    pub buyer_name: Option<String>,
    /// Whether or not this was purchased from a mannequin. This may be null.
    #[serde(rename = "onMannequin")]
    pub on_mannequin: Option<bool>,
    /// The sale time, in seconds since the UNIX epoch.
    pub timestamp: i64, // int64
    /// The world name, if applicable.
    #[serde(rename = "worldName")]
    pub world_name: Option<String>,
    /// The world ID, if applicable.
    #[serde(rename = "worldID")]
    pub world_id: Option<i32>, // int32
}
