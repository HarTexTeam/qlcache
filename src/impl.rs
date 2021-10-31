//! # Module `impl`
//!
//! This module implements the querying from the cache.

use dashmap::DashMap;

use crate::QlCache;

impl QlCache {
    /// # Constructor `QlCache::new`
    ///
    /// Creates a new `QlCache`.
    #[must_use]
    pub fn new() -> Self {
        Self {
            cache: DashMap::new()
        }
    }
}

impl Default for QlCache {
    fn default() -> Self {
        Self::new()
    }
}
