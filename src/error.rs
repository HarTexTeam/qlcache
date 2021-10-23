//! # Module `error`
//!
//! This module implements various errors for the crate.

/// # Enumeration `QlError`
///
/// An enumeration representing various errors types.
pub enum QlError {
}

/// # Typealias `QlResult`
///
/// A typealias for `Result<T, QlError>`.
pub type QlResult<T> = Result<T, QlError>;
