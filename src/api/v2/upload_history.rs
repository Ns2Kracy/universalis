use serde::{Deserialize, Serialize};

/// See <https://docs.universalis.app/#schema-uploadcounthistoryview>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadCountHistoryView {
    /// The list of upload counts per day, over the past 30 days.
    #[serde(rename = "uploadCountByDay")]
    pub upload_count_by_day: Option<Vec<i32>>,
}
