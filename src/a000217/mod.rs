/// https://oeis.org/A000217
/// Triangular numbers: a(n) = binomial(n+1, 2) = n*(n+1)/2.
pub fn a000217<T>(n: T) -> Option<T>
where
    T: num_traits::CheckedAdd
        + num_traits::CheckedMul
        + std::ops::Div<Output = T>
        + std::ops::Rem<Output = T>
        + std::cmp::PartialEq
        + From<u8>
        + Copy,
{
    let one = T::from(1);
    let two = T::from(2);
    let n_plus_one = n.checked_add(&one)?;
    if n % two == T::from(0) {
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
