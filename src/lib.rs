use core::fmt;
use core::ops::{Deref, DerefMut};

pub use hexfloat2::SupportedFloat;

/// Wraps [`hexfloat2::HexFloat`] with a `Display` impl that always emits an
/// explicit `+` sign in the binary exponent (e.g. `0x1.0p+0` instead of `0x1.0p0`).
#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd, Hash)]
pub struct HexFloat<T>(pub hexfloat2::HexFloat<T>);

pub type HexFloat32 = HexFloat<f32>;
pub type HexFloat64 = HexFloat<f64>;

impl<T> HexFloat<T> {
    pub const fn new(value: T) -> Self {
        Self(hexfloat2::HexFloat::new(value))
    }
}

impl<T> From<T> for HexFloat<T> {
    fn from(value: T) -> Self {
        HexFloat(hexfloat2::HexFloat(value))
    }
}

impl<T> Deref for HexFloat<T> {
    type Target = hexfloat2::HexFloat<T>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for HexFloat<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T: SupportedFloat> fmt::Display for HexFloat<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = format!("{}", self.0);
        // hexfloat2 emits "0x0.0p0" for zero; strip the fractional part.
        let s = s.replace("0x0.0p", "0x0p");
        match s.find('p') {
            Some(p) if !matches!(s.as_bytes().get(p + 1), Some(b'+') | Some(b'-')) => {
                f.write_str(&s[..=p])?;
                f.write_str("+")?;
                f.write_str(&s[p + 1..])
            }
            _ => f.write_str(&s),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positive_exponent_gets_plus() {
        assert_eq!(format!("{}", HexFloat::<f32>::from(1.0f32)), "0x1.0p+0");
        assert_eq!(format!("{}", HexFloat::<f32>::from(2.0f32)), "0x1.0p+1");
        assert_eq!(format!("{}", HexFloat::<f64>::from(1.0f64)), "0x1.0p+0");
    }

    #[test]
    fn negative_exponent_unchanged() {
        assert_eq!(format!("{}", HexFloat::<f32>::from(0.5f32)), "0x1.0p-1");
        assert_eq!(format!("{}", HexFloat::<f64>::from(0.25f64)), "0x1.0p-2");
    }

    #[test]
    fn zero_and_special_values() {
        assert_eq!(format!("{}", HexFloat::<f32>::from(0.0f32)), "0x0p+0");
        assert_eq!(format!("{}", HexFloat::<f32>::from(-0.0f32)), "-0x0p+0");
        assert_eq!(format!("{}", HexFloat::<f32>::from(f32::INFINITY)), "inf");
        assert_eq!(format!("{}", HexFloat::<f32>::from(f32::NAN)), "NaN");
    }
}
