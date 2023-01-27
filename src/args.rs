use std::cell::UnsafeCell;
use std::mem::MaybeUninit;
use std::sync::atomic::AtomicU8;
use std::sync::atomic::Ordering::*;
use std::{fmt, ops};

// TODO:
//  All of this `OnceCell<T>` nonsense should be replaced by the standard library's when it is
//  stabilized. We might even be able to use standard "lazy" type!

/// Indicates that a [`OnceCell<T>`] is not yet initialized.
const UNINIT: u8 = 0;
/// Indicates that a [`OnceCell<T>`] is currently being initialized.
const IN_PROGRESS: u8 = 1;
/// Indicates that a [`OnceCell<T>`] is initialized.
const INIT: u8 = 2;

/// A minimal implementation of a "OnceCell".
struct OnceCell<T> {
    /// The protected value.
    ///
    /// # Safety
    ///
    /// * If `state` is `UNINIT`, the value is not initialized, but not borrowed in any way.
    ///
    /// * If `state` is `IN_PROGRESS`, the value is not initialized yet, but is currently borrowed
    /// exclusively.
    ///
    ///  * If `state` is `INIT`, the value is initialized, but potentially borrowed.
    value: MaybeUninit<UnsafeCell<T>>,
    /// The internal state of the once cell.
    state: AtomicU8,
}

unsafe impl<T: Send + Sync> Sync for OnceCell<T> {}
unsafe impl<T: Send> Send for OnceCell<T> {}

impl<T> OnceCell<T> {
    /// Creates a new [`OnceCell<T>`].
    pub const fn new() -> Self {
        Self {
            value: MaybeUninit::uninit(),
            state: AtomicU8::new(UNINIT),
        }
    }

    /// Returns the value stored in this [`OnceCell<T>`].
    ///
    /// If the [`OnceCell<T>`] has not been initialized yet, the passed closure is called and its
    /// return value is used to populate the instance.
    pub fn get_or_init<F>(&self, f: F) -> &T
    where
        F: FnOnce() -> T,
    {
        /// In case the `f` function panics, we need to make sure that the previous state is
        /// properly restored.
        struct Guard<'a> {
            /// The state to be restored.
            state: &'a AtomicU8,
            /// The state  to be restored.
            new_state: u8,
        }

        impl<'a> Drop for Guard<'a> {
            fn drop(&mut self) {
                // Restore the state.
                self.state.store(self.new_state, Release);
            }
        }

        loop {
            match self
                .state
                .compare_exchange_weak(UNINIT, IN_PROGRESS, Acquire, Acquire)
            {
                Ok(_) => {
                    // SAFETY:
                    //  The state of the cell is currently `IN_PROGRESS`, meaning that we have
                    //  exclusive access to the value.
                    let slot = unsafe { &mut *self.value.assume_init_ref().get() };

                    let mut guard = Guard {
                        state: &self.state,
                        new_state: UNINIT,
                    };

                    *slot = f();

                    // The function did not panic! The guard must now mark the value as being
                    // initialized.
                    guard.new_state = INIT;

                    break slot;
                }
                Err(INIT) => {
                    // SAFETY:
                    //  The value is already initialized. We can simply return a reference to the
                    //  underlying value.
                    break unsafe { &*self.value.assume_init_ref().get() };
                }
                Err(IN_PROGRESS | UNINIT) => {
                    // The value is currently being initialized by another thread. We just have to
                    // retry sometime later. This branch also takes care of spurious fails of
                    // `compare_exchange_weak`.

                    // NOTE:
                    //  This is basically a spin-loop. It's not ideal, but it will suffice for our
                    //  use-case.
                    std::thread::yield_now();
                }
                Err(_) => unsafe {
                    // SAFETY:
                    //  The `state` can ever only take three values: `INIT`, `IN_PROGRESS` and
                    //  `INCOMPLETE`.
                    std::hint::unreachable_unchecked();
                },
            }
        }
    }
}

/// Represents the arguments passed to the application.
pub struct Args {
    /// The cached arguments.
    ///
    /// The first time those arguments are accessed, this cell is initialized.
    cache: OnceCell<Box<[Box<str>]>>,
}

impl Args {
    /// Creates a new [`Args`] instance.
    const fn new() -> Self {
        Self {
            cache: OnceCell::new(),
        }
    }

    /// Forces the cache of this [`Args`] instance to be populated. The content of the now-complete
    /// cache is returned.
    fn force(&self) -> &[Box<str>] {
        self.cache
            .get_or_init(|| std::env::args().map(String::into_boxed_str).collect())
    }
}

impl fmt::Debug for Args {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self.force(), f)
    }
}

impl ops::Index<usize> for Args {
    type Output = str;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.force()[index]
    }
}

/// The arguments passed to the application.
///
/// # Examples
///
/// ```no_run
/// use ftkit::ARGS;
///
/// println!("{} arguments!", ARGS.len());
/// println!("name of the process: {}", &ARGS[0]);
/// ```
pub static ARGS: Args = Args::new();
