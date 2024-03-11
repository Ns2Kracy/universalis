use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct GetMostRecentlyUpdatedItems {
    pub world: Option<String>,
    #[serde(rename = "dcName")]
    pub dc_name: Option<String>,
    pub entries: Option<String>,
}

/// See <https://docs.universalis.app/#schema-mostrecentlyupdateditemsview>
#[derive(Debug, Clone, Deserialize)]
pub struct MostRecentlyUpdatedItemsView {
    /// A list of item upload information in timestamp-descending order.
    pub items: Option<Vec<WorldItemRecencyView>>,
}

/// See <https://docs.universalis.app/#schema-worlditemrecencyview>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldItemRecencyView {
    /// The item ID.
    #[serde(rename = "itemID")]
    pub item_id: i32, // int32
    /// The last upload time for the item on the listed world.
    #[serde(rename = "lastUploadTime")]
    pub last_upload_time: i64,
    /// The world ID.
    #[serde(rename = "worldID")]
    pub world_id: i32, // int32
    /// The world name.
    #[serde(rename = "worldName")]
    pub world_name: Option<String>,
}
