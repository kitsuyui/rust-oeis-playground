#[cfg(test)]
/// Asserts that `fn_seq(T::from(n as u8))` equals `expected[n]` for each index `n`.
///
/// # Panics
///
/// Panics if any sequence value does not match or if `fn_seq` returns `None`
/// for an index within the provided slice (i.e. unexpected overflow for small `n`).
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
