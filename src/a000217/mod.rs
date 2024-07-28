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

    #[test]
    fn test_a000217() {
        assert_eq!(a000217(0), 0);
        assert_eq!(a000217(1), 1);
        assert_eq!(a000217(2), 3);
        assert_eq!(a000217(3), 6);
        assert_eq!(a000217(4), 10);
        assert_eq!(a000217(5), 15);
        assert_eq!(a000217(6), 21);
        assert_eq!(a000217(7), 28);
        assert_eq!(a000217(8), 36);
        assert_eq!(a000217(9), 45);
        assert_eq!(a000217(10), 55);
    }
}
