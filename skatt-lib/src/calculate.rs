use crate::api::base_deduction::BaseDeduction;
use crate::api::tax_by_municipality::MunicipalTaxSpecification;
use crate::number::MaxNumberValue;
use crate::rules::employer::employer_fee;
use crate::rules::income_tax_reduction::income_tax_reduction;
use crate::rules::public_service_fee::public_service_fee;
use crate::rules::state_tax::{state_income_tax, state_income_tax_limit};
use crate::rules::work_tax_deduction::{work_tax_deduction_over_66, work_tax_deduction_under_66};
use crate::CURRENT_YEAR;
use anyhow::{anyhow, bail, Context, Result};
use rust_decimal::prelude::ToPrimitive;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::cmp::Ordering;
use std::ops::Range;

#[derive(PartialEq)]
pub struct TaxSeriesCalculation {
    pub step: MaxNumberValue,
    pub taxes: Vec<GrossTaxCalculation>,
}

impl TaxSeriesCalculation {
    #[must_use]
    fn new(count: MaxNumberValue, step: MaxNumberValue) -> Self {
        Self {
            step,
            taxes: Vec::with_capacity(count.into_usize()),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CalculationRange {
    pub start: MaxNumberValue,
    pub step: MaxNumberValue,
    pub end: MaxNumberValue,
}

impl CalculationRange {
    #[must_use]
    pub const fn new(start: MaxNumberValue, step: MaxNumberValue, end: MaxNumberValue) -> Self {
        Self { start, step, end }
    }

    #[inline]
    #[must_use]
    pub fn f32_range(&self) -> Range<f32> {
        self.start.into_f32()..self.end.into_f32()
    }
}

pub fn run_calculation_range(
    calculation_range: &CalculationRange,
    dob: u16,
    include_church: bool,
    municipal_tax_specification: &MunicipalTaxSpecification,
    deductions: &[BaseDeduction],
) -> Result<TaxSeriesCalculation> {
    let mut series = TaxSeriesCalculation::new(calculation_range.end, calculation_range.step);
    for i in (calculation_range.start.into_i32()..=calculation_range.end.into_i32())
        .step_by(calculation_range.step.into_usize())
    {
        series.taxes.push(calculate_single(
            MaxNumberValue::try_new(i)
                .map_err(|e| anyhow!("[BUG] impossible invalid PositiveI32: {e}"))?,
            dob,
            include_church,
            municipal_tax_specification,
            deductions,
        )?);
    }
    Ok(series)
}

pub fn calculate_single(
    gross: MaxNumberValue,
    dob: u16,
    include_church: bool,
    municipal_tax_specification: &MunicipalTaxSpecification,
    deductions: &[BaseDeduction],
) -> Result<GrossTaxCalculation> {
    let gross_monthly = Decimal::from(gross.into_i32());
    let gross_yearly = gross_monthly * dec!(12);
    // Todo: Binary search, it's sorted
    let bd = binary_search_base_deductions(gross_yearly, deductions)?.deduction;

    let base_deduction = bd.unwrap_or(gross_yearly);
    let calc = calculate_by_gross(
        gross_monthly,
        include_church,
        dob,
        municipal_tax_specification,
        base_deduction,
    )?;
    Ok(calc)
}

#[derive(PartialEq)]
pub struct GrossTaxCalculation {
    pub gross_yearly: f32,
    pub public_service_fee: f32,
    pub burial_fee: f32,
    pub church_fee: f32,
    pub regional_tax: f32,
    pub municipal_tax: f32,
    pub state_tax: f32,
    pub employer_fee: f32,
    pub work_tax_deduction: f32,
    pub income_tax_deduction: f32,
}

impl GrossTaxCalculation {
    #[inline]
    #[must_use]
    pub fn gross_monthly(&self) -> f32 {
        self.gross_yearly / 12.0
    }

    #[inline]
    #[must_use]
    pub fn personal_tax(&self) -> f32 {
        self.total_municipal_tax() + self.state_tax + self.public_service_fee
            - self.work_tax_deduction
            - self.income_tax_deduction
    }

    #[inline]
    #[must_use]
    pub fn personal_tax_monthly(&self) -> f32 {
        self.personal_tax() / 12.0
    }

    #[inline]
    #[must_use]
    pub fn personal_tax_percentage(&self) -> f32 {
        let ye = self.gross_yearly;
        if ye == 0.0 {
            0.0
        } else {
            self.personal_tax() / self.gross_yearly
        }
    }
    #[inline]
    #[must_use]
    pub fn total_municipal_tax(&self) -> f32 {
        self.church_fee + self.burial_fee + self.municipal_tax + self.regional_tax
    }

    #[inline]
    #[must_use]
    pub fn total_municipal_tax_monthly(&self) -> f32 {
        self.total_municipal_tax() / 12.0
    }

    #[inline]
    #[must_use]
    pub fn total_tax(&self) -> f32 {
        self.personal_tax() + self.employer_fee
    }

    #[inline]
    #[must_use]
    pub fn total_tax_monthly(&self) -> f32 {
        self.total_tax() / 12.0
    }

    #[inline]
    #[must_use]
    pub fn total_tax_percentage(&self) -> f32 {
        let ye = self.gross_yearly;
        if ye == 0.0 {
            // Should never happen, employer fee as always present
            0.0
        } else {
            self.total_tax() / ye
        }
    }
}

fn calculate_by_gross(
    gross_monthly: Decimal,
    include_church: bool,
    dob: u16,
    municipal_tax_specification: &MunicipalTaxSpecification,
    base_deduction: Decimal,
) -> Result<GrossTaxCalculation> {
    let age = CURRENT_YEAR - dob;
    let yearly_gross = gross_monthly * dec!(12);

    let yearly_gross_after_base_deduction = (yearly_gross - base_deduction).max(Decimal::ZERO);
    if yearly_gross_after_base_deduction <= Decimal::ZERO {
        return Ok(GrossTaxCalculation {
            gross_yearly: yearly_gross
                .to_f32()
                .context("gross yearly not a valid f32")?,
            public_service_fee: 0.0,
            burial_fee: 0.0,
            church_fee: 0.0,
            regional_tax: 0.0,
            municipal_tax: 0.0,
            state_tax: 0.0,
            employer_fee: (yearly_gross * employer_fee(dob))
                .to_f32()
                .context("employer fee not a valid f32")?,
            work_tax_deduction: 0.0,
            income_tax_deduction: 0.0,
        });
    }

    let mut work_tax_deduction = if age < 66 {
        work_tax_deduction_under_66(yearly_gross, base_deduction, municipal_tax_specification)
    } else {
        work_tax_deduction_over_66(yearly_gross)
    };

    let public_service_fee = public_service_fee(yearly_gross_after_base_deduction);
    let burial_fee = yearly_gross_after_base_deduction * municipal_tax_specification.burial_fee;
    let church_fee = if include_church {
        yearly_gross_after_base_deduction * municipal_tax_specification.church_fee
    } else {
        Decimal::ZERO
    };
    let regional_tax = yearly_gross_after_base_deduction * municipal_tax_specification.regional_tax;
    let municipal_tax =
        yearly_gross_after_base_deduction * municipal_tax_specification.municipal_tax;
    let total_municipal = municipal_tax + regional_tax + burial_fee + church_fee;
    if work_tax_deduction > total_municipal {
        work_tax_deduction = total_municipal;
    }

    let income_taxed_with_state_tax =
        (yearly_gross_after_base_deduction - state_income_tax_limit()).max(Decimal::ZERO);
    let state_tax = income_taxed_with_state_tax * state_income_tax();
    let employer_fee = yearly_gross * employer_fee(dob);
    let income_tax_deduction = income_tax_reduction(yearly_gross_after_base_deduction);

    Ok(GrossTaxCalculation {
        gross_yearly: yearly_gross
            .to_f32()
            .context("yearly gross not a valid f32")?,
        public_service_fee: public_service_fee
            .to_f32()
            .context("public service fee not a valid f32")?,
        burial_fee: burial_fee.to_f32().context("burial fee not a valid f32")?,
        church_fee: church_fee.to_f32().context("church fee not a valid f32")?,
        regional_tax: regional_tax
            .to_f32()
            .context("regional tax not a valid f32")?,
        municipal_tax: municipal_tax
            .to_f32()
            .context("municipal tax not a valid f32")?,
        state_tax: state_tax.to_f32().context("state tax not a valid f32")?,
        employer_fee: employer_fee
            .to_f32()
            .context("employer fee not a valid f32")?,
        work_tax_deduction: work_tax_deduction
            .to_f32()
            .context("work tax deduction not a valid f32")?,
        income_tax_deduction: income_tax_deduction
            .to_f32()
            .context("income tax deduction not a valid f32")?,
    })
}

// Approximate the incline, just a simple delta y / delta x,
#[inline]
#[must_use]
pub fn calculate_incline(y1: f32, y2: f32, step: MaxNumberValue) -> f32 {
    (y2 - y1) / (step.into_f32())
}

#[must_use]
pub const fn under_66_years_old(birth_year: u16) -> bool {
    CURRENT_YEAR - 66 < birth_year
}

fn binary_search_base_deductions(
    gross: Decimal,
    deductions: &[BaseDeduction],
) -> Result<&BaseDeduction> {
    let mut search = deductions;
    loop {
        let len = search.len();
        let ind = len / 2;
        let try_deduction = &search[ind];
        match gross_cmp_base_deduction(&gross, try_deduction) {
            Ordering::Less => {
                if ind == 0 {
                    bail!("Failed to find base deduction (reached floor) for gross yearly pay={gross}, search={search:?}");
                }
                search = &search[..len / 2];
            }
            Ordering::Equal => {
                return Ok(try_deduction);
            }
            Ordering::Greater => {
                if ind == 0 && len == 1 {
                    bail!(
                        "Failed to find base deduction (reached ceil) for gross yearly pay={gross}, search={search:?}"
                    );
                }
                search = &search[len / 2..];
            }
        }
    }
}

fn gross_cmp_base_deduction(gross: &Decimal, deduction: &BaseDeduction) -> Ordering {
    // The deductions are discontinuous, there's a gap of 100 between each, going negative isn't
    // a problem here, so just subtract.
    // This does introduce an error on gap, it's impossible to tell if the deduction should be
    // from the previous or next entry, this opts for the next entry.
    if *gross < deduction.income_from - dec!(99.999) {
        Ordering::Less
    } else if gross > &deduction.income_to {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}
