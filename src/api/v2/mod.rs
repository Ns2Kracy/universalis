use crate::BASE_URL;

use self::{data_centers::DataCenter, trade_volume::GetTradeVolume};

pub mod content;
pub mod data_centers;
pub mod entities;
pub mod least_recently_updated;
pub mod market_board_current_data;
pub mod market_board_sale_history;
pub mod marketable;
pub mod most_recently_updated;
pub mod recently_updated;
pub mod tax_rates;
pub mod trade_volume;
pub mod upload_history;
pub mod uploader_upload_counts;
pub mod user_lists;
pub mod world_upload_counts;
pub mod worlds;

#[derive(Debug, Default)]
pub struct UniversalisV2 {
    pub base_url: String,
    pub client: reqwest::Client,
}
impl UniversalisV2 {
    pub fn new() -> Self {
        Self {
            base_url: BASE_URL.to_string(),
            client: reqwest::Client::new(),
        }
    }

    /// (Unstable) Trade volume
    ///
    /// GET - /api/v2/extra/stats/trade-volume
    ///
    /// Retrieves the unit trade volume (total units sold)
    /// and Gil trade volume (total Gil exchanged) of the specified item over the provided period.
    /// Tax is not included in this calculation.
    pub async fn get_trade_volume(&self, _params: GetTradeVolume) {}

    /// Available data centers
    ///
    /// GET - /api/v2/data-centers
    ///
    /// Returns the content object associated with the provided content ID.
    /// Please note that this endpoint is largely untested, and may return inconsistent data at times.
    pub async fn get_available_data_centers(&self) -> anyhow::Result<Vec<DataCenter>> {
        let url = format!("{}/api/v2/data-centers", self.base_url);
        let response = self.client.get(&url).send().await?.json().await?;
        Ok(response)
    }

    pub async fn get_available_worlds(&self) {}

    pub async fn get_game_entities(&self) {}

    pub async fn get_least_recently_updated_items(&self) {}

    pub async fn get_market_board_current_data(&self) {}

    pub async fn get_market_board_sale_history(&self) {}

    pub async fn get_market_tax_rates(&self) {}

    pub async fn get_marketable_items(&self) {}

    pub async fn get_most_recently_updated_items(&self) {}

    pub async fn get_recently_updated_items(&self) {}

    pub async fn get_upload_counts_by_upload_application(&self) {}

    pub async fn get_upload_counts_by_world(&self) {}

    pub async fn get_uploads_per_day(&self) {}

    pub async fn get_user_lists(&self) {}
}
