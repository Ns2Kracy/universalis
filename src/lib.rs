pub mod api;

pub const BASE_URL: &str = "https://universalis.app";

#[cfg(test)]
mod test {
    use crate::api::v2::UniversalisV2;

    #[tokio::test]
    async fn get_trade_volume() {}

    #[tokio::test]
    async fn get_available_data_centers() {
        let universalis = UniversalisV2::new();
        let data_centers = universalis.get_available_data_centers().await.unwrap();
        println!("{:?}", data_centers);
    }

    #[tokio::test]
    async fn get_available_worlds() {
        let universalis = UniversalisV2::new();
        let worlds = universalis.get_available_worlds().await.unwrap();
        println!("{:?}", worlds);
    }

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
