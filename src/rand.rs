use std::cell::Cell;
use std::ops::{Bound, RangeBounds};

thread_local! {
    /// The state of the global random number generator.
    ///
    /// When the value equals `0`, the PRNG has not been initialized yet and its state should not
    /// be used as a seed.
    static RAND_STATE: Cell<u64> = Cell::new(0);
}

/// Generates a pseudo-random `u32` instance.
///
/// This internal function do not support bounds.
fn next_u64() -> u64 {
    RAND_STATE.with(|state| {
        if state.get() == 0 {
            state.set(
                std::time::SystemTime::UNIX_EPOCH
                    .elapsed()
                    .unwrap()
                    .as_nanos() as u64,
            );
        }

        // Credits:
        //  WyRand: https://github.com/wangyi-fudan/wyhash
        state.set(state.get().wrapping_add(0xa0761d6478bd642f));
        let t = (state.get() as u128).wrapping_mul((state.get() ^ 0xe7037ed1a0b428db) as u128);
        (t.wrapping_shr(64) ^ t) as u64
    })
}

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

    let raw = next_u64() as i32;
    if min == i32::MIN && max == i32::MAX {
        raw
    } else {
        let range_size = (max as u32).wrapping_sub(min as u32).wrapping_add(1);
        (raw as u32)
            .wrapping_rem(range_size)
            .wrapping_add(min as u32) as i32
    }
}

#[cfg(test)]
mod random_number {
    use super::random_number;

    #[test]
    fn range() {
        let mut found = [false; 10];

        for _ in 0..1000 {
            found[random_number(0..10) as usize] = true;
        }

        for (i, f) in found.iter().enumerate() {
            assert!(f, "{i} was never generated");
        }
    }

    #[test]
    fn range_inclusive() {
        let mut found = [false; 10];

        for _ in 0..1000 {
            found[random_number(0..10) as usize] = true;
        }

        for (i, f) in found.iter().enumerate() {
            assert!(f, "{i} was never generated");
        }
    }
}
