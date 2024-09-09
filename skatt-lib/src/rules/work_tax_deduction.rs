//! [Jobbskatteavdrag](https://www4.skatteverket.se/rattsligvagledning/edition/2024.3/2940.html)

use crate::api::tax_by_municipality::MunicipalTaxSpecification;
use crate::rules::pbb::pbb;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/// [Should only use the municipal and regional parts](https://www4.skatteverket.se/rattsligvagledning/edition/2024.3/2930.html#h-Kommunal-inkomstskatt)
#[must_use]
pub fn work_tax_deduction_under_66(
    gross_yearly: Decimal,
    base_deduction: Decimal,
    municipal_tax: &MunicipalTaxSpecification,
) -> Decimal {
    let pbb = pbb();
    if gross_yearly < pbb * dec!(0.91) {
        let diff = (gross_yearly - base_deduction).abs();
        diff * municipal_tax.kommunal_inkomstskatt_rate()
    } else if gross_yearly < pbb * dec!(3.24) {
        let s = (gross_yearly - dec!(0.91) * pbb) * dec!(0.3874);
        (dec!(0.91) * pbb + s - base_deduction).abs() * municipal_tax.kommunal_inkomstskatt_rate()
    } else if gross_yearly < pbb * dec!(8.08) {
        let s = (gross_yearly - dec!(3.24) * pbb) * dec!(0.1643);
        (dec!(1.813) * pbb + s - base_deduction).abs() * municipal_tax.kommunal_inkomstskatt_rate()
    } else if gross_yearly < pbb * dec!(13.54) {
        (pbb * dec!(2.608) - base_deduction).abs() * municipal_tax.kommunal_inkomstskatt_rate()
    } else {
        ((pbb * dec!(2.608) - base_deduction).abs() * municipal_tax.kommunal_inkomstskatt_rate()
            - (dec!(0.03) * (gross_yearly - pbb * dec!(13.54))))
        .max(Decimal::ZERO)
    }
}

#[must_use]
pub fn work_tax_deduction_over_66(gross: Decimal) -> Decimal {
    let pbb = pbb();
    if gross < pbb * dec!(1.75) {
        gross * dec!(0.22)
    } else if gross < pbb * dec!(5.24) {
        dec!(0.2635) * pbb + gross * dec!(0.07)
    } else if gross < pbb * dec!(10.48) {
        dec!(0.6293) * pbb
    } else {
        (dec!(0.6293) * pbb + dec!(0.03) - (gross - dec!(10.48) * pbb) * dec!(0.03))
            .max(Decimal::ZERO)
    }
}
