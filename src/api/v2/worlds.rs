use serde::{Deserialize, Serialize};

pub struct GetAvailableWorlds {}

/// See <https://docs.universalis.app/#schema-world>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct World {
    pub id: i32, // int32
    pub name: Option<String>,
}
