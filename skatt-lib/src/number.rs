use std::fmt::{Display, Formatter};

/// Doing a bunch of conversions between i32, u32, and f32 with this
/// Requirements are positive and less than 23 bits which
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct MaxNumberValue(i32);

impl MaxNumberValue {
    pub const ZERO: Self = Self(0);
    // 23 bit mantissa of a f32
    pub const MAX: Self = Self(2_i32.pow(23));

    pub fn try_new(val: i32) -> Result<Self, String> {
        if val < 0 {
            Err(format!("Value was negative, val={val}"))
        } else if val > Self::MAX.0 {
            Err(format!("Value was too big, val={val}"))
        } else {
            Ok(Self(val))
        }
    }

    #[inline]
    #[must_use]
    pub const fn const_new(val: i32) -> Self {
        assert!(val >= 0 && val <= Self::MAX.0);
        Self(val)
    }

    #[inline]
    #[must_use]
    #[expect(clippy::cast_sign_loss)]
    pub fn into_usize(self) -> usize {
        self.0 as usize
    }

    #[inline]
    #[must_use]
    #[expect(clippy::cast_sign_loss)]
    pub fn into_u32(self) -> u32 {
        self.0 as u32
    }

    #[inline]
    #[must_use]
    pub fn into_i32(self) -> i32 {
        self.0
    }

    #[inline]
    #[must_use]
    #[expect(clippy::cast_precision_loss)]
    pub fn into_f32(self) -> f32 {
        self.0 as f32
    }
}

impl Display for MaxNumberValue {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

#[expect(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
pub fn f64_to_u32(input: f64) -> Result<u32, String> {
    let rounded = input.round();
    if rounded < 0.0 {
        return Err(format!(
            "Input value={input} cannot be cast to u32, below zero"
        ));
    }
    if rounded > f64::from(u32::MAX) {
        return Err(format!(
            "Input value={input} cannot be cast to u32, too big"
        ));
    }
    Ok(rounded as u32)
}
