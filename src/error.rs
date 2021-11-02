//! # Module `error`
//!
//! This module implements various errors for the crate.

/// # Enumeration `QlError`
///
/// An enumeration representing various errors types.
#[allow(clippy::module_name_repetitions)]
#[allow(missing_docs)] // variants are pretty self explanatory?
#[derive(Debug)]
pub enum QlError {
    NoFirstConstraintFoundBeforeAndOr,
    RequiredFieldIsNone { field_name: String },
    VecCannotBeEmpty
}

/// # Typealias `QlResult`
///
/// A typealias for `Result<T, QlError>`.
pub type QlResult<T> = Result<T, QlError>;
