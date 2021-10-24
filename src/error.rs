//! # Module `error`
//!
//! This module implements various errors for the crate.

use sqlparser::parser::ParserError;

/// # Enumeration `QlError`
///
/// An enumeration representing various errors types.
pub enum QlError {
    SqlParseError(ParserError)
}

impl From<ParserError> for QlError {
    fn from(error: ParserError) -> Self {
        Self::SqlParseError(error)
    }
}

/// # Typealias `QlResult`
///
/// A typealias for `Result<T, QlError>`.
pub type QlResult<T> = Result<T, QlError>;
