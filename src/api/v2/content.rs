use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct GetGameEntities {
    pub content_id: String,
}

/// See <See <https://docs.universalis.app/#schema-contentview>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentView {
    /// The content ID of the object.
    #[serde(rename = "contentID")]
    pub content_id: Option<String>,
    /// The content type of this object.
    #[serde(rename = "contentType")]
    pub content_type: Option<String>,
    /// The character name associated with this character object, if this is one.
    #[serde(rename = "characterName")]
    pub character_name: Option<String>,
}
