// Contents: Power of 2
/// https://oeis.org/A000079
/// Power of 2
pub fn a000079<T>(n: T) -> T
where
    T: std::ops::Shl<Output = T> + From<u8>,
{
    T::from(1) << n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a000079() {
        assert_eq!(a000079(0), 1);
        assert_eq!(a000079(1), 2);
        assert_eq!(a000079(2), 4);
        assert_eq!(a000079(3), 8);
        assert_eq!(a000079(4), 16);
        assert_eq!(a000079(5), 32);
        assert_eq!(a000079(6), 64);
        assert_eq!(a000079(7), 128);
        assert_eq!(a000079(8), 256);
        assert_eq!(a000079(9), 512);
        assert_eq!(a000079(10), 1024);
    }
}
