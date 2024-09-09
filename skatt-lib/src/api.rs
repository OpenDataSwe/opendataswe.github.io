use crate::api::base_deduction::BaseDeduction;
use crate::api::tax_by_municipality::MunicipalTaxSpecification;
use crate::generated::municipalities::Municipalities;
use anyhow::{Context, Result};
use reqwest::{Method, Url};
use rustc_hash::{FxHashMap, FxHashSet};

pub mod base_deduction;
pub mod tax_by_municipality;

#[derive(Clone)]
pub struct Client {
    inner: reqwest::Client,
}

impl Client {
    #[must_use]
    pub fn new() -> Self {
        let inner = reqwest::Client::new();
        Self { inner }
    }

    pub async fn fetch_base_deductions(
        &self,
        check_for_year: u16,
        under_66: bool,
    ) -> Result<Vec<BaseDeduction>> {
        let mut url: Url = if under_66 {
            "https://skatteverket.entryscape.net/rowstore/dataset/ebbd8d70-9b9c-4327-b2ce-a371ee66744c/json".parse()
        } else {
            "https://skatteverket.entryscape.net/rowstore/dataset/43902bf8-cb34-4288-ab95-2080f8a878dc/json".parse()
        }
            .context("Failed to construct base deduction url")?;
        url.query_pairs_mut()
            .append_pair("år", &check_for_year.to_string());
        let mut req: base_deduction::Root = self
            .inner
            .request(Method::GET, url)
            .send()
            .await
            .context("Failed to send request for base deductions")?
            .json()
            .await
            .context("Failed to receive response for base deductions as json")?;
        let mut deductions = Vec::new();
        for res in req.results {
            let bd = BaseDeduction::convert_result_to_base_deduction(&res)?;
            deductions.push(bd);
        }
        while let Some(n) = req.next {
            let next_url: Url = n
                .parse()
                .context("Failed to parse next url for base deduction")?;
            req = self
                .inner
                .request(Method::GET, next_url)
                .send()
                .await
                .context("Failed to send request for base deductions")?
                .json()
                .await
                .context("Failed to receive response for base deductions as json")?;
            for res in req.results {
                let bd = BaseDeduction::convert_result_to_base_deduction(&res)?;
                deductions.push(bd);
            }
        }
        deductions.sort_by_key(|d| d.income_from);

        Ok(deductions)
    }

    pub async fn fetch_tax_for_municipality_by_congregation(
        &self,
        municipality: Municipalities,
        year: u16,
    ) -> Result<FxHashMap<String, MunicipalTaxSpecification>> {
        let mut by_congregation = FxHashMap::default();
        let mut url: Url = "https://skatteverket.entryscape.net/rowstore/dataset/c67b320b-ffee-4876-b073-dd9236cd2a99/json".parse()
            .context("Failed to municipal tax url")?;
        url.query_pairs_mut().append_pair("år", &year.to_string());
        url.query_pairs_mut()
            .append_pair("kommun", municipality.to_request());
        let mut req: tax_by_municipality::Root = self
            .inner
            .request(Method::GET, url)
            .send()
            .await
            .context("Failed to send request for base deductions")?
            .json()
            .await
            .context("Failed to receive response for base deductions as json")?;
        for res in req.results {
            let mts = MunicipalTaxSpecification::convert_from_result(&res)?;
            by_congregation.insert(mts.congregation.clone(), mts);
        }
        while let Some(n) = req.next {
            let next_url: Url = n
                .parse()
                .context("Failed to parse next url for base deduction")?;
            req = self
                .inner
                .request(Method::GET, next_url)
                .send()
                .await
                .context("Failed to send request for base deductions")?
                .json()
                .await
                .context("Failed to receive response for base deductions as json")?;
            for res in req.results {
                let mts = MunicipalTaxSpecification::convert_from_result(&res)?;
                by_congregation.insert(mts.congregation.clone(), mts);
            }
        }

        Ok(by_congregation)
    }

    /// These practically never change, fetch offline and embed into FE
    pub async fn fetch_available_municipalities(&self, year: u16) -> Result<FxHashSet<String>> {
        let mut municipalities = FxHashSet::default();
        let mut url: Url = "https://skatteverket.entryscape.net/rowstore/dataset/c67b320b-ffee-4876-b073-dd9236cd2a99/json".parse()
            .context("Failed to municipal tax url")?;
        url.query_pairs_mut().append_pair("år", &year.to_string());
        let mut req: tax_by_municipality::Root = self
            .inner
            .request(Method::GET, url)
            .send()
            .await
            .context("Failed to send request for base deductions")?
            .json()
            .await
            .context("Failed to receive response for base deductions as json")?;
        for res in req.results {
            municipalities.insert(res.kommun);
        }
        while let Some(n) = req.next {
            let next_url: Url = n
                .parse()
                .context("Failed to parse next url for base deduction")?;
            req = self
                .inner
                .request(Method::GET, next_url)
                .send()
                .await
                .context("Failed to send request for base deductions")?
                .json()
                .await
                .context("Failed to receive response for base deductions as json")?;
            for res in req.results {
                municipalities.insert(res.kommun);
            }
        }

        Ok(municipalities)
    }
}
