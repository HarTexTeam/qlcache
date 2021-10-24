//! # Module `impl`
//!
//! This module implements the querying from the cache.

use sqlparser::{
    dialect::GenericDialect,
    parser::Parser
};

use crate::{
    error::QlResult,
    QlCache
};

impl QlCache {
    /// # Instance Method `QlCache::query`
    ///
    /// Executes a query.
    ///
    /// ## Parameters
    /// - `query`, type `&str`: the query
    pub fn query(&self, query: &str) -> QlResult<()> {
        let query_ast = Parser::parse_sql(&GenericDialect {}, query)?;
    }
}
