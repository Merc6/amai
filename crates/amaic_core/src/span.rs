//! See [`Span`].

use std::ops::Range;

use derive_more::Constructor;

/// A byte-range used for representing a position in source-code.
#[derive(Clone, Copy, Constructor, Debug, Default, PartialEq, Eq)]
#[allow(
    // `allow` used here in place of `expect`, because it doesn't play nice
    // with `Constructor`.
    clippy::arbitrary_source_item_ordering,
    reason = "Intuitively, start comes first."
)]
pub struct Span {
    /// The start of the span.
    start: usize,

    /// The end of the span.
    end: usize,
}

impl Span {
    /// The end of the byte-range.
    #[must_use]
    pub const fn end(&self) -> usize {
        self.end
    }

    /// Creates a new span with the bounds, `[self.start(), other.end())`.
    #[must_use]
    pub const fn merge(&self, other: &Self) -> Self {
        Self::new(self.start(), other.end())
    }

    /// The start of the byte-range.
    #[must_use]
    pub const fn start(&self) -> usize {
        self.start
    }
}

impl From<Range<usize>> for Span {
    fn from(value: Range<usize>) -> Self {
        Self::new(value.start, value.end)
    }
}

impl From<Span> for Range<usize> {
    fn from(value: Span) -> Self {
        value.start()..value.end()
    }
}
