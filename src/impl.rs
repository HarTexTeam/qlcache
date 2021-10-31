//! # Module `impl`
//!
//! This module implements the querying from the cache.

use dashmap::DashMap;

use crate::{
    CacheSchema,
    QlCache
};

impl QlCache {
    /// # Constructor `QlCache::new`
    ///
    /// Creates a new `QlCache`.
    #[must_use]
    pub fn new() -> Self {
        let dashmap = DashMap::new();
        let default_schema = String::from("PUBLIC");

        dashmap.insert(default_schema.clone(), CacheSchema::new(default_schema));

        Self {
            cache: dashmap
        }
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
