use serde::{Deserialize, Serialize};

/// See <https://docs.universalis.app/#schema-sourceuploadcountview>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceUploadCountView {
    /// The name of the client application.
    #[serde(rename = "sourceName")]
    pub source_name: Option<String>,
    /// The i32 of uploads originating from the client application.
    #[serde(rename = "uploadCount")]
    pub upload_count: i32,
}
