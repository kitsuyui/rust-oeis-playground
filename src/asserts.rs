#[cfg(test)]
/// Asserts that `fn_seq(T::try_from(n).unwrap())` equals `expected[n]` for each index `n`.
///
/// # Panics
///
/// Panics if any sequence value does not match, if `fn_seq` returns `None`
/// for an index within the provided slice, or if the index `n` cannot be
/// represented as `T`.
pub fn assert_sequence<T, F>(fn_seq: F, expected: &[T])
where
    T: std::cmp::PartialEq + std::fmt::Debug + Sized + Copy + TryFrom<usize>,
    T::Error: std::fmt::Debug,
    F: Fn(T) -> Option<T>,
{
    for (n, &expected) in expected.iter().enumerate() {
        let index = T::try_from(n).expect("sequence index exceeds type range");
        let result = fn_seq(index).expect("sequence value should not overflow for small n");
        assert_eq!(result, expected);
    }
}
