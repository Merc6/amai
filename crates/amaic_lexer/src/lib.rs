//! The crate responsible for converting source-text into more easily-
//! interpretable [`Tokens`](TokenKind).

mod error;
mod token_kind;

pub use error::LexError;
pub use token_kind::TokenKind;

/// The lexer for the Amai language.
pub type AmaicLexer<'src> = logos::Lexer<'src, TokenKind>;
