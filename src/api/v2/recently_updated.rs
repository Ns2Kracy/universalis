use serde::{Deserialize, Serialize};

/// See <https://docs.universalis.app/#schema-recentlyupdateditemsview>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentlyUpdatedItemsView {
    /// A list of item IDs, with the most recent first.
    items: Option<Vec<i32>>,
}
