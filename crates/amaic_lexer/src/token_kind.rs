//! See [`TokenKind`].

use crate::error::LexError;

use logos::Logos;
use std::fmt::{self, Display, Formatter};

/// Tokens that are produced by the [`Lexer`](crate::AmaicLexer).
#[derive(Clone, Copy, Debug, Eq, Hash, Logos, PartialEq)]
#[logos(skip r"[ \t\r\n]+")]
#[logos(error(LexError, LexError::from))]
#[repr(u8)]
#[non_exhaustive] // non-exhaustive until language stabilizes
pub enum TokenKind {
    /// Keyword: `const`.
    #[token("const")]
    Const,
    /// Keyword: `var`.
    #[token("var")]
    Var,
    /// Keyword: `if`.
    #[token("if")]
    If,
    /// Keyword: `else`.
    #[token("else")]
    Else,
    /// Keyword: `while`.
    #[token("while")]
    While,
    /// Keyword: `for`.
    #[token("for")]
    For,
    /// Keyword: `in`.
    #[token("in")]
    In,
    /// Keyword: `return`.
    #[token("return")]
    Return,
    /// Keyword: `true`.
    #[token("true")]
    True,
    /// Keyword: `false`.
    #[token("false")]
    False,
    /// Keyword: `import`.
    #[token("import")]
    Import,
    /// Keyword: `func`.
    #[token("func")]
    Func,
    /// Keyword: `self`.
    #[token("self")]
    SSelf,
    /// Keyword: `and`.
    #[token("and")]
    And,
    /// Keyword: `or`.
    #[token("or")]
    Or,
    /// Keyword: `match`.
    #[token("match")]
    Match,

    /// Delimiters: `(`.
    #[token("(")]
    OpenParen,
    /// Delimiters: `)`.
    #[token(")")]
    ClosedParen,
    /// Delimiters: `[`.
    #[token("[")]
    OpenBrack,
    /// Delimiters: `]`.
    #[token("]")]
    ClosedBrack,
    /// Delimiters: `{`.
    #[token("{")]
    OpenBrace,
    /// Delimiters: `}`.
    #[token("}")]
    ClosedBrace,
    /// Delimiters: `<`.
    #[token("<")]
    OpenAngle,
    /// Delimiters: `>`.
    #[token(">")]
    ClosedAngle,

    /// Symbol: `:`.
    #[token(":")]
    Colon,
    /// Symbol: `,`.
    #[token(",")]
    Comma,
    /// Symbol: `.`.
    #[token(".")]
    Dot,
    /// Symbol: `?`.
    #[token("?")]
    Question,
    /// Symbol: `!`.
    #[token("!")]
    Bang,
    /// Symbol: `%`.
    #[token("%")]
    Percent,
    /// Symbol: `^`.
    #[token("^")]
    Caret,
    /// Symbol: `=`.
    #[token("=")]
    Equal,
    /// Symbol: `&`.
    #[token("&")]
    Ampersand,
    /// Symbol: `|`.
    #[token("|")]
    Pipe,
    /// Symbol: `+`.
    #[token("+")]
    Plus,
    /// Symbol: `-`.
    #[token("-")]
    Minus,
    /// Symbol: `*`.
    #[token("*")]
    Star,
    /// Symbol: `/`.
    #[token("/")]
    Slash,

    /// Combination-Symbol: `+=`.
    #[token("+=")]
    PlusEqual,
    /// Combination-Symbol: `-=`.
    #[token("-=")]
    MinusEqual,
    /// Combination-Symbol: `*=`.
    #[token("*=")]
    StarEqual,
    /// Combination-Symbol: `/=`.
    #[token("/=")]
    SlashEqual,
    /// Combination-Symbol: `%=`.
    #[token("%=")]
    PercentEqual,
    /// Combination-Symbol: `==`.
    #[token("==")]
    EqualEqual,
    /// Combination-Symbol: `!=`.
    #[token("!=")]
    BangEqual,
    /// Combination-Symbol: `>=`.
    #[token(">=")]
    ClosedAngleEqual,
    /// Combination-Symbol: `<=`.
    #[token("<=")]
    OpenAngleEqual,
    /// Combination-Symbol: `->`.
    #[token("->")]
    DashClosedAngle,
    /// Combination-Symbol: `=>`.
    #[token("=>")]
    EqualClosedAngle,
    /// Combination-Symbol: `..`.
    #[token("..")]
    DotDot,
    /// Combination-Symbol: `..=`.
    #[token("..=")]
    DotDotEqual,
    /// Combination-Symbol: `::`.
    #[token("::")]
    ColonColon,

    /// Special: Identifies a given value.
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,
    /// Special: What you're reading right now.
    #[regex("//[^\r\n]*", allow_greedy = true)]
    Comment,

    /// Literal: Integer-values; Comes in all the lovely bases of.
    /// - 16
    /// - 10
    /// - 8
    /// - 2
    #[regex(r"-?[0-9][0-9_]*")]
    #[regex(r"-?0[xX]_*[0-9a-fA-F]*")]
    #[regex(r"-?0[oO]_*[0-8_]*")]
    #[regex(r"-?0[bB]_*[01]*")]
    Int,
    /// Literal: floating-point number.
    #[regex(r"-?[0-9][0-9_]*\._*[0-9_]*")]
    Float,
    /// Literal: A collection of characters.
    #[regex(r#""([^\r\n]|\\.)*""#)]
    String,
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let as_str = match *self {
            Self::Ampersand => "&",
            Self::And => "and",
            Self::Bang => "!",
            Self::BangEqual => "!=",
            Self::Caret => "^",
            Self::ClosedAngle => ">",
            Self::ClosedAngleEqual => ">=",
            Self::ClosedBrace => "}",
            Self::ClosedBrack => "]",
            Self::ClosedParen => ")",
            Self::Colon => ":",
            Self::ColonColon => "::",
            Self::Comma => ",",
            Self::Comment => "<comment>",
            Self::Const => "const",
            Self::DashClosedAngle => "->",
            Self::Dot => ".",
            Self::DotDot => "..",
            Self::DotDotEqual => "..=",
            Self::Else => "else",
            Self::Equal => "=",
            Self::EqualClosedAngle => "=>",
            Self::EqualEqual => "==",
            Self::False => "false",
            Self::Float => "<float literal>",
            Self::For => "for",
            Self::Func => "func",
            Self::Identifier => "<identifier>",
            Self::If => "if",
            Self::Import => "import",
            Self::In => "in",
            Self::Int => "<integer literal>",
            Self::Match => "match",
            Self::Minus => "-",
            Self::MinusEqual => "-=",
            Self::OpenAngle => "<",
            Self::OpenAngleEqual => "<=",
            Self::OpenBrace => "{",
            Self::OpenBrack => "[",
            Self::OpenParen => "(",
            Self::Or => "or",
            Self::Percent => "%",
            Self::PercentEqual => "%=",
            Self::Pipe => "|",
            Self::Plus => "+",
            Self::PlusEqual => "+=",
            Self::Question => "?",
            Self::Return => "return",
            Self::SSelf => "self",
            Self::Slash => "/",
            Self::SlashEqual => "/=",
            Self::Star => "*",
            Self::StarEqual => "*=",
            Self::String => "<string literal>",
            Self::True => "true",
            Self::Var => "var",
            Self::While => "while",
        };

        write!(f, "{as_str}")
    }
}
