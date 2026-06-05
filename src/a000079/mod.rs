// Contents: Power of 2
/// https://oeis.org/A000079
/// Power of 2
pub fn a000079<T>(n: T) -> Option<T>
where
    T: num_traits::CheckedShl + num_traits::One + num_traits::ToPrimitive + num_traits::Unsigned,
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
        let expected: [u32; 11] = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];
        assert_sequence(a000079, &expected);
    }

    #[test]
    fn test_a000079_overflow() {
        assert_eq!(a000079::<u8>(8_u8), None);
        assert_eq!(a000079::<u32>(32_u32), None);
    }
}
