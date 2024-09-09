use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/// [Source](https://www.skatteverket.se/privat/etjansterochblanketter/svarpavanligafragor/inkomstavtjanst/privattjansteinkomsterfaq/narskamanbetalastatliginkomstskattochhurhogarden.5.10010ec103545f243e8000166.html)
#[must_use]
pub fn state_income_tax_limit() -> Decimal {
    dec!(598_500)
}

/// [Source](https://www.skatteverket.se/privat/etjansterochblanketter/svarpavanligafragor/inkomstavtjanst/privattjansteinkomsterfaq/narskamanbetalastatliginkomstskattochhurhogarden.5.10010ec103545f243e8000166.html)
#[must_use]
pub fn state_income_tax() -> Decimal {
    dec!(0.2)
}
