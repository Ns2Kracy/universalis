use serde::{Deserialize, Serialize};

/// See <https://docs.universalis.app/#schema-worlduploadcountview>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldUploadCountView {
    /// The i32 of times an upload has occurred on this world.
    pub count: i32,
    /// The proportion of uploads on this world to the total i32 of uploads.
    pub proportion: i32,
}
