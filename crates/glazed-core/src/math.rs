use std::convert::TryFrom;

/// Calculates a percentage of a value.
/// Internally, converts all values into a u32, performs the calculation, and converts back to the value type.
/// As long as the numerator is less than the denominator, The type of the original value can remain the same.
/// Proof: Any u16 * (5/6) will fit in a u16
/// In cases were the numerator may greatly exceed the denominator, take caution. Make absolutely sure
/// you won't roll over the original type!
pub fn ratio<F, G>(value: F, num: G, denom: G) -> F
    where F: Into<u32> + TryFrom<u32> + Default, G: Into<u32>
{
    F::try_from(value.into() * num.into() / denom.into()).unwrap_or_default()
}

/// Calculates a percentage of a value.
/// Internally, converts all values into a u32, performs the calculation, and converts back to the value type.
/// As long as the numerator is less than the denominator, The type of the original value can remain the same.
/// Proof: Any u16 * (5/6) will fit in a u16
/// In cases were the numerator may greatly exceed the denominator, take caution. Make absolutely sure
/// you won't roll over the original type!
pub fn fraction<F, G>(value: F, fraction: (G, G)) -> F
    where F: Into<u32> + TryFrom<u32> + Default, G: Into<u32>
{
    let (num, denom) = fraction;
    ratio(value, num, denom)
}

/// Cap a value to a certain max
/// If value > max, max is returned. Else, value is returned
pub fn cap_max<T: Ord>(value: T, max: T) -> T {
    if value > max {
        max
    } else {
        value
    }
}

/// Cap a value to a certain max
/// If value < min, min is returned. Else, value is returned
pub fn cap_min<T: Ord>(value: T, min: T) -> T {
    if value < min {
        min
    } else {
        value
    }
}