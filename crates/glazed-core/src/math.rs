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