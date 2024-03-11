use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserLists {
    #[serde(rename = "listId")]
    pub list_id: String,
}

/// See <https://docs.universalis.app/#schema-userlistview>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserListView {
    /// The list's ID.
    pub id: Option<String>,
    /// The time that this list was created, in milliseconds since the UNIX epoch.
    pub created: Option<String>,
    /// The time that this list was updated, in milliseconds since the UNIX epoch.
    pub updated: Option<String>,
    /// The name of this list.
    pub name: Option<String>,
    /// The IDs of the list items.
    #[serde(rename = "itemIDs")]
    pub item_ids: Option<Vec<i32>>,
}
