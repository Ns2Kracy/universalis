use crate::BASE_URL;

use self::{
    content::{ContentView, GetGameEntities},
    data_centers::DataCenter,
    entities::{CurrentlyShownMultiViewV2, HistoryMultiViewV2},
    least_recently_updated::{GetLeastRecentlyUpdatedItems, LeastRecentlyUpdatedItemsView},
    market_board_current_data::GetMarketBoardCurrentData,
    market_board_sale_history::GetMarketBoardSaleHistory,
    most_recently_updated::{GetMostRecentlyUpdatedItems, MostRecentlyUpdatedItemsView},
    recently_updated::RecentlyUpdatedItemsView,
    tax_rates::{GetMarketTaxRates, TaxRatesView},
    trade_volume::{GetTradeVolume, TradeVolumeView},
    upload_history::UploadCountHistoryView,
    uploader_upload_counts::SourceUploadCountView,
    user_lists::{GetUserLists, UserListView},
    world_upload_counts::WorldUploadCountView,
    worlds::World,
};

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

    // TODO: test
    /// (Unstable) Trade volume
    ///
    /// GET - /api/v2/extra/stats/trade-volume
    ///
    /// Retrieves the unit trade volume (total units sold)
    /// and Gil trade volume (total Gil exchanged) of the specified item over the provided period.
    /// Tax is not included in this calculation.
    pub async fn get_trade_volume(
        &self,
        params: GetTradeVolume,
    ) -> anyhow::Result<TradeVolumeView> {
        let url = format!("{}/api/v2/extra/stats/trade-volume", self.base_url);
        let response = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }

    // TODO: test
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

    // TODO: test
    /// Available worlds
    ///
    /// GET - /api/v2/worlds
    ///
    /// Returns the IDs and names of all worlds supported by the API.
    pub async fn get_available_worlds(&self) -> anyhow::Result<Vec<World>> {
        let url = format!("{}/api/v2/worlds", self.base_url);
        let response = self.client.get(&url).send().await?.json().await?;
        Ok(response)
    }

    // TODO: test
    /// Game entities
    ///
    /// GET - /api/v2/extra/content/{contentId}
    ///
    /// Returns the content object associated with the provided content ID.
    /// Please note that this endpoint is largely untested, and may return inconsistent data at times.
    pub async fn get_game_entities(&self, content: GetGameEntities) -> anyhow::Result<ContentView> {
        let url = format!(
            "{}/api/v2/extra/content/{}",
            self.base_url, content.content_id
        );
        let response = self.client.get(&url).send().await?.json().await?;
        Ok(response)
    }

    // TODO: test
    /// Least-recently updated items
    ///
    /// GET - /api/v2/extra/stats/least-recently-updated
    ///
    /// Get the least-recently updated items on the specified world or data center,
    /// along with the upload times for each item.
    pub async fn get_least_recently_updated_items(
        &self,
        params: GetLeastRecentlyUpdatedItems,
    ) -> anyhow::Result<LeastRecentlyUpdatedItemsView> {
        let url = format!(
            "{}/api/v2/extra/stats/least-recently-updated",
            self.base_url
        );
        let response = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }

    // TODO: test
    /// Market board current data
    ///
    /// GET - /api/v2/{worldDcRegion}/{itemIds}
    ///
    /// Retrieves the data currently shown on the market board for the requested item and world or data center.
    /// Up to 100 item IDs can be comma-separated in order to retrieve data for multiple items at once.
    pub async fn get_market_board_current_data(
        &self,
        params: GetMarketBoardCurrentData,
    ) -> anyhow::Result<Vec<CurrentlyShownMultiViewV2>> {
        let url = format!(
            "{}/api/v2/{}/{}/",
            self.base_url, params.world_dc_region, params.item_ids
        );
        let queries = vec![
            ("listings", params.listings),
            ("entries", params.entries),
            ("hq", params.hq),
            ("statsWithin", params.stats_within),
            ("entriesWithin", params.entries_within),
            ("fields", params.fields),
        ];
        let response = self
            .client
            .get(&url)
            .query(&queries)
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }

    // TODO: test
    /// Market board sale history
    ///
    /// GET - /api/v2/history/{worldDcRegion}/{itemIds}
    ///
    /// Retrieves the history data for the requested item and world or data center.
    /// Up to 100 item IDs can be comma-separated in order to retrieve data for multiple items at once.
    pub async fn get_market_board_sale_history(
        &self,
        params: GetMarketBoardSaleHistory,
    ) -> anyhow::Result<HistoryMultiViewV2> {
        let url = format!(
            "{}/api/v2/history/{}/{}",
            self.base_url, params.world_dc_region, params.item_ids
        );
        let queries = vec![
            ("entriesToReturn", params.entries_to_return),
            ("statsWithin", params.stats_within),
            ("entriesWithin", params.entries_within),
            ("minSalePrice", params.min_sale_price),
            ("maxSalePrice", params.max_sale_price),
        ];
        let response = self
            .client
            .get(&url)
            .query(&queries)
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }

    // TODO: test
    /// Market tax rates
    ///
    /// GET - /api/v2/tax-rates
    ///
    /// Retrieves the current tax rate data for the specified world.
    /// This data is provided by the Retainer Vocate in each major city.
    pub async fn get_tax_rates(&self, params: GetMarketTaxRates) -> anyhow::Result<TaxRatesView> {
        let url = format!("{}/api/v2/tax-rates", self.base_url);
        let response = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }

    // TODO: test
    /// Marketable items
    ///
    /// GET - /api/v2/marketable
    ///
    /// Returns the set of marketable item IDs.
    pub async fn get_marketable(&self) -> anyhow::Result<Vec<i32>> {
        let url = format!("{}/api/v2/marketable", self.base_url);
        let response = self.client.get(&url).send().await?.json().await?;
        Ok(response)
    }

    // TODO: test
    /// Most-recently updated items
    ///
    /// GET - /api/v2/extra/stats/most-recently-updated
    ///
    /// Get the most-recently updated items on the specified world or data center, along with the upload times for each item.
    pub async fn get_most_recently_updated(
        &self,
        params: GetMostRecentlyUpdatedItems,
    ) -> anyhow::Result<MostRecentlyUpdatedItemsView> {
        let url = format!("{}/api/v2/extra/stats/most-recently-updated", self.base_url);
        let response = self
            .client
            .get(&url)
            .query(&params)
            .send()
            .await?
            .json()
            .await?;
        Ok(response)
    }

    // TODO: test
    /// Recently updated items
    ///
    /// GET - /api/v2/extra/stats/recently-updated
    ///
    /// Returns a list of some of the most recently updated items on the website.
    /// This endpoint is a legacy endpoint
    /// and does not include any data on which worlds or data centers the updates happened on.
    pub async fn get_recently_updated(&self) -> anyhow::Result<RecentlyUpdatedItemsView> {
        let url = format!("{}/api/v2/extra/stats/recently-updated", self.base_url);
        let response = self.client.get(&url).send().await?.json().await?;
        Ok(response)
    }

    // TODO: test
    /// Upload counts by upload application
    ///
    /// GET - /api/v2/extra/stats/uploader-upload-counts
    ///
    /// Returns the total upload counts for each client application that uploads data to Universalis.
    pub async fn get_uploader_upload_counts(&self) -> anyhow::Result<SourceUploadCountView> {
        let url = format!(
            "{}/api/v2/extra/stats/uploader-upload-counts",
            self.base_url
        );
        let response = self.client.get(&url).send().await?.json().await?;
        Ok(response)
    }

    // TODO: test
    /// Upload counts by world
    ///
    /// GET - /api/v2/extra/stats/world-upload-counts
    ///
    /// Returns the world upload counts and proportions of the total uploads for each world.
    pub async fn get_world_upload_counts(&self) -> anyhow::Result<Vec<WorldUploadCountView>> {
        let url = format!("{}/api/v2/extra/stats/world-upload-counts", self.base_url);
        let response = self.client.get(&url).send().await?.json().await?;
        Ok(response)
    }

    // TODO: test
    /// Uploads per day
    ///
    /// GET - /api/v2/extra/stats/upload-history
    ///
    /// Returns the number of uploads per day over the past 30 days.
    pub async fn get_upload_history(&self) -> anyhow::Result<UploadCountHistoryView> {
        let url = format!("{}/api/v2/extra/stats/upload-history", self.base_url);
        let response = self.client.get(&url).send().await?.json().await?;
        Ok(response)
    }

    // TODO: test
    /// User lists
    ///
    /// GET - /api/v2/lists/{listId}
    ///
    /// Retrieves a user list.
    pub async fn get_user_lists(&self, list_id: GetUserLists) -> anyhow::Result<UserListView> {
        let url = format!("{}/api/v2/lists/{}", self.base_url, list_id.list_id);
        let response = self.client.get(&url).send().await?.json().await?;
        Ok(response)
    }
}
