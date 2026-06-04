/// https://oeis.org/A000217
/// Triangular numbers: a(n) = binomial(n+1, 2) = n*(n+1)/2.
pub fn a000217<T>(n: T) -> T
where
    T: std::ops::Add<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Rem<Output = T>
        + std::cmp::PartialEq
        + From<u8>
        + Copy,
{
    let one = T::from(1);
    let two = T::from(2);

    if n % two == T::from(0) {
        (n / two) * (n + one)
    } else {
        n * ((n + one) / two)
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
    fn test_a000217_u8_boundary_without_intermediate_overflow() {
        assert_eq!(a000217(16_u8), 136);
        assert_eq!(a000217(22_u8), 253);
    }
}
