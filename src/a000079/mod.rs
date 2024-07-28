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
    use crate::asserts::assert_sequence;

    #[test]
    fn test_a000079() {
        let expected = [1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024];
        assert_sequence(a000079, &expected);
    }
}
