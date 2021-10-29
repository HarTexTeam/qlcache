//! # Module `ql`
//!
//! This module implements the query language.

use crate::ql::select::{
    Select,
    SelectBuilder
};

pub mod constraints;
pub mod select;
pub mod sortby;

/// # Trait `QueryKind`
///
/// The super trait for various query kinds.
pub trait QueryKind {}

/// # Struct `Query`
///
/// A query.
#[allow(dead_code)]
pub struct Query<T: QueryKind> {
    pub(crate) query: T
}

/// # Struct `QueryBuilder`
///
/// The builder for a `Query`.
pub struct QueryBuilder;

impl QueryBuilder {
    /// # Static Method `QueryBuilder::select`
    ///
    /// Returns a builder for building a `SELECT` query.
    #[must_use]
    pub fn select() -> SelectBuilder {
        Select::builder()
    }
}
