#[cfg(test)]
pub fn assert_sequence<T, F>(fn_seq: F, expected: &[T])
where
    T: std::cmp::PartialEq + std::fmt::Debug + Sized + Copy + From<u8>,
    F: Fn(T) -> Option<T>,
{
    for (n, &expected) in expected.iter().enumerate() {
        let result =
            fn_seq(T::from(n as u8)).expect("sequence value should not overflow for small n");
        assert_eq!(result, expected);
    }
}
