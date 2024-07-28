#[cfg(test)]
pub fn assert_sequence<T>(fn_seq: fn(T) -> T, expected: &[T])
where
    T: std::cmp::PartialEq + std::fmt::Debug + Sized + Copy + From<u8>,
{
    for (n, &expected) in expected.iter().enumerate() {
        assert_eq!(fn_seq(T::from(n as u8)), expected);
    }
}
