/// (Unstable) Trade volume
/// 
/// GET - /api/v2/extra/stats/trade-volume
///
/// Retrieves the unit trade volume (total units sold)
/// and Gil trade volume (total Gil exchanged) of the specified item over the provided period.
/// Tax is not included in this calculation.
pub struct GetTradeVolume {}
