use self::{
    content::{ContentView, GetGameEntities},
    data_centers::DataCenter,
    least_recently_updated::{GetLeastRecentlyUpdatedItems, LeastRecentlyUpdatedItemsView},
    market_board_current_data::{CurrentlyShownView, GetMarketBoardCurrentData},
    market_board_sale_history::{GetMarketBoardSaleHistory, HistoryMultiViewV2},
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

use super::BASE_URL;

pub mod content;
pub mod data_centers;
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

    /// Available data centers
    ///
    /// GET - /api/v2/data-centers
    ///
    /// Returns the content object associated with the provided content ID.
    /// Please note that this endpoint is largely untested, and may return inconsistent data at times.
    pub async fn get_data_centers(&self) -> anyhow::Result<Vec<DataCenter>> {
        let url = format!("{}/api/v2/data-centers", self.base_url);
        let response = self.client.get(&url).send().await?.json().await?;
        Ok(response)
    }

    /// Available worlds
    ///
    /// GET - /api/v2/worlds
    ///
    /// Returns the IDs and names of all worlds supported by the API.
    pub async fn get_worlds(&self) -> anyhow::Result<Vec<World>> {
        let url = format!("{}/api/v2/worlds", self.base_url);
        let response = self.client.get(&url).send().await?.json().await?;
        Ok(response)
    }

    /// Game entities
    ///
    /// GET - /api/v2/extra/content/{contentId}
    ///
    /// Returns the content object associated with the provided content ID.
    /// Please note that this endpoint is largely untested, and may return inconsistent data at times.
    pub async fn get_game_entities(&self, params: GetGameEntities) -> anyhow::Result<ContentView> {
        let url = format!(
            "{}/api/v2/extra/content/{}",
            self.base_url, params.content_id
        );
        let response = self.client.get(&url).send().await?.json().await?;
        Ok(response)
    }

    /// Least-recently updated items
    ///
    /// GET - /api/v2/extra/stats/least-recently-updated
    ///
    /// Get the least-recently updated items on the specified world or data center,
    /// along with the upload times for each item.
    pub async fn get_least_recently_updated(
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

    /// Market board current data
    ///
    /// GET - /api/v2/{worldDcRegion}/{itemIds}
    ///
    /// Retrieves the data currently shown on the market board for the requested item and world or data center.
    /// Up to 100 item IDs can be comma-separated in order to retrieve data for multiple items at once.
    pub async fn get_market_board_current_data(
        &self,
        params: GetMarketBoardCurrentData,
    ) -> anyhow::Result<CurrentlyShownView> {
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

    /// Upload counts by upload application
    ///
    /// GET - /api/v2/extra/stats/uploader-upload-counts
    ///
    /// Returns the total upload counts for each client application that uploads data to Universalis.
    pub async fn get_uploader_upload_counts(&self) -> anyhow::Result<Vec<SourceUploadCountView>> {
        let url = format!(
            "{}/api/v2/extra/stats/uploader-upload-counts",
            self.base_url
        );
        let response = self.client.get(&url).send().await?.json().await?;
        Ok(response)
    }

    /// Upload counts by world
    ///
    /// GET - /api/v2/extra/stats/world-upload-counts
    ///
    /// Returns the world upload counts and proportions of the total uploads for each world.
    pub async fn get_world_upload_counts(&self) -> anyhow::Result<WorldUploadCountView> {
        let url = format!("{}/api/v2/extra/stats/world-upload-counts", self.base_url);
        let response = self.client.get(&url).send().await?.json().await?;
        Ok(response)
    }

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

    /// User lists
    ///
    /// GET - /api/v2/lists/{listId}
    ///
    /// Retrieves a user list.
    pub async fn get_user_lists(&self, params: GetUserLists) -> anyhow::Result<UserListView> {
        let url = format!("{}/api/v2/lists/{}", self.base_url, params.list_id);
        let response = self.client.get(&url).send().await?.json().await?;
        Ok(response)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn get_trade_volume() {
        let api = UniversalisV2::new();
        let trade_volume = api
            .get_trade_volume(GetTradeVolume {
                world: Some("Crystal".to_string()),
                dc_name: Some("Aether".to_string()),
                item: Some(27814),
                // 2024-01-01 00:00:00
                from: Some(1704038400),
                // 2024-01-02 00:00:00
                to: Some(1704124800),
            })
            .await
            .unwrap();

        println!("{:?}", trade_volume);
    }

    #[tokio::test]
    async fn get_data_centers() {
        let api = UniversalisV2::new();
        let data_centers = api.get_data_centers().await.unwrap();
        println!("{:?}", data_centers);
    }

    #[tokio::test]
    async fn get_worlds() {
        let api = UniversalisV2::new();
        let worlds = api.get_worlds().await.unwrap();
        println!("{:?}", worlds);
    }

    #[tokio::test]
    async fn get_game_entities() {
        let api = UniversalisV2::new();
        let game_entities = api
            .get_game_entities(GetGameEntities {
                content_id: "1".to_string(),
            })
            .await
            .unwrap();
        println!("{:?}", game_entities);
    }

    #[tokio::test]
    async fn get_least_recently_updated() {
        let api = UniversalisV2::new();
        let least_recently_updated = api
            .get_least_recently_updated(GetLeastRecentlyUpdatedItems {
                world: Some("".to_string()),
                dc_name: Some("1172".to_string()),
                entries: Some("".to_string()),
            })
            .await
            .unwrap();
        println!("{:?}", least_recently_updated);
    }

    #[tokio::test]
    async fn get_market_board_current_data() {
        let api = UniversalisV2::new();
        let market_board_current_data = api
            .get_market_board_current_data(GetMarketBoardCurrentData {
                item_ids: "15".to_string(),
                world_dc_region: "1172".to_string(),
                listings: Some("".to_string()),
                entries: Some("".to_string()),
                hq: Some("".to_string()),
                stats_within: Some("".to_string()),
                entries_within: Some("".to_string()),
                fields: Some("".to_string()),
            })
            .await
            .unwrap();
        println!("{:?}", market_board_current_data);
    }

    #[tokio::test]
    async fn get_market_board_sale_history() {
        let api = UniversalisV2::new();
        let market_board_sale_history = api
            .get_market_board_sale_history(GetMarketBoardSaleHistory {
                item_ids: "15".to_string(),
                world_dc_region: "1172".to_string(),
                entries_to_return: Some("".to_string()),
                stats_within: Some("".to_string()),
                entries_within: Some("".to_string()),
                min_sale_price: Some("0".to_string()),
                max_sale_price: Some("2147483647".to_string()),
            })
            .await
            .unwrap();
        println!("{:?}", market_board_sale_history);
    }

    #[tokio::test]
    async fn get_tax_rates() {
        let api = UniversalisV2::new();
        let tax_rates = api
            .get_tax_rates(GetMarketTaxRates {
                world: Some("1172".to_string()),
            })
            .await
            .unwrap();
        println!("{:?}", tax_rates);
    }

    #[tokio::test]
    async fn get_marketable() {
        let api = UniversalisV2::new();
        let marketable = api.get_marketable().await.unwrap();
        println!("{:?}", marketable);
    }

    #[tokio::test]
    async fn get_most_recently_updated() {
        let api = UniversalisV2::new();
        let most_recently_updated = api
            .get_most_recently_updated(GetMostRecentlyUpdatedItems {
                world: Some("".to_string()),
                dc_name: Some("1172".to_string()),
                entries: Some("".to_string()),
            })
            .await
            .unwrap();
        println!("{:?}", most_recently_updated);
    }

    #[tokio::test]
    async fn get_recently_updated() {
        let api = UniversalisV2::new();
        let recently_updated = api.get_recently_updated().await.unwrap();
        println!("{:?}", recently_updated);
    }

    #[tokio::test]
    async fn get_uploader_upload_counts() {
        let api = UniversalisV2::new();
        let uploader_upload_counts = api.get_uploader_upload_counts().await.unwrap();
        println!("{:?}", uploader_upload_counts);
    }

    #[tokio::test]
    async fn get_world_upload_counts() {
        let api = UniversalisV2::new();
        let world_upload_counts = api.get_world_upload_counts().await.unwrap();
        println!("{:?}", world_upload_counts);
    }

    #[tokio::test]
    async fn get_upload_history() {
        let api = UniversalisV2::new();
        let upload_history = api.get_upload_history().await.unwrap();
        println!("{:?}", upload_history);
    }

    #[tokio::test]
    async fn get_user_lists() {
        let api = UniversalisV2::new();
        let user_lists = api
            .get_user_lists(GetUserLists {
                list_id: "1".to_string(),
            })
            .await
            .unwrap();
        println!("{:?}", user_lists);
    }
}
