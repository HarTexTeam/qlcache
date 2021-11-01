//! # Module `table`
//!
//! This module implements the `CREATE TABLE` query.

/// # Struct `CreateBuilder`
///
/// A builder for a `Create`, constructs a `CREATE TABLE` query.
pub struct CreateTableBuilder {
    pub(crate) name: Option<String>
}

impl CreateTableBuilder {
    /// # Instance Method `CreateTableBuilder::name`
    ///
    /// Sets the name of the object to create.
    ///
    /// ## Parameters
    /// - `name`, type `String`; the table name
    #[must_use]
    pub fn name(mut self, name: String) -> Self {
        self.name.replace(name);
        self
    }
}
