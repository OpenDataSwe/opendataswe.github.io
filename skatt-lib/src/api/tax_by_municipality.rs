//! [Per municipality](https://www7.skatteverket.se/portal/apier-och-oppna-data/utvecklarportalen/oppetdata/Skattesatser%20per%20kommun)

use anyhow::Context;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub next: Option<String>,
    pub result_count: i64,
    pub offset: i64,
    pub limit: i64,
    pub query_time: i64,
    pub results: Vec<Result>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    pub kyrkoavgift: String,
    #[serde(rename = "summa, inkl. kyrkoavgift")]
    pub summa_inkl_kyrkoavgift: String,
    #[serde(rename = "församling")]
    pub frsamling: String,
    #[serde(rename = "kommunal-skatt")]
    pub kommunal_skatt: String,
    #[serde(rename = "församlings-kod")]
    pub frsamlings_kod: String,
    pub kommun: String,
    #[serde(rename = "begravnings-avgift")]
    pub begravnings_avgift: String,
    #[serde(rename = "landstings-skatt")]
    pub landstings_skatt: String,
    #[serde(rename = "summa, exkl. kyrkoavgift")]
    pub summa_exkl_kyrkoavgift: String,
    #[serde(rename = "år")]
    pub year: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MunicipalTaxSpecification {
    pub congregation: String,
    pub church_fee: Decimal,
    pub municipal_tax: Decimal,
    pub burial_fee: Decimal,
    pub regional_tax: Decimal,
    pub total: Decimal,
    pub year: u16,
}

impl MunicipalTaxSpecification {
    /// [source](https://www4.skatteverket.se/rattsligvagledning/edition/2024.3/2930.html#h-Kommunal-inkomstskatt)
    #[inline]
    #[must_use]
    pub fn kommunal_inkomstskatt_rate(&self) -> Decimal {
        self.municipal_tax + self.regional_tax
    }

    pub fn convert_from_result(result: &Result) -> anyhow::Result<Self> {
        let church_fee = to_dec_empty_is_zero(&result.kyrkoavgift).with_context(|| {
            format!(
                "Failed to convert church fee={} to a decimal",
                result.kyrkoavgift
            )
        })? / dec!(100);
        let municipal_tax = to_dec_empty_is_zero(&result.kommunal_skatt).with_context(|| {
            format!(
                "Failed to convert municipal tax={} to a decimal",
                result.kommunal_skatt
            )
        })? / dec!(100);
        let burial_fee = to_dec_empty_is_zero(&result.begravnings_avgift).with_context(|| {
            format!(
                "Failed to convert burial fee={} to a decimal",
                result.begravnings_avgift
            )
        })? / dec!(100);
        let regional_tax = to_dec_empty_is_zero(&result.landstings_skatt).with_context(|| {
            format!(
                "Failed to convert regional tax={} to a decimal",
                result.landstings_skatt
            )
        })? / dec!(100);
        let year = result
            .year
            .parse()
            .with_context(|| format!("Failed to parse year={} to a u16", result.year))?;
        let total = result.summa_exkl_kyrkoavgift.parse().with_context(|| {
            format!(
                "Failed to parse total={} to a u16",
                result.summa_exkl_kyrkoavgift
            )
        })?;
        Ok(Self {
            congregation: result.frsamling.clone(),
            church_fee,
            municipal_tax,
            burial_fee,
            regional_tax,
            year,
            total,
        })
    }
}

fn to_dec_empty_is_zero(input: &str) -> anyhow::Result<Decimal> {
    if input.is_empty() {
        Ok(Decimal::ZERO)
    } else {
        input.parse().context("Decimal parse failure")
    }
}
