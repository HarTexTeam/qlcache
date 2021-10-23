//! # Module `lexeme`
//!
//! This module implements the lexemes for the query language.

#[derive(Clone, PartialEq)]
pub(crate) struct Lexeme {
    pub(crate) len: usize,
    pub(crate) kind: LexemeKind
}

#[derive(Clone, PartialEq)]
pub(crate) enum LexemeKind {
}
