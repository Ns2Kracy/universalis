use serde::{Deserialize, Serialize};

pub mod api;

/// See <https://docs.universalis.app/#schema-problemdetails>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemDetails {
    pub r#type: Option<String>,
    pub title: Option<String>,
    pub status: Option<i32>, // int32
    pub detail: Option<String>,
    pub instance: Option<String>,
}
