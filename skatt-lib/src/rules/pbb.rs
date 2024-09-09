//! [Prisbasbelopp](https://www.skatteverket.se/privat/skatter/beloppochprocent/2024.4.7da1d2e118be03f8e4f4a88.html#h-Prisbasbelopp)

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

#[must_use]
pub fn pbb() -> Decimal {
    dec!(57300)
}
