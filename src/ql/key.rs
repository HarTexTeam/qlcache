//! # Module `key`
//!
//! This module implements `PRIMARY KEY` and `FOREIGN KEY`.

/// # Struct `PrimaryKey`
///
/// The primary key of a table.
#[allow(clippy::module_name_repetitions)]
pub struct PrimaryKey(pub(crate) String);

impl PrimaryKey {
    /// # Constructor `PrimaryKey::new`
    #[must_use]
    pub fn new(name: String) -> PrimaryKey {
        Self(name)
    }
}
