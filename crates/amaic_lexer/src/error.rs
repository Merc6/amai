//! See [`LexError`].

use crate::AmaicLexer;

use thiserror::Error;

/// An error that occurs while lexing.
#[derive(Clone, Copy, Debug, Default, Error, PartialEq, Eq, Hash)]
#[non_exhaustive] // non-exhaustive until language stabilizes
pub enum LexError {
    /// An error that occurs when something very unexpected happens.
    #[default]
    #[error("Un undefined error occurred")]
    Undefined,

    /// An error that occurs when an unexpected byte is parsed.
    #[error("An unexpected byte was found")]
    UnexpectedByte,

    /// An error that occurs when a string was not properly terminated.
    #[error("A string was found to not properly terminate")]
    UnterminatedString,
}

impl From<&mut AmaicLexer<'_>> for LexError {
    fn from(value: &mut AmaicLexer) -> Self {
        match *value.slice().as_bytes() {
            [b'"', ..] => Self::UnterminatedString,
            [_] => Self::UnexpectedByte,
            _ => Self::Undefined,
        }
    }
}
