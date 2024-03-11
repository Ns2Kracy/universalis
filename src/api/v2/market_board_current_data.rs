use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct GetMarketBoardCurrentData {
    #[serde(rename = "itemIds")]
    pub item_ids: String,
    #[serde(rename = "worldDcRegion")]
    pub world_dc_region: String,
    pub listings: Option<String>,
    pub entries: Option<String>,
    pub hq: Option<String>,
    #[serde(rename = "statsWithin")]
    pub stats_within: Option<String>,
    #[serde(rename = "entriesWithin")]
    pub entries_within: Option<String>,
    pub fields: Option<String>,
}

/// See <https://docs.universalis.app/#schema-currentlyshownview>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentlyShownView {
    /// The item ID.
    #[serde(rename = "itemID")]
    pub item_id: i32, // int32
    /// The world ID, if applicable.
    #[serde(rename = "worldID")]
    pub world_id: Option<i32>, // int32
    /// The last upload time for this endpoint, in milliseconds since the UNIX epoch.
    #[serde(rename = "lastUploadTime")]
    pub last_upload_time: i64, // int64
    /// The currently-shown listings.
    pub listings: Option<Vec<ListingView>>,
    /// The currently-shown sales.
    #[serde(rename = "recentHistory")]
    pub recent_history: Option<Vec<SaleView>>,
    /// The DC name, if applicable.
    #[serde(rename = "dcName")]
    pub dc_name: Option<String>,
    /// The region name, if applicable.
    #[serde(rename = "regionName")]
    pub region_name: Option<String>,
    /// The average listing price, with outliers removed beyond 3 standard deviations of the mean.
    #[serde(rename = "currentAveragePrice")]
    pub current_average_price: f32,
    /// The average NQ listing price, with outliers removed beyond 3 standard deviations of the mean.
    #[serde(rename = "currentAveragePriceNQ")]
    pub current_average_price_nq: f32,
    /// The average HQ listing price, with outliers removed beyond 3 standard deviations of the mean.
    #[serde(rename = "currentAveragePriceHQ")]
    pub current_average_price_hq: f32,
    /// The average i32 of sales per day, over the past seven days (or the entirety of the shown sales, whichever comes first).
    /// This i32 will tend to be the same for every item, because the i32 of shown sales is the same and over the same period.
    /// This statistic is more useful in historical queries.
    #[serde(rename = "regularSaleVelocity")]
    pub regular_sale_velocity: f32,
    /// The average i32 of NQ sales per day, over the past seven days (or the entirety of the shown sales, whichever comes first).
    /// This i32 will tend to be the same for every item, because the i32 of shown sales is the same and over the same period.
    /// This statistic is more useful in historical queries.
    #[serde(rename = "nqSaleVelocity")]
    pub nq_sale_velocity: f32,
    /// The average i32 of HQ sales per day, over the past seven days (or the entirety of the shown sales, whichever comes first).
    /// This i32 will tend to be the same for every item, because the i32 of shown sales is the same and over the same period.
    /// This statistic is more useful in historical queries.
    #[serde(rename = "hqSaleVelocity")]
    pub hq_sale_velocity: f32,
    /// The average sale price, with outliers removed beyond 3 standard deviations of the mean.
    #[serde(rename = "averagePrice")]
    pub average_price: f32,
    /// The average NQ sale price, with outliers removed beyond 3 standard deviations of the mean.
    #[serde(rename = "averagePriceNQ")]
    pub average_price_nq: f32,
    /// The average HQ sale price, with outliers removed beyond 3 standard deviations of the mean.
    #[serde(rename = "averagePriceHQ")]
    pub average_price_hq: f32,
    /// The minimum listing price.
    #[serde(rename = "minPrice")]
    pub min_price: i32, // int32
    /// The minimum NQ listing price.
    #[serde(rename = "minPriceNQ")]
    pub min_price_nq: i32, // int32
    /// The minimum HQ listing price.
    #[serde(rename = "minPriceHQ")]
    pub min_price_hq: i32, // int32
    /// The maximum listing price.
    #[serde(rename = "maxPrice")]
    pub max_price: i32, // int32
    /// The maximum NQ listing price.
    #[serde(rename = "maxPriceNQ")]
    pub max_price_nq: i32, // int32
    /// The maximum HQ listing price.
    #[serde(rename = "maxPriceHQ")]
    pub max_price_hq: i32, // int32
    /// A map of quantities to listing counts, representing the i32 of listings of each quantity.
    #[serde(rename = "stackSizeHistogram")]
    pub stack_size_histogram: Option<serde_json::Value>,
    /// A map of quantities to NQ listing counts, representing the i32 of listings of each quantity.
    #[serde(rename = "stackSizeHistogramNQ")]
    pub stack_size_histogram_nq: Option<serde_json::Value>,
    /// A map of quantities to HQ listing counts, representing the i32 of listings of each quantity.
    #[serde(rename = "stackSizeHistogramHQ")]
    pub stack_size_histogram_hq: Option<serde_json::Value>,
    /// The world name, if applicable.
    #[serde(rename = "worldName")]
    pub world_name: Option<String>,
    /// The last upload times in milliseconds since epoch for each world in the response, if this is a DC request.
    #[serde(rename = "worldUploadTimes")]
    pub world_upload_times: Option<serde_json::Value>,
    /// The i32 of listings retrieved for the request. When using the "listings" limit parameter, this may be
    /// different from the i32 of sale entries returned in an API response.
    #[serde(rename = "listingsCount")]
    pub listings_count: i32, // int32
    /// The i32 of sale entries retrieved for the request. When using the "entries" limit parameter, this may be
    /// different from the i32 of sale entries returned in an API response.
    #[serde(rename = "recentHistoryCount")]
    pub recent_history_count: i32, // int32
    /// The i32 of items (not listings) up for sale.
    #[serde(rename = "unitsForSale")]
    pub units_for_sale: i32, // int32
    /// The i32 of items (not sale entries) sold over the retrieved sales.
    #[serde(rename = "unitsSold")]
    pub units_sold: i32, // int32
}

/// See <https://docs.universalis.app/#schema-currentlyshownmultiviewv2>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentlyShownMultiViewV2 {
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

/// See <https://docs.universalis.app/#schema-saleview>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleView {
    /// Whether or not the item was high-quality.
    pub hq: bool,
    /// The price per unit sold.
    #[serde(rename = "pricePerUnit")]
    pub price_per_unit: i32, // int32
    /// The stack size sold.
    pub quantity: i32, // int32
    /// The sale time, in seconds since the UNIX epoch.
    pub timestamp: i64, // int64
    /// Whether or not this was purchased from a mannequin. This may be null.
    #[serde(rename = "onMannequin")]
    pub on_mannequin: Option<bool>,
    /// The world name, if applicable.
    #[serde(rename = "worldName")]
    pub world_name: Option<String>,
    /// The world ID, if applicable.
    #[serde(rename = "worldID")]
    pub world_id: Option<i32>, // int32
    /// The buyer name.
    #[serde(rename = "buyerName")]
    pub buyer_name: Option<String>,
    /// The total price.
    pub total: i32, // int32
}

/// See <https://docs.universalis.app/#schema-listingview>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListingView {
    /// The time that this listing was posted, in seconds since the UNIX epoch.
    #[serde(rename = "lastReviewTime")]
    pub last_review_time: i64, // int64
    /// The price per unit sold.
    #[serde(rename = "pricePerUnit")]
    pub price_per_unit: i32, // int32
    /// The stack size sold.
    pub quantity: i32, // int32
    /// The ID of the dye on this item.
    #[serde(rename = "stainID")]
    pub stain_id: i32, // int32
    /// The world name, if applicable.
    #[serde(rename = "worldName")]
    pub world_name: Option<String>,
    /// The world ID, if applicable.
    #[serde(rename = "worldID")]
    pub world_id: Option<i32>, // int32
    /// The creator's character name.
    #[serde(rename = "creatorName")]
    pub creator_name: Option<String>,
    /// A SHA256 hash of the creator's ID.
    #[serde(rename = "creatorID")]
    pub creator_id: Option<String>,
    /// Whether or not the item is high-quality.
    pub hq: bool,
    /// Whether or not the item is crafted.
    #[serde(rename = "isCrafted")]
    pub is_crafted: bool,
    /// A SHA256 hash of the ID of this listing. Due to some current client-side bugs, this will almost always be null.
    #[serde(rename = "listingID")]
    pub listing_id: Option<String>,
    /// The materia on this item.
    pub materia: Option<Vec<MateriaView>>,
    /// Whether or not the item is being sold on a mannequin.
    #[serde(rename = "onMannequin")]
    pub on_mannequin: bool,
    /// The city ID of the retainer. This is a game ID, all possible values can be seen at
    /// <https://xivapi.com/Town>.
    ///
    /// Limsa Lominsa = 1
    /// Gridania = 2
    /// Ul'dah = 3
    /// Ishgard = 4
    /// Kugane = 7
    /// Crystarium = 10
    /// Old Sharlayan = 12
    #[serde(rename = "retainerCity")]
    pub retainer_city: i32, // int32
    /// A SHA256 hash of the retainer's ID.
    #[serde(rename = "retainerID")]
    pub retainer_id: Option<String>,
    /// The retainer's name.
    #[serde(rename = "retainerName")]
    pub retainer_name: Option<String>,
    /// A SHA256 hash of the seller's ID.
    #[serde(rename = "sellerID")]
    pub seller_id: Option<String>,
    /// The total price.
    pub total: i32, // int32
    /// The Gil sales tax (GST) to be added to the total price during purchase.
    pub tax: i32, // int32
}

/// See <https://docs.universalis.app/#schema-materiaview>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MateriaView {
    /// The materia slot.
    #[serde(rename = "slotID")]
    pub slot_id: i32, // int32
    /// The materia item ID.
    #[serde(rename = "materiaID")]
    pub materia_id: i32, // int32
}
