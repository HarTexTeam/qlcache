//! # Module `error`
//!
//! This module implements various errors for the crate.

/// # Enumeration `QlError`
///
/// An enumeration representing various errors types.
pub enum QlError {
    ParseQueryError(ParseQueryErrorKind)
}

/// # Enumeration `ParseQueryErrorKind`
///
/// An enumeration represents various errors that may occur during query parsing.
#[derive(Clone, PartialEq)]
pub enum ParseQueryErrorKind {
    SyntaxError {
        description: String
    }
}

pub type QlResult<T> = Result<T, QlError>;
