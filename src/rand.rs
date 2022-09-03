use std::ops::{Bound, RangeBounds};

/// Generates a random number within the provided bounds.
///
/// # Panics
///
/// This function panics if the provided range is empty. For example, `12..12` is an empty range,
/// but `12..=12` is not.
///
/// # Examples
///
/// ```
/// # use std::ops::RangeBounds;
/// #
/// # macro_rules! assert_matches {
/// #   ($e:expr, $p:pat) => {{
/// #       match $e {
/// #           $p => (),
/// #           val => panic!("assert failed: {val:?} does not match {}", stringify!($p)),
/// #       }
/// #   }}
/// # }
/// assert_matches!(ftkit::random_number(..), i32::MIN..=i32::MAX);
/// assert_matches!(ftkit::random_number(12..15), 12..=14);
/// assert_matches!(ftkit::random_number(-15..=15), -15..=15);
/// assert_eq!(ftkit::random_number(16..=16), 16);
/// assert!(ftkit::random_number(0..) >= 0);
/// ```
pub fn random_number(range: impl RangeBounds<i32>) -> i32 {
    let min = match range.start_bound() {
        Bound::Excluded(&n) => n
            .checked_add(1)
            .expect("can't generate a random number larger than i32::MAX"),
        Bound::Included(&n) => n,
        Bound::Unbounded => i32::MIN,
    };

    let max = match range.end_bound() {
        Bound::Excluded(&n) => n
            .checked_sub(1)
            .expect("can't generate a random number smaller than i32::MIN"),
        Bound::Included(&n) => n,
        Bound::Unbounded => i32::MAX,
    };

    assert!(
        min <= max,
        "can't generate a random number within an empty range"
    );

    let mut bytes = [0u8; 4];
    getrandom::getrandom(&mut bytes).expect("failed to generate a random number");

    if min == i32::MIN && max == i32::MAX {
        i32::from_ne_bytes(bytes)
    } else {
        u32::from_ne_bytes(bytes)
            .wrapping_rem((max as u32).wrapping_sub(min as u32).wrapping_add(1))
            .wrapping_add(min as u32) as i32
    }
}
