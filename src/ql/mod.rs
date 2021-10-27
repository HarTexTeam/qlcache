//! # Module `ql`
//!
//! This module implements the query language.

use crate::ql::{
    constraints::ComputableConstraint,
    select::{
        Select,
        SelectBuilder,
    }
};

pub mod constraints;
pub mod select;

/// # Trait `QueryKind`
///
/// The super trait for various query kinds.
pub trait QueryKind {}

/// # Struct `Query`
///
/// A query.
pub struct Query<T: QueryKind> {
    pub(crate) query: T
}

/// # Struct `QueryBuilder`
///
/// The builder for a `Query`.
pub struct QueryBuilder<C>
where
    C: ComputableConstraint;

impl<C> QueryBuilder<C>
where
    C: ComputableConstraint {
    /// # Static Method `QueryBuilder::select`
    ///
    /// Returns a builder for building a `SELECT` query.
    #[must_use]
    pub fn select() -> SelectBuilder<C> {
        Select::builder()
    }
}
