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

use crate::{client::Client, concat_endpoint};

/// GET - /api/v2/extra/stats/trade-volume
pub const TRADE_VOLUME: &str = concat_endpoint!("/api/v2/trade-volume");
/// GET - /api/v2/data-centers
pub const AVAILABLE_DATA_CENTERS: &str = concat_endpoint!("/api/v2/data-centers");
/// GET - /api/v2/worlds
pub const AVAILABLE_WORLDS: &str = concat_endpoint!("/api/v2/worlds");
/// GET - /api/v2/extra/content/{contentId}
pub const GAME_ENTITIES: &str = concat_endpoint!("/api/v2/extra/content/{}");
/// GET - /api/v2/extra/stats/least-recently-updated
pub const LEAST_RECENTLY_UPDATED_ITEMS: &str =
    concat_endpoint!("/api/v2/extra/stats/least-recently-updated");
/// GET - /api/v2/{worldDcRegion}/{itemIds}
pub const MARKET_BOARD_CURRENT_DATA: &str = concat_endpoint!("/api/v2/{}/{}");
/// GET - /api/v2/history/{worldDcRegion}/{itemIds}
pub const MARKET_BOARD_SALE_HISTORY: &str = concat_endpoint!("/api/v2/history/{}/{}");
/// GET - /api/v2/tax-rates
pub const MARKET_TAX_RATES: &str = concat_endpoint!("/api/v2/tax-rates");
/// GET - /api/v2/marketable
pub const MARKETABLE_ITEMS: &str = concat_endpoint!("/api/v2/marketable");
/// GET - /api/v2/extra/stats/most-recently-updated
pub const MOST_RECENTLY_UPDATED_ITEMS: &str =
    concat_endpoint!("/api/v2/extra/stats/most-recently-updated");
/// GET - /api/v2/extra/stats/recently-updated
pub const RECENTLY_UPDATED_ITEMS: &str = concat_endpoint!("/api/v2/extra/stats/recently-updated");
/// GET - /api/v2/extra/stats/uploader-upload-counts
pub const UPLOAD_COUNTS_BY_UPLOAD_APPLICATION: &str =
    concat_endpoint!("/api/v2/extra/stats/uploader-upload-counts");
/// GET - /api/v2/extra/stats/world-upload-counts
pub const UPLOAD_COUNTS_BY_WORLD: &str =
    concat_endpoint!("/api/v2/extra/stats/world-upload-counts");
/// GET - /api/v2/extra/stats/upload-history
pub const UPLOADS_PER_DAY: &str = concat_endpoint!("/api/v2/extra/stats/upload-history");
/// GET - /api/v2/lists/{listId}
pub const USER_LISTS: &str = concat_endpoint!("/api/v2/lists/{}");

pub struct ApiV2 {
    pub client: Client,
}

impl ApiV2 {
}
