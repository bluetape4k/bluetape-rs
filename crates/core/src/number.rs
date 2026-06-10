use std::fmt::Display;

use crate::error::{RangeKind, ValidationError};

/// Numeric types accepted by validation helpers.
pub trait Number: Copy + PartialOrd + Display {
    /// Returns whether this value can be compared reliably by validation helpers.
    fn is_valid_number(self) -> bool;
}

macro_rules! impl_number {
    ($($type:ty),* $(,)?) => {
        $(
            impl Number for $type {
                fn is_valid_number(self) -> bool {
                    true
                }
            }
        )*
    };
}

impl_number!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
);

impl Number for f32 {
    fn is_valid_number(self) -> bool {
        self.is_finite()
    }
}

impl Number for f64 {
    fn is_valid_number(self) -> bool {
        self.is_finite()
    }
}

/// Returns `Ok(value)` when it is inside the inclusive `[lower, upper]` range.
pub fn require_in_range<T>(
    name: impl Into<String>,
    value: T,
    lower: T,
    upper: T,
) -> Result<T, ValidationError>
where
    T: Number,
{
    let name = name.into();
    require_finite_number(&name, value)?;
    require_finite_number(&name, lower)?;
    require_finite_number(&name, upper)?;
    if lower > upper {
        return Err(invalid_range(name, lower, upper, RangeKind::Inclusive));
    }
    if value < lower || value > upper {
        return Err(out_of_range(
            name,
            value,
            lower,
            upper,
            RangeKind::Inclusive,
        ));
    }
    Ok(value)
}

/// Returns `Ok(value)` when it is inside the half-open `[lower, upper)` range.
pub fn require_in_half_open_range<T>(
    name: impl Into<String>,
    value: T,
    lower: T,
    upper: T,
) -> Result<T, ValidationError>
where
    T: Number,
{
    let name = name.into();
    require_finite_number(&name, value)?;
    require_finite_number(&name, lower)?;
    require_finite_number(&name, upper)?;
    if lower >= upper {
        return Err(invalid_range(name, lower, upper, RangeKind::HalfOpen));
    }
    if value < lower || value >= upper {
        return Err(out_of_range(name, value, lower, upper, RangeKind::HalfOpen));
    }
    Ok(value)
}

/// Returns `Ok(value)` when it is greater than zero.
pub fn require_positive<T>(name: impl Into<String>, value: T) -> Result<T, ValidationError>
where
    T: Number + Default,
{
    let name = name.into();
    require_finite_number(&name, value)?;
    if value <= T::default() {
        Err(ValidationError::NotPositive {
            name,
            value: value.to_string(),
        })
    } else {
        Ok(value)
    }
}

/// Returns `Ok(value)` when it is greater than or equal to zero.
pub fn require_non_negative<T>(name: impl Into<String>, value: T) -> Result<T, ValidationError>
where
    T: Number + Default,
{
    let name = name.into();
    require_finite_number(&name, value)?;
    if value < T::default() {
        Err(ValidationError::Negative {
            name,
            value: value.to_string(),
        })
    } else {
        Ok(value)
    }
}

/// Returns `value` constrained to the inclusive `[lower, upper]` range.
pub fn clamp<T>(value: T, lower: T, upper: T) -> Result<T, ValidationError>
where
    T: Number,
{
    require_finite_number("value", value)?;
    require_finite_number("lower", lower)?;
    require_finite_number("upper", upper)?;
    if lower > upper {
        return Err(invalid_range(
            "range".to_string(),
            lower,
            upper,
            RangeKind::Inclusive,
        ));
    }
    if value < lower {
        Ok(lower)
    } else if value > upper {
        Ok(upper)
    } else {
        Ok(value)
    }
}

fn require_finite_number<T>(name: &str, value: T) -> Result<(), ValidationError>
where
    T: Number,
{
    if value.is_valid_number() {
        Ok(())
    } else {
        Err(ValidationError::NonFinite {
            name: name.to_string(),
            value: value.to_string(),
        })
    }
}

fn invalid_range<T>(name: String, lower: T, upper: T, kind: RangeKind) -> ValidationError
where
    T: Display,
{
    ValidationError::InvalidRange {
        name,
        lower: lower.to_string(),
        upper: upper.to_string(),
        kind,
    }
}

fn out_of_range<T>(name: String, value: T, lower: T, upper: T, kind: RangeKind) -> ValidationError
where
    T: Display,
{
    ValidationError::OutOfRange {
        name,
        value: value.to_string(),
        lower: lower.to_string(),
        upper: upper.to_string(),
        kind,
    }
}
