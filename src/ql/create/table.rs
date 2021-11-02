//! # Module `table`
//!
//! This module implements the `CREATE TABLE` query.

use crate::ColumnDataType;

/// # Struct `CreateBuilder`
///
/// A builder for a `Create`, constructs a `CREATE TABLE` query.
pub struct CreateTableBuilder {
    pub(crate) name: Option<String>,
    pub(crate) columns: Vec<(String, (ColumnDataType, bool))>
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

    /// # Instance Method `CreateTableBuilder::name`
    ///
    /// Sets the name of the object to create.
    ///
    /// ## Parameters
    /// - `fields`, type `impl Iterator<Item = (String, (ColumnDataType, bool))`; an iterator over
    ///             the fields, where:
    ///               - `TUPLE.0`, type `String`; the name of the column
    ///               - `TUPLE.1.0`, type `ColumnDataType`; the data type of the column
    ///               - `TUPLE.1.1`, type `bool`; whether the field value can be `NULL`.
    #[must_use]
    pub fn columns(
        mut self,
        fields: impl Iterator<Item = (String, (ColumnDataType, bool))>
    ) -> Self {
        self.columns = fields.collect();
        self
    }
}
