use serde::{Deserialize, Serialize};

use super::entities::{ListingView, SaleView};

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
    pub current_average_price: i32,
    /// The average NQ listing price, with outliers removed beyond 3 standard deviations of the mean.
    #[serde(rename = "currentAveragePriceNQ")]
    pub current_average_price_nq: i32,
    /// The average HQ listing price, with outliers removed beyond 3 standard deviations of the mean.
    #[serde(rename = "currentAveragePriceHQ")]
    pub current_average_price_hq: i32,
    /// The average i32 of sales per day, over the past seven days (or the entirety of the shown sales, whichever comes first).
    /// This i32 will tend to be the same for every item, because the i32 of shown sales is the same and over the same period.
    /// This statistic is more useful in historical queries.
    #[serde(rename = "regularSaleVelocity")]
    pub regular_sale_velocity: i32,
    /// The average i32 of NQ sales per day, over the past seven days (or the entirety of the shown sales, whichever comes first).
    /// This i32 will tend to be the same for every item, because the i32 of shown sales is the same and over the same period.
    /// This statistic is more useful in historical queries.
    #[serde(rename = "nqSaleVelocity")]
    pub nq_sale_velocity: i32,
    /// The average i32 of HQ sales per day, over the past seven days (or the entirety of the shown sales, whichever comes first).
    /// This i32 will tend to be the same for every item, because the i32 of shown sales is the same and over the same period.
    /// This statistic is more useful in historical queries.
    #[serde(rename = "hqSaleVelocity")]
    pub hq_sale_velocity: i32,
    /// The average sale price, with outliers removed beyond 3 standard deviations of the mean.
    #[serde(rename = "averagePrice")]
    pub average_price: i32,
    /// The average NQ sale price, with outliers removed beyond 3 standard deviations of the mean.
    #[serde(rename = "averagePriceNQ")]
    pub average_price_nq: i32,
    /// The average HQ sale price, with outliers removed beyond 3 standard deviations of the mean.
    #[serde(rename = "averagePriceHQ")]
    pub average_price_hq: i32,
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
