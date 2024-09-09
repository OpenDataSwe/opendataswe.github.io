//! [Grundavdrag under 66](https://www7.skatteverket.se/portal/apier-och-oppna-data/utvecklarportalen/oppetdata/Grundavdrag%20f%C3%B6r%20personer%20under%2066%20%C3%A5r)
//! [Grundavdrag 66 och äldre](https://www7.skatteverket.se/portal/apier-och-oppna-data/utvecklarportalen/oppetdata/Grundavdrag%20f%C3%B6r%20personer%2066%20%C3%A5r%20och%20%C3%A4ldre)

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
    #[serde(rename = "gr avdrag")]
    pub gr_avdrag: String,
    #[serde(rename = "ink from")]
    pub ink_from: String,
    #[serde(rename = "år")]
    pub year: String,
    #[serde(rename = "ink tom")]
    pub ink_tom: String,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BaseDeduction {
    /// None here means same as gross (yearly)
    pub deduction: Option<Decimal>,
    pub income_from: Decimal,
    pub income_to: Decimal,
    pub year: u16,
}

impl BaseDeduction {
    pub fn convert_result_to_base_deduction(value: &Result) -> anyhow::Result<Self> {
        let from: Decimal = value.ink_from.parse().with_context(|| {
            format!(
                "Failed to convert income from={} to a decimal",
                value.ink_from
            )
        })?;
        let to: Decimal = if value.ink_tom.is_empty() {
            dec!(999_999_999)
        } else {
            value.ink_tom.parse().with_context(|| {
                format!("Failed to convert income to={} to a decimal", value.ink_tom)
            })?
        };
        let deduction: Option<Decimal> = if value.gr_avdrag == "FastInk" {
            None
        } else {
            Some(value.gr_avdrag.parse().with_context(|| {
                format!(
                    "Failed to convert base_deduction={} to a decimal",
                    value.gr_avdrag
                )
            })?)
        };
        let year: u16 = value
            .year
            .parse()
            .with_context(|| format!("Failed to convert year={} to a u16", value.year))?;
        Ok(Self {
            deduction,
            income_from: from,
            income_to: to,
            year,
        })
    }
}
