//! # Module `key`
//!
//! This module implements `PRIMARY KEY` and `FOREIGN KEY`.

/// # Struct `PrimaryKey`
///
/// The primary key of a table.
pub struct PrimaryKey(pub(crate) String);

impl PrimaryKey {
    /// # Constructor `PrimaryKey::new`
    pub fn new(name: String) -> PrimaryKey {
        Self(name)
    }
}
