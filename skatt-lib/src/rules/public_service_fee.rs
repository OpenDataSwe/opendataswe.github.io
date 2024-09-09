use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/// [Rules at SKV](https://www.skatteverket.se/privat/skatter/arbeteochinkomst/askattsedelochskattetabeller/publicserviceavgift.4.22501d9e166a8cb399f31dd.html)
#[must_use]
pub fn public_service_fee(yearly_gross: Decimal) -> Decimal {
    if yearly_gross <= dec!(121919) {
        yearly_gross * dec!(0.01)
    } else {
        dec!(1219)
    }
}
