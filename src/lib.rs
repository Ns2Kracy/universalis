pub mod api;
pub mod client;
pub mod error;

#[cfg(test)]
mod test {
    #[tokio::test]
    async fn get_trade_volume() {}

    #[tokio::test]
    async fn test_get_available_data_centers() {}

    #[tokio::test]
    async fn get_available_data_centers() {}

    #[tokio::test]
    async fn get_available_worlds() {}

    #[tokio::test]
    async fn get_game_entities() {}

    #[tokio::test]
    async fn get_least_recently_updated_items() {}

    #[tokio::test]
    async fn get_market_board_current_data() {}

    #[tokio::test]
    async fn get_market_board_sale_history() {}

    #[tokio::test]
    async fn get_market_tax_rates() {}

    #[tokio::test]
    async fn get_marketable_items() {}

    #[tokio::test]
    async fn get_most_recently_updated_items() {}

    #[tokio::test]
    async fn get_recently_updated_items() {}

    #[tokio::test]
    async fn get_upload_counts_by_upload_application() {}

    #[tokio::test]
    async fn get_upload_counts_by_world() {}

    #[tokio::test]
    async fn get_uploads_per_day() {}
    
    #[tokio::test]
    async fn get_user_lists() {}
}
