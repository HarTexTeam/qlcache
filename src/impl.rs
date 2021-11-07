//! # Module `impl`
//!
//! This module implements the querying from the cache.

use dashmap::DashMap;

use crate::{
    error::QlResult,
    ql::{
        Query,
        QueryAs,
        QueryAsType,
        QueryRow
    },
    CacheSchema,
    CacheTableRow,
    FromRow,
    QlCache
};

impl QlCache {
    /// # Constructor `QlCache::new`
    ///
    /// Creates a new `QlCache`.
    ///
    /// ## Example
    ///
    /// ```
    /// use qlcache::QlCache;
    ///
    /// let cache = QlCache::new();
    /// ```
    #[must_use]
    pub fn new() -> Self {
        let dashmap = DashMap::new();
        let default_schema = String::from("PUBLIC");

        dashmap.insert(default_schema.clone(), CacheSchema::new(default_schema));

        Self {
            cache: dashmap
        }
    }

    /// # Instance Method `QlCache::execute`
    ///
    /// Executes a query.
    ///
    /// ## Parameters
    /// - `query`, type `Query<T> where T: QueryKind`; the query to execute
    ///
    /// ## Errors
    ///
    /// Returns query-related errors.
    pub fn execute<T: QueryRow>(&self, query: Query<T>) -> QlResult<Vec<CacheTableRow>> {
        query.execute(self)
    }

    // TODO: implement this method
    /// # Instance Method `QlCache::execute_as`
    ///
    /// Executes a query, returns the value as specified by `U`.
    ///
    /// ## Parameters
    /// - `query`, type `QueryAs<T, U> where T: QueryKind, U: FromRow`; the query to execute
    ///
    /// ## Errors
    ///
    /// Returns query-related errors.
    pub fn execute_as<'row, T: QueryAsType<'row, U>, U: FromRow<'row>>(
        &self,
        _: QueryAs<'row, T, U>
    ) {
    }
}

impl CacheSchema {
    /// # Constructor `CacheSchema::new`
    ///
    /// Creates a new `CacheSchema`.
    #[must_use]
    pub fn new(name: String) -> Self {
        Self {
            name,
            tables: DashMap::new()
        }
    }
}

impl Default for QlCache {
    fn default() -> Self {
        Self::new()
    }
}
