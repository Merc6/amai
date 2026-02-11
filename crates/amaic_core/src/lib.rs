//! Types that are common-enough that they need their own crate, but are small-
//! enough to be bundled into one.
//!
//! # Note
//!
//! This API is completely-unstable and subject to change.

mod diagnostic;
mod span;

pub use diagnostic::*;
pub use span::Span;
