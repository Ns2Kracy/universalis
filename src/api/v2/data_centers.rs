use serde::{Deserialize, Serialize};



/// See <https://docs.universalis.app/#schema-datacenter>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCenter {
    pub name: Option<String>,
    pub region: Option<String>,
    pub worlds: Option<Vec<i32>>,
}
