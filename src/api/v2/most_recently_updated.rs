use serde::{Deserialize, Serialize};

use super::entities::WorldItemRecencyView;

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
