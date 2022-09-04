use std::ops::Index;

/// Represents the arguments passed to the application.
pub struct Arguments(Box<[Box<str>]>);

impl Arguments {
    /// Creates a new [`Arguments`] instance.
    pub fn from_env() -> Self {
        Self(std::env::args().map(String::into_boxed_str).collect())
    }

    /// Returns the number of arguments that were passed to the application.
    #[inline(always)]
    pub fn count(&self) -> usize {
        self.0.len()
    }

    /// Gets one of the arguments passed to the application.
    ///
    /// If the provided index is out of bounds (greater than [`count`](Arguments::count)), [`None`]
    /// is returned.
    #[inline(always)]
    pub fn get(&self, index: usize) -> Option<&str> {
        self.0.get(index).map(Box::as_ref)
    }
}

impl Index<usize> for Arguments {
    type Output = str;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).unwrap_or_else(|| {
            panic!(
                "tried to access argument {index}, but only {} are available",
                self.count()
            );
        })
    }
}
