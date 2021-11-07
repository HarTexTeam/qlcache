//! # Module `ql`
//!
//! This module implements the query language.

use std::marker::PhantomData;

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
    CacheTableRow,
    FromRow,
    QlCache
};

pub mod constraints;
pub mod create;
pub mod key;
pub mod select;
pub mod sortby;

/// # Trait `QueryRow`
///
/// The super trait for queries that returns rows.
pub trait QueryRow {
    /// # Trait Method `QueryRow::execute`
    ///
    /// Executes the query.
    ///
    /// ## Errors
    ///
    /// Returns query-related errors.
    fn execute(self, cache: &QlCache) -> QlResult<Vec<CacheTableRow>>;
}

/// # Trait `QueryAsType`
///
/// The super trait for queries that returns some specific structures that are
/// constructed from rows.
pub trait QueryAsType<'row, T: FromRow<'row>> {
    /// # Trait Method `QueryAsType::execute`
    ///
    /// Executes the query.
    ///
    /// ## Errors
    ///
    /// Returns query-related errors.
    fn execute_as(self, cache: &QlCache) -> QlResult<T>;
}

/// # Struct `Query`
///
/// A query.
pub struct Query<T: QueryRow> {
    pub(crate) query: T
}

impl<T: QueryRow> Query<T> {
    /// # Instance Method `Query::execute`
    ///
    /// Executes a query.
    ///
    /// ## Parameters
    /// - `cache`, type `&QlCache`; the cache to execute this query on
    ///
    /// ## Errors
    ///
    /// Returns query-related errors.
    pub fn execute(self, cache: &QlCache) -> QlResult<Vec<CacheTableRow>> {
        self.query.execute(cache)
    }
}

/// # Struct `QueryBuilder`
///
/// The builder for a `Query`.
pub struct QueryBuilder;

/// # Struct `Query`
///
/// A query where the results are mapped to a specific struct.
#[allow(dead_code)]
pub struct QueryAs<'row, T: QueryAsType<'row, U>, U: FromRow<'row>> {
    pub(crate) query: T,
    pub(crate) phantom: PhantomData<&'row U>
}

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

#[cfg(test)]
mod tests {
    use super::{
        Query,
        QueryBuilder,
        QueryRow
    };
    use crate::{
        error::QlResult,
        QlCache
    };

    struct Dummy;

    impl QueryRow for Dummy {
        type ResultType = ();

        fn execute(self, _: &QlCache) -> QlResult<Self::ResultType> {
            todo!()
        }
    }

    static_assertions::assert_impl_all!(Query<Dummy>: Send, Sync);
    static_assertions::assert_impl_all!(QueryBuilder: Send, Sync);
}
