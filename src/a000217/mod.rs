/// <https://oeis.org/A000217>
///
/// Triangular numbers: `a(n) = n * (n + 1) / 2`.
///
/// `T` must be a primitive integer type. OEIS A000217 is defined only for
/// non-negative integer indices.
///
/// Returns `None` if `n + 1` or the subsequent multiplication overflows `T`.
pub fn a000217<T>(n: T) -> Option<T>
where
    T: num_traits::PrimInt,
{
    let one = T::one();
    let two = one + one;
    let n_plus_one = n.checked_add(&one)?;
    if n % two == T::zero() {
        (n / two).checked_mul(&n_plus_one)
    } else {
        n.checked_mul(&(n_plus_one / two))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asserts::assert_sequence;

    #[test]
    fn test_a000217() {
        let expected = [0, 1, 3, 6, 10, 15, 21, 28, 36, 45, 55];
        assert_sequence(a000217, &expected);
    }

    #[test]
    fn test_a000217_u8_boundary() {
        assert_eq!(a000217(16_u8), Some(136));
        assert_eq!(a000217(22_u8), Some(253));
        assert_eq!(a000217(23_u8), None);
    }
}
