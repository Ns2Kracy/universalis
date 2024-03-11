use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize)]
pub struct GetMarketTaxRates {
    pub world: Option<String>,
}

/// See <https://docs.universalis.app/#schema-taxratesview>
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxRatesView {
    /// The percent retainer tax in Limsa Lominsa.
    #[serde(rename = "Limsa Lominsa")]
    pub limsa_lominsa: i32, // int32
    /// The percent retainer tax in Gridania.
    #[serde(rename = "Ul'dah")]
    pub gridania: i32, // int32
    /// The percent retainer tax in Ul'dah.
    #[serde(rename = "UlDah")]
    pub uldah: i32, // int32
    /// The percent retainer tax in Ishgard.
    #[serde(rename = "Ishgard")]
    pub ishgard: i32, // int32
    /// The percent retainer tax in Kugane.
    #[serde(rename = "Kugane")]
    pub kugane: i32, // int32
    /// The percent retainer tax in the Crystarium.
    #[serde(rename = "Crystarium")]
    pub crystarium: i32, // int32
    /// The percent retainer tax in Old Sharlayan.
    #[serde(rename = "Old Sharlayan")]
    pub old_sharlayan: i32, // int32
}
