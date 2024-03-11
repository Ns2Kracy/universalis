use serde::Serialize;

use super::most_recently_updated::MostRecentlyUpdatedItemsView;

#[derive(Debug, Clone, Serialize)]
pub struct GetLeastRecentlyUpdatedItems {
    pub world: Option<String>,
    #[serde(rename = "dcName")]
    pub dc_name: Option<String>,
    pub entries: Option<String>,
}

pub type LeastRecentlyUpdatedItemsView = MostRecentlyUpdatedItemsView;
