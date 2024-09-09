//! Tax calculation, see <https://www.skatteverket.se/foretag/arbetsgivare/arbetsgivaravgifterochskatteavdrag/skattetabeller.4.96cca41179bad4b1aa8a46.html>
pub mod api;
pub mod calculate;
pub mod generated;
pub mod number;
pub mod rules;

pub const CURRENT_YEAR: u16 = 2024;
