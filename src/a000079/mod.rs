// Contents: Power of 2
/// <https://oeis.org/A000079>
///
/// Returns the n-th power of 2 (i.e. `2^n`).
///
/// Returns `None` if `n` cannot be converted to `u32` (e.g. the value is
/// negative or exceeds [`u32::MAX`]), or if `2^n` would overflow `T`.
pub fn a000079<T>(n: T) -> Option<T>
where
    T: num_traits::CheckedShl + num_traits::One + num_traits::ToPrimitive,
{
    let shift = n.to_u32()?;
    T::one().checked_shl(shift)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asserts::assert_sequence;

    #[test]
    fn test_a000079() {
        let expected = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];
        assert_sequence(a000079, &expected);
    }

    #[test]
    fn test_a000079_overflow() {
        assert_eq!(a000079::<u8>(8_u8), None);
        assert_eq!(a000079::<u32>(32_u32), None);
    }
}
