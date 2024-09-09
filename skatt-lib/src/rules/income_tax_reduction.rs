use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/// [source](https://www.skatteverket.se/privat/skatter/arbeteochinkomst/skattereduktioner.4.3810a01c150939e893f1a17e.html)
#[must_use]
pub fn income_tax_reduction(yearly_gross: Decimal) -> Decimal {
    const BREAK_POINT: Decimal = dec!(40_000);
    const UPPER_LIMIT: Decimal = dec!(240_000);
    if yearly_gross >= UPPER_LIMIT {
        dec!(1_500)
    } else if yearly_gross >= BREAK_POINT {
        let diff = yearly_gross - BREAK_POINT;
        // 0.75%
        diff * dec!(0.0075)
    } else {
        Decimal::ZERO
    }
}
