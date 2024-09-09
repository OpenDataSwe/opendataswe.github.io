use crate::calculate::under_66_years_old;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/// [source](https://www.skatteverket.se/foretag/arbetsgivare/arbetsgivaravgifterochskatteavdrag/arbetsgivaravgifter.4.233f91f71260075abe8800020817.html)
#[must_use]
pub const fn employer_fee(birth_year: u16) -> Decimal {
    pub const EMPLOYER_FEE_BELOW_66: Decimal = dec!(0.3142);
    pub const EMPLOYER_FEE_66_OR_OVER: Decimal = dec!(0.1021);
    if under_66_years_old(birth_year) {
        EMPLOYER_FEE_BELOW_66
    } else {
        EMPLOYER_FEE_66_OR_OVER
    }
}
