/// https://oeis.org/A000217
/// Triangular numbers: a(n) = binomial(n+1, 2) = n*(n+1)/2.
pub fn a000217<T>(n: T) -> T
where
    T: std::ops::Add<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + From<u8>
        + Copy,
{
    n * (n + T::from(1)) / T::from(2)
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
}
