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
    ColumnDoesNotExist { name: String },
    NoFirstConstraintFoundBeforeAndOr,
    PrimaryKeyAlreadySet,
    QueryError(QueryError),
    RequiredFieldIsNone { field_name: String },
    VecCannotBeEmpty { vec_name: String }
}

/// # Enumeration `QueryError`
///
/// An enumeration representing various errors types regarding queries.
#[allow(clippy::module_name_repetitions)]
#[allow(missing_docs)] // variants are pretty self explanatory?
#[derive(Debug)]
pub enum QueryError {
    ObjectAlreadyExists { name: String },
    ObjectDoesNotExist { name: String }
}

/// # Typealias `QlResult`
///
/// A typealias for `Result<T, QlError>`.
pub type QlResult<T> = Result<T, QlError>;
