use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentView {
    // The content ID of the serde_json::Value.
    pub content_id: Option<String>,
    // The content type of this serde_json::Value.
    pub content_type: Option<String>,
    // The character name associated with this character serde_json::Value, if this is one.
    pub character_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentlyShownMultiViewV2 {
    // The item IDs that were requested.
    pub item_ids: Option<Vec<i32>>,
    // The item data that was requested, keyed on the item ID.
    pub items: Option<serde_json::Value>,
    // The ID of the world requested, if applicable.
    pub world_id: Option<i32>, // int32
    // The name of the DC requested, if applicable.
    pub dc_name: Option<String>,
    // The name of the region requested, if applicable.
    pub region_name: Option<String>,
    // A list of IDs that could not be resolved to any item data.
    pub unresolved_items: Option<Vec<i32>>,
    // The name of the world requested, if applicable.
    pub world_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentlyShownView {
    // The item ID.
    pub item_id: i32, // int32
    // The world ID, if applicable.
    pub world_id: Option<i32>, // int32
    // The last upload time for this endpoint, in milliseconds since the UNIX epoch.
    pub last_upload_time: i64, // int64
    // The currently-shown listings.
    pub listings: Option<Vec<ListingView>>,
    // The currently-shown sales.
    pub recent_history: Option<Vec<SaleView>>,
    // The DC name, if applicable.
    pub dc_name: Option<String>,
    // The region name, if applicable.
    pub region_name: Option<String>,
    // The average listing price, with outliers removed beyond 3 standard deviations of the mean.
    pub current_average_price: i32,
    // The average NQ listing price, with outliers removed beyond 3 standard deviations of the mean.
    pub current_average_price_nq: i32,
    // The average HQ listing price, with outliers removed beyond 3 standard deviations of the mean.
    pub current_average_price_hq: i32,
    // The average i32 of sales per day, over the past seven days (or the entirety of the shown sales, whichever comes first).
    // This i32 will tend to be the same for every item, because the i32 of shown sales is the same and over the same period.
    // This statistic is more useful in historical queries.
    pub regular_sale_velocity: i32,
    // The average i32 of NQ sales per day, over the past seven days (or the entirety of the shown sales, whichever comes first).
    // This i32 will tend to be the same for every item, because the i32 of shown sales is the same and over the same period.
    // This statistic is more useful in historical queries.
    pub nq_sale_velocity: i32,
    // The average i32 of HQ sales per day, over the past seven days (or the entirety of the shown sales, whichever comes first).
    // This i32 will tend to be the same for every item, because the i32 of shown sales is the same and over the same period.
    // This statistic is more useful in historical queries.
    pub hq_sale_velocity: i32,
    // The average sale price, with outliers removed beyond 3 standard deviations of the mean.
    pub average_price: i32,
    // The average NQ sale price, with outliers removed beyond 3 standard deviations of the mean.
    pub average_price_nq: i32,
    // The average HQ sale price, with outliers removed beyond 3 standard deviations of the mean.
    pub average_price_hq: i32,
    // The minimum listing price.
    pub min_price: i32, // int32
    // The minimum NQ listing price.
    pub min_price_nq: i32, // int32
    // The minimum HQ listing price.
    pub min_price_hq: i32, // int32
    // The maximum listing price.
    pub max_price: i32, // int32
    // The maximum NQ listing price.
    pub max_price_nq: i32, // int32
    // The maximum HQ listing price.
    pub max_price_hq: i32, // int32
    // A map of quantities to listing counts, representing the i32 of listings of each quantity.
    pub stack_size_histogram: Option<serde_json::Value>,
    // A map of quantities to NQ listing counts, representing the i32 of listings of each quantity.
    pub stack_size_histogram_nq: Option<serde_json::Value>,
    // A map of quantities to HQ listing counts, representing the i32 of listings of each quantity.
    pub stack_size_histogram_hq: Option<serde_json::Value>,
    // The world name, if applicable.
    pub world_name: Option<String>,
    // The last upload times in milliseconds since epoch for each world in the response, if this is a DC request.
    pub world_upload_times: Option<serde_json::Value>,
    // The i32 of listings retrieved for the request. When using the "listings" limit parameter, this may be
    // different from the i32 of sale entries returned in an API response.
    pub listings_count: i32, // int32
    // The i32 of sale entries retrieved for the request. When using the "entries" limit parameter, this may be
    // different from the i32 of sale entries returned in an API response.
    pub recent_history_count: i32, // int32
    // The i32 of items (not listings) up for sale.
    pub units_for_sale: i32, // int32
    // The i32 of items (not sale entries) sold over the retrieved sales.
    pub units_sold: i32, // int32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCenter {
    pub name: Option<String>,
    pub region: Option<String>,
    pub worlds: Option<Vec<i32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryMultiViewV2 {
    // The item IDs that were requested.
    pub item_ids: Option<Vec<i32>>,
    // The item data that was requested, keyed on the item ID.
    pub items: Option<serde_json::Value>,
    // The ID of the world requested, if applicable.
    pub world_id: Option<i32>, // int32
    // The name of the DC requested, if applicable.
    pub dc_name: Option<String>,
    // The name of the region requested, if applicable.
    pub region_name: Option<String>,
    // A list of IDs that could not be resolved to any item data.
    pub unresolved_items: Option<Vec<i32>>,
    // The name of the world requested, if applicable.
    pub world_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryView {
    // The item ID.
    pub item_id: i32, // int32
    // The world ID, if applicable.
    pub world_id: Option<i32>, // int32
    // The last upload time for this endpoint, in milliseconds since the UNIX epoch.
    pub last_upload_time: i64, // int64
    // The historical sales.
    pub entries: Option<Vec<MinimizedSaleView>>,
    // The DC name, if applicable.
    pub dc_name: Option<String>,
    // The region name, if applicable.
    pub region_name: Option<String>,
    // A map of quantities to sale counts, representing the i32 of sales of each quantity.
    pub stack_size_histogram: Option<serde_json::Value>,
    // A map of quantities to NQ sale counts, representing the i32 of sales of each quantity.
    pub stack_size_histogram_nq: Option<serde_json::Value>,
    // A map of quantities to HQ sale counts, representing the i32 of sales of each quantity.
    pub stack_size_histogram_hq: Option<serde_json::Value>,
    // The average i32 of sales per day, over the past seven days (or the entirety of the shown sales, whichever comes first).
    pub regular_sale_velocity: i32,
    // The average i32 of NQ sales per day, over the past seven days (or the entirety of the shown sales, whichever comes first).
    pub nq_sale_velocity: i32,
    // The average i32 of HQ sales per day, over the past seven days (or the entirety of the shown sales, whichever comes first).
    pub hq_sale_velocity: i32,
    // The world name, if applicable.
    pub world_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListingView {
    // The time that this listing was posted, in seconds since the UNIX epoch.
    pub last_review_time: i64, // int64
    // The price per unit sold.
    pub price_per_unit: i32, // int32
    // The stack size sold.
    pub quantity: i32, // int32
    // The ID of the dye on this item.
    pub stain_id: i32, // int32
    // The world name, if applicable.
    pub world_name: Option<String>,
    // The world ID, if applicable.
    pub world_id: Option<i32>, // int32
    // The creator's character name.
    pub creator_name: Option<String>,
    // A SHA256 hash of the creator's ID.
    pub creator_id: Option<String>,
    // Whether or not the item is high-quality.
    pub hq: bool,
    // Whether or not the item is crafted.
    pub is_crafted: bool,
    // A SHA256 hash of the ID of this listing. Due to some current client-side bugs, this will almost always be null.
    pub listing_id: Option<String>,
    // The materia on this item.
    pub materia: Option<Vec<MateriaView>>,
    // Whether or not the item is being sold on a mannequin.
    pub on_mannequin: bool,
    // The city ID of the retainer. This is a game ID, all possible values can be seen at
    // https://xivapi.com/Town.
    //
    // Limsa Lominsa = 1
    // Gridania = 2
    // Ul'dah = 3
    // Ishgard = 4
    // Kugane = 7
    // Crystarium = 10
    // Old Sharlayan = 12
    pub retainer_city: i32, // int32
    // A SHA256 hash of the retainer's ID.
    pub retainer_id: Option<String>,
    // The retainer's name.
    pub retainer_name: Option<String>,
    // A SHA256 hash of the seller's ID.
    pub seller_id: Option<String>,
    // The total price.
    pub total: i32, // int32
    // The Gil sales tax (GST) to be added to the total price during purchase.
    pub tax: i32, // int32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MateriaView {
    // The materia slot.
    pub slot_id: i32, // int32
    // The materia item ID.
    pub materia_id: i32, // int32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinimizedSaleView {
    // Whether or not the item was high-quality.
    pub hq: bool,
    // The price per unit sold.
    pub price_per_unit: i32, // int32
    // The stack size sold.
    pub quantity: i32, // int32
    // The buyer's character name. This may be null.
    pub buyer_name: Option<String>,
    // Whether or not this was purchased from a mannequin. This may be null.
    pub on_mannequin: Option<bool>,
    // The sale time, in seconds since the UNIX epoch.
    pub timestamp: i64, // int64
    // The world name, if applicable.
    pub world_name: Option<String>,
    // The world ID, if applicable.
    pub world_id: Option<i32>, // int32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MostRecentlyUpdatedItemsView {
    // A list of item upload information in timestamp-descending order.
    pub items: Option<Vec<WorldItemRecencyView>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemDetails {
    pub r#type: Option<String>,
    pub title: Option<String>,
    pub status: Option<i32>, // int32
    pub detail: Option<String>,
    pub instance: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentlyUpdatedItemsView {
    // Whether or not the item was high-quality.
    pub hq: bool,
    // The price per unit sold.
    pub price_per_unit: i32, // int32
    // The stack size sold.
    pub quantity: i32, // int32
    // The sale time, in seconds since the UNIX epoch.
    pub timestamp: i64, // int64
    // Whether or not this was purchased from a mannequin. This may be null.
    pub on_mannequin: Option<bool>,
    // The world name, if applicable.
    pub world_name: Option<String>,
    // The world ID, if applicable.
    pub world_id: Option<i32>, // int32
    // The buyer name.
    pub buyer_name: Option<String>,
    // The total price.
    pub total: i32, // int32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaleView {
    // The name of the client application.
    pub source_name: Option<String>,
    // The i32 of uploads originating from the client application.
    pub upload_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceUploadCountView {
    // The percent retainer tax in Limsa Lominsa.
    pub limsa_lominsa: i32, // int32
    // The percent retainer tax in Gridania.
    pub gridania: i32, // int32
    // The percent retainer tax in Ul'dah.
    pub uldah: i32, // int32
    // The percent retainer tax in Ishgard.
    pub ishgard: i32, // int32
    // The percent retainer tax in Kugane.
    pub kugane: i32, // int32
    // The percent retainer tax in the Crystarium.
    pub crystarium: i32, // int32
    // The percent retainer tax in Old Sharlayan.
    pub old_sharlayan: i32, // int32
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxRatesView {
    // The i32 of units sold over the query interval.
    pub units: i64, // int64
    // The total Gil exchanged over the query interval.
    pub gil: i64, // int64
    // The start time for the query interval.
    pub from: i64, // int64
    // The end time for the query interval.
    pub to: i64, // int64
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeVolumeView {
    // The list of upload counts per day, over the past 30 days.
    pub upload_count_by_day: Option<Vec<i32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadCountHistoryView {
    // The list of upload counts per day, over the past 30 days.
    pub upload_count_by_day: Option<Vec<i32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserListView {
    // The list's ID.
    pub id: Option<String>,
    // The time that this list was created, in milliseconds since the UNIX epoch.
    pub created: Option<String>,
    // The time that this list was updated, in milliseconds since the UNIX epoch.
    pub updated: Option<String>,
    // The name of this list.
    pub name: Option<String>,
    // The IDs of the list items.
    pub item_ids: Option<Vec<i32>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub id: i32, // int32
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldItemRecencyView {
    // The item ID.
    pub item_id: i32, // int32
    // The last upload time for the item on the listed world.
    pub last_upload_time: i32,
    // The world ID.
    pub world_id: i32, // int32
    // The world name.
    pub world_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldUploadCountView {
    // The i32 of times an upload has occurred on this world.
    pub count: i32,
    // The proportion of uploads on this world to the total i32 of uploads.
    pub proportion: i32,
}
