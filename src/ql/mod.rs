//! # Module `ql`
//!
//! This module implements the query language.

use crate::{
    error::QlResult,
    ql::{
        create::{
            Create,
            CreateBuilder
        },
        select::{
            Select,
            SelectBuilder
        }
    },
    QlCache
};

pub mod constraints;
pub mod create;
pub mod key;
pub mod select;
pub mod sortby;

/// # Trait `QueryKind`
///
/// The super trait for various query kinds.
pub trait QueryKind {
    /// # Trait Associated Type `ResultType`
    ///
    /// The type of the returned result from this query.
    type ResultType;

    /// # Trait Method `QueryKind::execute`
    ///
    /// Executes the query.
    ///
    /// ## Errors
    ///
    /// Returns query-related errors.
    fn execute(self, cache: &QlCache) -> QlResult<Self::ResultType>;
}

/// # Struct `Query`
///
/// A query.
#[allow(dead_code)]
pub struct Query<T: QueryKind> {
    pub(crate) query: T
}

impl<T: QueryKind> Query<T> {
    /// # Instance Method `Query::execute`
    ///
    /// Executes a query.
    ///
    /// ## Parameters
    /// - `cache`, type `&QlCache`; the cache to execute this query on
    pub fn execute(self, cache: &QlCache) -> QlResult<T::ResultType> {
        self.query.execute(cache)
    }
}

/// # Struct `QueryBuilder`
///
/// The builder for a `Query`.
pub struct QueryBuilder;

impl QueryBuilder {
    /// # Static Method `QueryBuilder::create`
    ///
    /// Returns a builder for building a `CREATE` query.
    #[must_use]
    pub fn create() -> CreateBuilder {
        Create::builder()
    }

    /// # Static Method `QueryBuilder::select`
    ///
    /// Returns a builder for building a `SELECT` query.
    #[must_use]
    pub fn select() -> SelectBuilder {
        Select::builder()
    }
}
