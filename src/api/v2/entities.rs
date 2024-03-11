use serde::{Deserialize, Serialize};

// TODO: Need to confirm whether Object in the See <https:///docs.universalis.app/ should be serde_json:Value



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

/// See <https://docs.universalis.app/#schema-problemdetails>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemDetails {
    pub r#type: Option<String>,
    pub title: Option<String>,
    pub status: Option<i32>, // int32
    pub detail: Option<String>,
    pub instance: Option<String>,
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



/// See <https://docs.universalis.app/#schema-worlditemrecencyview>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldItemRecencyView {
    /// The item ID.
    #[serde(rename = "itemID")]
    pub item_id: i32, // int32
    /// The last upload time for the item on the listed world.
    #[serde(rename = "lastUploadTime")]
    pub last_upload_time: i32,
    /// The world ID.
    #[serde(rename = "worldID")]
    pub world_id: i32, // int32
    /// The world name.
    #[serde(rename = "worldName")]
    pub world_name: Option<String>,
}
